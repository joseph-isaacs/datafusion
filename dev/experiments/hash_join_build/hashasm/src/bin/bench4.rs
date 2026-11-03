//! Interleaved, repeated timing: each round runs every variant once in a
//! rotating order; reports min / median / p90 over rounds.
use hashasm::*;
use std::hint::black_box;
use std::time::Instant;

type K = fn(&[i64], &mut [u64]);

fn main() {
    let variants: [(&str, K); 4] = [
        ("scalar mul (today)", k_fold_scalar),
        ("SSE2, 2 lanes", k_fold_sse2),
        ("AVX2, 4 lanes", |v, o| unsafe { k_fold_avx2(v, o) }),
        ("AVX-512, 8 lanes", |v, o| unsafe {
            k_foldmul_emul_avx512(v, o)
        }),
    ];
    const ROUNDS: usize = 41;
    for &rows in &[8192usize, 4_000_000] {
        let v: Vec<i64> = (0..rows as i64)
            .map(|x| x.wrapping_mul(0x5851F42D4C957F2D))
            .collect();
        let mut out = vec![0u64; rows];
        // ~20 ms of work per sample.
        let iters = (20_000_000 / rows).max(3);
        let mut samples = vec![Vec::new(); variants.len()];
        for (_, f) in &variants {
            for _ in 0..iters {
                f(&v, &mut out);
            }
        } // warm up
        for round in 0..ROUNDS {
            for k in 0..variants.len() {
                let i = (k + round) % variants.len();
                let f = variants[i].1;
                let t = Instant::now();
                for _ in 0..iters {
                    f(black_box(&v), &mut out);
                    black_box(&mut out);
                }
                samples[i].push(t.elapsed().as_nanos() as f64 / (iters * rows) as f64);
            }
        }
        println!("rows = {rows}: ns/row over {ROUNDS} interleaved rounds");
        println!(
            "  {:<22} {:>7} {:>7} {:>7} {:>9}",
            "variant", "min", "median", "p90", "vs today"
        );
        let med = |s: &mut Vec<f64>| {
            s.sort_by(f64::total_cmp);
            (s[0], s[s.len() / 2], s[s.len() * 9 / 10])
        };
        let base = med(&mut samples[0].clone()).1;
        for (i, (name, _)) in variants.iter().enumerate() {
            let (mn, md, p90) = med(&mut samples[i]);
            println!(
                "  {:<22} {:>7.3} {:>7.3} {:>7.3} {:>8.2}x",
                name,
                mn,
                md,
                p90,
                base / md
            );
        }
    }
}
