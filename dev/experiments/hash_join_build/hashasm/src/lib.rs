use arrow::array::{ArrayRef, Int64Array};
use datafusion_common::hash_utils::{RandomState, create_hashes};
use std::hash::BuildHasher;

/// The real DataFusion path for one Int64 column, no nulls.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn df_create_hashes_i64(arr: &ArrayRef, out: &mut [u64]) {
    let state = RandomState::with_seed(12210250226015887276);
    create_hashes([arr], &state, out).unwrap();
}

/// The real DataFusion path for two Int64 columns (rehash path).
#[unsafe(no_mangle)]
#[inline(never)]
pub fn df_create_hashes_i64x2(a: &ArrayRef, b: &ArrayRef, out: &mut [u64]) {
    let state = RandomState::with_seed(12210250226015887276);
    create_hashes([a, b], &state, out).unwrap();
}

macro_rules! kernel {
    ($name:ident, $st:expr) => {
        #[unsafe(no_mangle)]
        #[inline(never)]
        pub fn $name(v: &[i64], out: &mut [u64]) {
            let s = $st;
            for (o, x) in out.iter_mut().zip(v) {
                *o = s.hash_one(*x);
            }
        }
    };
}
kernel!(
    k_foldhash_fast,
    foldhash::fast::FixedState::with_seed(12210250226015887276)
);
kernel!(
    k_foldhash_quality,
    foldhash::quality::FixedState::with_seed(12210250226015887276)
);
kernel!(k_ahash, ahash::RandomState::with_seeds(1, 2, 3, 4));
kernel!(
    k_rapidhash,
    rapidhash::fast::SeedableState::new(12210250226015887276)
);

/// Minimal multiply-xorshift mixer (what a specialised int-key hash could be).
#[unsafe(no_mangle)]
#[inline(never)]
pub fn k_mulxor(v: &[i64], out: &mut [u64]) {
    const K: u64 = 0x9E37_79B9_7F4A_7C15;
    for (o, x) in out.iter_mut().zip(v) {
        let h = (*x as u64 ^ 12210250226015887276).wrapping_mul(K);
        *o = h ^ (h >> 32);
    }
}

/// Folded 64x64->128 multiply (foldhash's core op, done by hand).
#[unsafe(no_mangle)]
#[inline(never)]
pub fn k_foldmul(v: &[i64], out: &mut [u64]) {
    const K: u64 = 0x243F_6A88_85A3_08D3;
    for (o, x) in out.iter_mut().zip(v) {
        let p = ((*x as u64 ^ 12210250226015887276) as u128) * (K as u128);
        *o = (p as u64) ^ ((p >> 64) as u64);
    }
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
#[inline(never)]
#[target_feature(enable = "sse4.2")]
pub unsafe fn k_crc32(v: &[i64], out: &mut [u64]) {
    use std::arch::x86_64::_mm_crc32_u64;
    for (o, x) in out.iter_mut().zip(v) {
        let c = _mm_crc32_u64(0x1234_5678, *x as u64);
        // crc32 alone only gives 32 bits; spread into the high bits for hashbrown's tag.
        *o = c | (c.wrapping_mul(0x9E37_79B9_7F4A_7C15) & 0xFFFF_FFFF_0000_0000);
    }
}

pub fn make_i64(v: Vec<i64>) -> ArrayRef {
    std::sync::Arc::new(Int64Array::from(v))
}

use arrow::array::StringArray;

/// Real DataFusion path for one Utf8 column.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn df_create_hashes_utf8(arr: &ArrayRef, out: &mut [u64]) {
    let state = RandomState::with_seed(12210250226015887276);
    create_hashes([arr], &state, out).unwrap();
}

pub fn make_utf8(v: Vec<String>) -> ArrayRef {
    std::sync::Arc::new(StringArray::from(v))
}

/// A vectorizable 64-bit mix: two 64x64->64 (low) multiplies, no 128-bit
/// product, so AVX-512DQ can do 8 lanes per `vpmullq`.
#[inline(always)]
fn mix_lo(x: u64) -> u64 {
    let mut h = x ^ 12210250226015887276;
    h = (h ^ (h >> 32)).wrapping_mul(0xD6E8_FEB8_6659_FD93);
    h = (h ^ (h >> 32)).wrapping_mul(0xD6E8_FEB8_6659_FD93);
    h ^ (h >> 32)
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn k_mix_lo(v: &[i64], out: &mut [u64]) {
    for (o, x) in out.iter_mut().zip(v) {
        *o = mix_lo(*x as u64);
    }
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
#[inline(never)]
#[target_feature(enable = "avx512f,avx512dq")]
pub unsafe fn k_mix_lo_avx512(v: &[i64], out: &mut [u64]) {
    for (o, x) in out.iter_mut().zip(v) {
        *o = mix_lo(*x as u64);
    }
}

/// foldhash's folded 128-bit multiply, emulated with 32x32->64 multiplies so
/// it can vectorize (`vpmuludq`). Bit-identical to the scalar `mul` fold.
#[inline(always)]
fn folded_mul_emul(x: u64, k: u64) -> u64 {
    let (xl, xh) = (x & 0xFFFF_FFFF, x >> 32);
    let (kl, kh) = (k & 0xFFFF_FFFF, k >> 32);
    let ll = xl * kl;
    let lh = xl * kh;
    let hl = xh * kl;
    let hh = xh * kh;
    let mid = (ll >> 32) + (lh & 0xFFFF_FFFF) + (hl & 0xFFFF_FFFF);
    let lo = (ll & 0xFFFF_FFFF) | (mid << 32);
    let hi = hh + (lh >> 32) + (hl >> 32) + (mid >> 32);
    lo ^ hi
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
#[inline(never)]
#[target_feature(enable = "avx512f,avx512dq")]
pub unsafe fn k_foldmul_emul_avx512(v: &[i64], out: &mut [u64]) {
    const K: u64 = 0x243F_6A88_85A3_08D3;
    for (o, x) in out.iter_mut().zip(v) {
        *o = folded_mul_emul(*x as u64 ^ 12210250226015887276, K);
    }
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn k_foldmul_check(v: &[i64]) -> bool {
    const K: u64 = 0x243F_6A88_85A3_08D3;
    v.iter().all(|x| {
        let y = *x as u64 ^ 12210250226015887276;
        let p = (y as u128) * (K as u128);
        folded_mul_emul(y, K) == ((p as u64) ^ ((p >> 64) as u64))
    })
}

/// Two Int64 columns hashed in one pass (fused), vs DataFusion's two passes.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn k_fused2_foldhash(a: &[i64], b: &[i64], out: &mut [u64]) {
    let s = foldhash::fast::FixedState::with_seed(12210250226015887276);
    for ((o, x), y) in out.iter_mut().zip(a).zip(b) {
        *o = s.hash_one((*x, *y));
    }
}

const FA: u64 = 12210250226015887276;
const FK: u64 = 0x243F_6A88_85A3_08D3;

#[inline(always)]
fn scalar_fold(x: u64) -> u64 {
    let p = ((x ^ FA) as u128) * (FK as u128);
    (p as u64) ^ ((p >> 64) as u64)
}

/// Same function with explicit SSE2 intrinsics (2 lanes), baseline x86-64.
#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
#[inline(never)]
pub fn k_fold_sse2(values: &[i64], out: &mut [u64]) {
    use std::arch::x86_64::*;
    let n = values.len().min(out.len()) / 2 * 2;
    unsafe {
        let va = _mm_set1_epi64x(FA as i64);
        let kl = _mm_set1_epi64x((FK & 0xFFFF_FFFF) as i64);
        let kh = _mm_set1_epi64x((FK >> 32) as i64);
        let m32 = _mm_set1_epi64x(0xFFFF_FFFF);
        for i in (0..n).step_by(2) {
            let y = _mm_xor_si128(_mm_loadu_si128(values.as_ptr().add(i).cast()), va);
            let yh = _mm_srli_epi64::<32>(y);
            let ll = _mm_mul_epu32(y, kl);
            let lh = _mm_mul_epu32(y, kh);
            let hl = _mm_mul_epu32(yh, kl);
            let hh = _mm_mul_epu32(yh, kh);
            let mid = _mm_add_epi64(
                _mm_add_epi64(_mm_srli_epi64::<32>(ll), _mm_and_si128(lh, m32)),
                _mm_and_si128(hl, m32),
            );
            let lo = _mm_or_si128(_mm_and_si128(ll, m32), _mm_slli_epi64::<32>(mid));
            let hi = _mm_add_epi64(
                _mm_add_epi64(hh, _mm_srli_epi64::<32>(lh)),
                _mm_add_epi64(_mm_srli_epi64::<32>(hl), _mm_srli_epi64::<32>(mid)),
            );
            _mm_storeu_si128(out.as_mut_ptr().add(i).cast(), _mm_xor_si128(lo, hi));
        }
    }
    for i in n..values.len().min(out.len()) {
        out[i] = scalar_fold(values[i] as u64);
    }
}

/// Same with AVX2 (4 lanes).
#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
#[inline(never)]
#[target_feature(enable = "avx2")]
pub unsafe fn k_fold_avx2(values: &[i64], out: &mut [u64]) {
    use std::arch::x86_64::*;
    let n = values.len().min(out.len()) / 4 * 4;
    let va = _mm256_set1_epi64x(FA as i64);
    let kl = _mm256_set1_epi64x((FK & 0xFFFF_FFFF) as i64);
    let kh = _mm256_set1_epi64x((FK >> 32) as i64);
    let m32 = _mm256_set1_epi64x(0xFFFF_FFFF);
    for i in (0..n).step_by(4) {
        unsafe {
            let y =
                _mm256_xor_si256(_mm256_loadu_si256(values.as_ptr().add(i).cast()), va);
            let yh = _mm256_srli_epi64::<32>(y);
            let ll = _mm256_mul_epu32(y, kl);
            let lh = _mm256_mul_epu32(y, kh);
            let hl = _mm256_mul_epu32(yh, kl);
            let hh = _mm256_mul_epu32(yh, kh);
            let mid = _mm256_add_epi64(
                _mm256_add_epi64(_mm256_srli_epi64::<32>(ll), _mm256_and_si256(lh, m32)),
                _mm256_and_si256(hl, m32),
            );
            let lo =
                _mm256_or_si256(_mm256_and_si256(ll, m32), _mm256_slli_epi64::<32>(mid));
            let hi = _mm256_add_epi64(
                _mm256_add_epi64(hh, _mm256_srli_epi64::<32>(lh)),
                _mm256_add_epi64(
                    _mm256_srli_epi64::<32>(hl),
                    _mm256_srli_epi64::<32>(mid),
                ),
            );
            _mm256_storeu_si256(out.as_mut_ptr().add(i).cast(), _mm256_xor_si256(lo, hi));
        }
    }
    for i in n..values.len().min(out.len()) {
        out[i] = scalar_fold(values[i] as u64);
    }
}

/// Scalar reference, same constants.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn k_fold_scalar(values: &[i64], out: &mut [u64]) {
    for (o, x) in out.iter_mut().zip(values) {
        *o = scalar_fold(*x as u64);
    }
}

/// Plain Rust, no target_feature: the 32-bit split of foldhash's multiply.
/// Whether this vectorizes depends only on the build's target CPU.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn k_split_plain(values: &[i64], out: &mut [u64]) {
    for (o, x) in out.iter_mut().zip(values) {
        *o = folded_mul_emul(*x as u64 ^ FA, FK);
    }
}
