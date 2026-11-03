// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Scratch phase breakdown of the hash join build (not for commit).

use datafusion_common::instant::Instant;
use std::hint::black_box;
use std::sync::Arc;

use arrow::array::{ArrayRef, Int64Array, RecordBatch};
use arrow::compute::concat_batches;
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_common::NullEquality;
use datafusion_common::hash_utils::{RandomState, create_hashes};
use datafusion_physical_expr::PhysicalExprRef;
use datafusion_physical_expr::expressions::col;
use datafusion_physical_expr_common::utils::evaluate_expressions_to_arrays;
use datafusion_physical_plan::joins::join_hash_map::JoinHashMapU32;
use datafusion_physical_plan::joins::utils::update_hash;
use hashbrown::HashTable;

/// With `HUGE=1`, asks for transparent huge pages on every allocation of at
/// least 4 MiB, as if `/sys/kernel/mm/transparent_hugepage/enabled` were
/// `always` for them.
mod huge {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicU8, Ordering};

    unsafe extern "C" {
        fn madvise(addr: *mut u8, len: usize, advice: i32) -> i32;
    }
    const MADV_HUGEPAGE: i32 = 14;
    const HUGE_PAGE: usize = 2 << 20;
    static ENABLED: AtomicU8 = AtomicU8::new(2);

    fn enabled() -> bool {
        match ENABLED.load(Ordering::Relaxed) {
            2 => {
                let on = std::env::var_os("HUGE").is_some_and(|v| v == "1");
                ENABLED.store(on as u8, Ordering::Relaxed);
                on
            }
            v => v == 1,
        }
    }

    unsafe fn advise(ptr: *mut u8, size: usize) {
        if ptr.is_null() || size < (4 << 20) || !enabled() {
            return;
        }
        let start = (ptr as usize).next_multiple_of(HUGE_PAGE);
        let end = (ptr as usize + size) / HUGE_PAGE * HUGE_PAGE;
        if end > start {
            unsafe { madvise(start as *mut u8, end - start, MADV_HUGEPAGE) };
        }
    }

    pub struct Huge;
    unsafe impl GlobalAlloc for Huge {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let p = unsafe { System.alloc(layout) };
            unsafe { advise(p, layout.size()) };
            p
        }
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            let p = unsafe { System.alloc_zeroed(layout) };
            unsafe { advise(p, layout.size()) };
            p
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            unsafe { System.dealloc(ptr, layout) }
        }
        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new: usize) -> *mut u8 {
            let p = unsafe { System.realloc(ptr, layout, new) };
            unsafe { advise(p, new) };
            p
        }
    }
}

/// An allocator that records where the table was allocated.
mod rec {
    use allocator_api2::alloc::{AllocError, Allocator, Global, Layout};
    use std::ptr::NonNull;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Clone, Default)]
    pub struct Rec(Arc<AtomicUsize>);

    impl Rec {
        pub fn base(&self) -> usize {
            self.0.load(Ordering::Relaxed)
        }
    }

    unsafe impl Allocator for Rec {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            let p = Global.allocate(layout)?;
            self.0
                .store(p.as_ptr().cast::<u8>() as usize, Ordering::Relaxed);
            Ok(p)
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            unsafe { Global.deallocate(ptr, layout) }
        }
    }

    #[inline(always)]
    pub fn prefetch(addr: usize) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use std::arch::x86_64::{_MM_HINT_T0, _mm_prefetch};
            _mm_prefetch(addr as *const i8, _MM_HINT_T0);
        }
    }
}

/// Candidate replacement for `HashTable<(u64, u32)>`: open addressing with
/// linear probing over one array of 8-byte slots, `tag << 32 | (row + 1)`,
/// where `tag` is the upper 32 bits of the hash and the home slot comes from
/// the lower bits. `0` is an empty slot.
mod own {
    pub struct Table {
        slots: Vec<u64>,
        mask: usize,
        len: usize,
    }

    #[inline(always)]
    fn tag(h: u64) -> u64 {
        h >> 32
    }

    impl Table {
        pub fn with_capacity(n: usize) -> Self {
            let cap = (n.max(1) * 2).next_power_of_two();
            Self {
                slots: vec![0; cap],
                mask: cap - 1,
                len: 0,
            }
        }

        #[inline(always)]
        pub fn prefetch(&self, h: u64) {
            super::rec::prefetch(
                self.slots.as_ptr() as usize + ((h as usize) & self.mask) * 8,
            );
        }

        /// Inserts `row` as the new head of `h`'s rows, returns the old head (+1) or 0.
        #[inline(always)]
        pub fn insert(&mut self, h: u64, row: u32) -> u32 {
            let t = tag(h);
            let mut p = (h as usize) & self.mask;
            loop {
                // SAFETY: p <= mask < slots.len()
                let s = unsafe { self.slots.get_unchecked_mut(p) };
                if *s == 0 {
                    *s = (t << 32) | (row as u64 + 1);
                    self.len += 1;
                    return 0;
                }
                if *s >> 32 == t {
                    let prev = *s as u32;
                    *s = (t << 32) | (row as u64 + 1);
                    return prev;
                }
                p = (p + 1) & self.mask;
            }
        }

        /// The head (+1) of `h`'s rows, or 0.
        #[inline(always)]
        pub fn find(&self, h: u64) -> u32 {
            let t = tag(h);
            let mut p = (h as usize) & self.mask;
            loop {
                let s = unsafe { *self.slots.get_unchecked(p) };
                if s == 0 {
                    return 0;
                }
                if s >> 32 == t {
                    return s as u32;
                }
                p = (p + 1) & self.mask;
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }
    }
}

/// Like `own`, but slots come in cache-line buckets of 8: a lookup compares
/// all 8 tags of its home bucket at once and stops at the first bucket with
/// an empty slot, so a miss is usually one cache line and no per-slot branch.
mod bucketed {
    const W: usize = 8;

    #[repr(C, align(64))]
    #[derive(Clone, Copy)]
    struct Bucket([u64; W]);

    pub struct Table {
        buckets: Vec<Bucket>,
        mask: usize,
        len: usize,
    }

    #[inline(always)]
    fn tag(h: u64) -> u64 {
        h >> 32
    }

    /// Bit `i` set if slot `i` holds tag `t`, and bit `8 + i` if it is empty.
    #[inline(always)]
    fn scan(b: &Bucket, t: u64) -> (u32, u32) {
        let mut hit = 0u32;
        let mut empty = 0u32;
        for i in 0..W {
            hit |= ((b.0[i] >> 32 == t && b.0[i] != 0) as u32) << i;
            empty |= ((b.0[i] == 0) as u32) << i;
        }
        (hit, empty)
    }

    impl Table {
        pub fn with_capacity(n: usize) -> Self {
            // At most half full, as `own`.
            let buckets = ((n.max(1) * 2).div_ceil(W)).next_power_of_two();
            Self {
                buckets: vec![Bucket([0; W]); buckets],
                mask: buckets - 1,
                len: 0,
            }
        }

        #[inline(always)]
        pub fn prefetch(&self, h: u64) {
            super::rec::prefetch(
                self.buckets.as_ptr() as usize + ((h as usize) & self.mask) * 64,
            );
        }

        #[inline(always)]
        pub fn insert(&mut self, h: u64, row: u32) -> u32 {
            let t = tag(h);
            let new = (t << 32) | (row as u64 + 1);
            let mut b = (h as usize) & self.mask;
            loop {
                let bucket = unsafe { self.buckets.get_unchecked_mut(b) };
                let (hit, empty) = scan(bucket, t);
                if hit != 0 {
                    let s = &mut bucket.0[hit.trailing_zeros() as usize];
                    let prev = *s as u32;
                    *s = new;
                    return prev;
                }
                if empty != 0 {
                    bucket.0[empty.trailing_zeros() as usize] = new;
                    self.len += 1;
                    return 0;
                }
                b = (b + 1) & self.mask;
            }
        }

        #[inline(always)]
        pub fn find(&self, h: u64) -> u32 {
            let t = tag(h);
            let mut b = (h as usize) & self.mask;
            loop {
                let bucket = unsafe { self.buckets.get_unchecked(b) };
                let (hit, empty) = scan(bucket, t);
                if hit != 0 {
                    return bucket.0[hit.trailing_zeros() as usize] as u32;
                }
                if empty != 0 {
                    return 0;
                }
                b = (b + 1) & self.mask;
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }
    }
}

#[global_allocator]
static GLOBAL: huge::Huge = huge::Huge;

const BATCH: usize = 8192;
const SEED: u64 = 12210250226015887276;

fn scatter(i: usize) -> i64 {
    (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) as i64
}

fn best<F: FnMut()>(mut f: F) -> f64 {
    f();
    let mut best = f64::MAX;
    for _ in 0..7 {
        let t = Instant::now();
        f();
        best = best.min(t.elapsed().as_secs_f64());
    }
    best
}

fn main() {
    let rs = RandomState::with_seed(SEED);
    for &(n, dup) in &[
        (100_000usize, 1usize),
        (1_000_000, 1),
        (4_000_000, 1),
        (4_000_000, 4),
    ] {
        let schema = Arc::new(Schema::new(vec![
            Field::new("k0", DataType::Int64, false),
            Field::new("payload", DataType::Int64, false),
        ]));
        let batches: Vec<RecordBatch> = (0..n)
            .step_by(BATCH)
            .map(|s| {
                let r = s..(s + BATCH).min(n);
                let cols: Vec<ArrayRef> = vec![
                    Arc::new(Int64Array::from_iter_values(
                        r.clone().map(|i| scatter(i / dup)),
                    )),
                    Arc::new(Int64Array::from_iter_values(r.map(|i| i as i64))),
                ];
                RecordBatch::try_new(Arc::clone(&schema), cols).unwrap()
            })
            .collect();
        let on: Vec<PhysicalExprRef> = vec![col("k0", &schema).unwrap()];
        let ns = |s: f64| s * 1e9 / n as f64;

        // Phase: evaluate keys + hash
        let t_hash = best(|| {
            let mut buf = vec![0u64; BATCH];
            for b in batches.iter().rev() {
                let k = evaluate_expressions_to_arrays(&on, b).unwrap();
                buf.resize(b.num_rows(), 0);
                create_hashes(&k, &rs, &mut buf).unwrap();
                black_box(&buf);
            }
        });

        // Phase: hashing + insertion through the real code path
        let t_update = best(|| {
            let mut map = JoinHashMapU32::with_capacity(n);
            let mut buf = vec![0u64; BATCH];
            let mut offset = n;
            for b in batches.iter().rev() {
                offset -= b.num_rows();
                buf.clear();
                buf.resize(b.num_rows(), 0);
                update_hash(
                    &on,
                    b,
                    &mut map,
                    offset,
                    &rs,
                    &mut buf,
                    0,
                    true,
                    NullEquality::NullEqualsNothing,
                )
                .unwrap();
            }
            black_box(map);
        });

        // Allocation of the table only (with_capacity + zeroed next)
        let t_alloc = best(|| {
            black_box(JoinHashMapU32::with_capacity(n));
        });

        let t_concat = best(|| {
            black_box(concat_batches(&schema, batches.iter()).unwrap());
        });

        // Precompute all hashes, so insert variants are timed on their own.
        let mut all_hashes = vec![0u64; n];
        let mut off = 0;
        for b in &batches {
            let k = evaluate_expressions_to_arrays(&on, b).unwrap();
            create_hashes(&k, &rs, &mut all_hashes[off..off + b.num_rows()]).unwrap();
            off += b.num_rows();
        }

        // Variant A: monomorphic insert loop, same algorithm as update_from_iter.
        let t_mono = best(|| {
            let mut map: HashTable<(u64, u32)> = HashTable::with_capacity(n);
            let mut next = vec![0u32; n];
            for row in (0..n).rev() {
                let h = all_hashes[row];
                match map.entry(h, |&(x, _)| x == h, |&(x, _)| x) {
                    hashbrown::hash_table::Entry::Occupied(mut o) => {
                        let prev = o.get().1;
                        o.get_mut().1 = row as u32 + 1;
                        next[row] = prev;
                    }
                    hashbrown::hash_table::Entry::Vacant(v) => {
                        v.insert((h, row as u32 + 1));
                    }
                }
            }
            black_box((map, next));
        });

        // Variant D: one counting-sort pass that groups rows by the top bits of
        // their hashbrown home bucket (stable, so chains keep their order),
        // then the same hashbrown insert as Variant A, now walking the table
        // region by region instead of at random.
        let partition_bits: u32 = std::env::var("PBITS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);
        let t_sorted = best(|| {
            let mut map: HashTable<(u64, u32)> = HashTable::with_capacity(n);
            let mut next = vec![0u32; n];
            let buckets = (map.capacity() + 1).next_power_of_two();
            let bucket_bits = buckets.trailing_zeros();
            let pbits = partition_bits.min(bucket_bits);
            let shift = bucket_bits - pbits;
            let mask = (buckets - 1) as u64;
            let part = |h: u64| ((h & mask) >> shift) as usize;
            let parts = 1usize << pbits;
            let mut counts = vec![0usize; parts + 1];
            for &h in &all_hashes {
                counts[part(h) + 1] += 1;
            }
            for i in 1..=parts {
                counts[i] += counts[i - 1];
            }
            let mut sorted: Vec<(u64, u32)> = Vec::with_capacity(n);
            let dst = sorted.spare_capacity_mut();
            // Descending row order, as the build inserts today.
            for row in (0..n).rev() {
                let h = all_hashes[row];
                let c = &mut counts[part(h)];
                dst[*c].write((h, row as u32));
                *c += 1;
            }
            unsafe { sorted.set_len(n) };
            for &(h, row) in &sorted {
                match map.entry(h, |&(x, _)| x == h, |&(x, _)| x) {
                    hashbrown::hash_table::Entry::Occupied(mut o) => {
                        next[row as usize] =
                            std::mem::replace(&mut o.get_mut().1, row + 1);
                    }
                    hashbrown::hash_table::Entry::Vacant(v) => {
                        v.insert((h, row + 1));
                    }
                }
            }
            black_box((map, next));
        });

        // Variant F: hashbrown, unchanged, with a recording allocator so the
        // home slot of a row AHEAD rows later can be prefetched. Assumes
        // hashbrown's layout: `buckets` 16-byte slots stored backwards in front
        // of the control bytes. A wrong guess costs speed, never correctness.
        let prefetch_ahead: usize = std::env::var("AHEAD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(16);
        let mut hit_rate = 0.0;
        let t_hb_prefetch = best(|| {
            let rec = rec::Rec::default();
            let mut map: HashTable<(u64, u32), rec::Rec> =
                HashTable::with_capacity_in(n, rec.clone());
            let mut next = vec![0u32; n];
            let buckets = (map.capacity() + 1).next_power_of_two();
            let mask = buckets - 1;
            let ctrl = rec.base() + buckets * 16;
            for row in (0..n).rev() {
                if row >= prefetch_ahead {
                    let pos = (all_hashes[row - prefetch_ahead] as usize) & mask;
                    rec::prefetch(ctrl + pos);
                    rec::prefetch(ctrl - (pos + 1) * 16);
                }
                let h = all_hashes[row];
                match map.entry(h, |&(x, _)| x == h, |&(x, _)| x) {
                    hashbrown::hash_table::Entry::Occupied(mut o) => {
                        next[row] = std::mem::replace(&mut o.get_mut().1, row as u32 + 1);
                    }
                    hashbrown::hash_table::Entry::Vacant(v) => {
                        v.insert((h, row as u32 + 1));
                    }
                }
            }
            // How often the guessed slot is where the row actually landed
            // (only the first SAMPLE rows; groups of 16 make exact hits rarer).
            const SAMPLE: usize = 4096;
            let mut hits = 0;
            for &h in all_hashes.iter().take(SAMPLE) {
                let found =
                    std::ptr::from_ref(map.find(h, |&(x, _)| x == h).unwrap()) as usize;
                let pos = (h as usize) & mask;
                let guess = ctrl - (pos + 1) * 16;
                // Same 64-byte cache line or the next three, as a group probe touches.
                if found <= guess && guess - found < 256 {
                    hits += 1;
                }
            }
            hit_rate = hits as f64 / SAMPLE.min(n) as f64;
            black_box((map, next));
        });

        // Variant B: open addressing table we own: power-of-two, linear probing,
        // 8-byte slots (u32 hash tag | u32 row+1), with software prefetch ahead.
        let t_own = best(|| {
            let cap = (n * 2).next_power_of_two();
            let mask = cap - 1;
            let mut slots = vec![0u64; cap];
            let mut next = vec![0u32; n];
            const AHEAD: usize = 16;
            for row in (0..n).rev() {
                if row >= AHEAD {
                    let p = (all_hashes[row - AHEAD] as usize) & mask;
                    #[cfg(target_arch = "x86_64")]
                    unsafe {
                        use std::arch::x86_64::{_MM_HINT_T0, _mm_prefetch};
                        _mm_prefetch(slots.as_ptr().add(p).cast::<i8>(), _MM_HINT_T0);
                    }
                }
                let h = all_hashes[row];
                let tag = (h >> 32) as u32;
                let mut p = (h as usize) & mask;
                loop {
                    let s = slots[p];
                    if s == 0 {
                        slots[p] = ((tag as u64) << 32) | (row as u64 + 1);
                        break;
                    }
                    if (s >> 32) as u32 == tag {
                        // Same tag: treat as same hash bucket and chain.
                        next[row] = s as u32;
                        slots[p] = ((tag as u64) << 32) | (row as u64 + 1);
                        break;
                    }
                    p = (p + 1) & mask;
                }
            }
            black_box((slots, next));
        });

        // Variant C: unchained build (count, prefix sum, scatter) on top hash bits.
        let t_unchained = best(|| {
            let bits = (n.next_power_of_two().trailing_zeros()).max(1);
            let buckets = 1usize << bits;
            let shift = 64 - bits;
            let mut dir = vec![0u32; buckets + 1];
            for &h in &all_hashes {
                dir[(h >> shift) as usize + 1] += 1;
            }
            for i in 1..=buckets {
                dir[i] += dir[i - 1];
            }
            let mut cursor: Vec<u32> = dir[..buckets].to_vec();
            let mut rows = vec![0u32; n];
            for (row, &h) in all_hashes.iter().enumerate() {
                let b = (h >> shift) as usize;
                let c = &mut cursor[b];
                rows[*c as usize] = row as u32;
                *c += 1;
            }
            black_box((dir, rows));
        });

        // Own table as a struct: build, then probe with 50% hits, against
        // hashbrown built and probed the same way.
        let probe: Vec<u64> = {
            let mut s = 0x2545_F491_4F6C_DD1Du64;
            (0..n)
                .map(|i| {
                    s ^= s << 13;
                    s ^= s >> 7;
                    s ^= s << 17;
                    if i % 2 == 0 {
                        all_hashes[(s as usize) % n]
                    } else {
                        s
                    }
                })
                .collect()
        };
        let mut own_table = None;
        let t_own_struct = best(|| {
            let mut t = own::Table::with_capacity(n);
            let mut next = vec![0u32; n];
            for row in (0..n).rev() {
                if row >= prefetch_ahead {
                    t.prefetch(all_hashes[row - prefetch_ahead]);
                }
                next[row] = t.insert(all_hashes[row], row as u32);
            }
            own_table = Some(black_box((t, next)));
        });
        let (own_t, _) = own_table.unwrap();
        let mut hb: HashTable<(u64, u32)> = HashTable::with_capacity(n);
        for row in (0..n).rev() {
            let h = all_hashes[row];
            match hb.entry(h, |&(x, _)| x == h, |&(x, _)| x) {
                hashbrown::hash_table::Entry::Occupied(mut o) => {
                    o.get_mut().1 = row as u32 + 1
                }
                hashbrown::hash_table::Entry::Vacant(v) => {
                    v.insert((h, row as u32 + 1));
                }
            }
        }
        assert_eq!(hb.len(), own_t.len());
        let mut found_hb = 0usize;
        let t_probe_hb = best(|| {
            found_hb = probe
                .iter()
                .filter(|&&h| hb.find(h, |&(x, _)| x == h).is_some())
                .count();
        });
        let mut found_own = 0usize;
        let t_probe_own = best(|| {
            let mut c = 0;
            for (i, &h) in probe.iter().enumerate() {
                if let Some(&ah) = probe.get(i + prefetch_ahead) {
                    own_t.prefetch(ah);
                }
                c += (own_t.find(h) != 0) as usize;
            }
            found_own = c;
        });
        let t_probe_own_nopf = best(|| {
            black_box(probe.iter().filter(|&&h| own_t.find(h) != 0).count());
        });
        assert_eq!(
            found_hb, found_own,
            "own table must find the same probe rows"
        );

        let mut bucketed_table = None;
        let t_bk_build = best(|| {
            let mut t = bucketed::Table::with_capacity(n);
            let mut next = vec![0u32; n];
            for row in (0..n).rev() {
                if row >= prefetch_ahead {
                    t.prefetch(all_hashes[row - prefetch_ahead]);
                }
                next[row] = t.insert(all_hashes[row], row as u32);
            }
            bucketed_table = Some(black_box((t, next)));
        });
        let (bk, _) = bucketed_table.unwrap();
        assert_eq!(bk.len(), own_t.len());
        let mut found_bk = 0;
        let t_bk_probe = best(|| {
            let mut c = 0;
            for (i, &h) in probe.iter().enumerate() {
                if let Some(&ah) = probe.get(i + prefetch_ahead) {
                    bk.prefetch(ah);
                }
                c += (bk.find(h) != 0) as usize;
            }
            found_bk = c;
        });
        let t_bk_probe_nopf = best(|| {
            black_box(probe.iter().filter(|&&h| bk.find(h) != 0).count());
        });
        assert_eq!(
            found_hb, found_bk,
            "bucketed table must find the same probe rows"
        );

        println!("rows={n} dup={dup}   (ns/row)");
        println!("  build: own struct+pf    {:6.2}", ns(t_own_struct));
        println!("  probe: hashbrown        {:6.2}", ns(t_probe_hb));
        println!("  probe: own, prefetch    {:6.2}", ns(t_probe_own));
        println!("  probe: own, no prefetch {:6.2}", ns(t_probe_own_nopf));
        println!("  build: bucketed+pf      {:6.2}", ns(t_bk_build));
        println!("  probe: bucketed, pf     {:6.2}", ns(t_bk_probe));
        println!("  probe: bucketed, no pf  {:6.2}", ns(t_bk_probe_nopf));
        println!("  alloc table             {:6.2}", ns(t_alloc));
        println!("  eval keys + hash        {:6.2}", ns(t_hash));
        println!(
            "  update_hash (hash+ins)  {:6.2}   => insert ~{:.2}",
            ns(t_update),
            ns(t_update - t_hash)
        );
        println!("  concat_batches          {:6.2}", ns(t_concat));
        println!("  insert: mono hashbrown  {:6.2}", ns(t_mono));
        println!(
            "  insert: partitioned hb  {:6.2}   (PBITS={partition_bits})",
            ns(t_sorted)
        );
        println!(
            "  insert: hb + prefetch   {:6.2}   (AHEAD={prefetch_ahead}, slot guess hit {:.1}%)",
            ns(t_hb_prefetch),
            hit_rate * 100.0
        );
        println!("  insert: own+prefetch    {:6.2}", ns(t_own));
        println!("  insert: unchained       {:6.2}", ns(t_unchained));
    }
}
