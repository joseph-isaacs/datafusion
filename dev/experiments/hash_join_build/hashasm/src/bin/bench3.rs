use hashasm::*;
use std::hint::black_box;
use std::time::Instant;

fn time<F: FnMut()>(mut f: F, rows: usize) -> f64 {
    for _ in 0..3 {
        f();
    }
    let mut best = f64::MAX;
    for _ in 0..15 {
        let iters = (50_000_000 / rows).max(1);
        let t = Instant::now();
        for _ in 0..iters {
            f();
        }
        best = best.min(t.elapsed().as_nanos() as f64 / (iters * rows) as f64);
    }
    best
}

fn main() {
    for &rows in &[8192usize, 4_000_000] {
        let v: Vec<i64> = (0..rows as i64)
            .map(|x| x.wrapping_mul(0x5851F42D4C957F2D))
            .collect();
        let (mut a, mut b, mut c, mut d) = (
            vec![0u64; rows],
            vec![0u64; rows],
            vec![0u64; rows],
            vec![0u64; rows],
        );
        k_fold_scalar(&v, &mut a);
        k_fold_sse2(&v, &mut b);
        unsafe { k_fold_avx2(&v, &mut c) };
        unsafe { k_foldmul_emul_avx512(&v, &mut d) };
        assert!(
            a == b && a == c && a == d,
            "all variants must be bit-identical"
        );
        let mut out = vec![0u64; rows];
        println!("rows = {rows}  (ns/row, best of 15) — outputs verified identical");
        macro_rules! r {
            ($n:expr, $e:expr) => {
                println!(
                    "  {:<34} {:.3}",
                    $n,
                    time(
                        || {
                            $e;
                            black_box(&mut out);
                        },
                        rows
                    )
                );
            };
        }
        r!("scalar mul (today)", k_fold_scalar(black_box(&v), &mut out));
        r!(
            "SSE2 intrinsics, 2 lanes",
            k_fold_sse2(black_box(&v), &mut out)
        );
        r!("AVX2 intrinsics, 4 lanes", unsafe {
            k_fold_avx2(black_box(&v), &mut out)
        });
        r!("AVX-512 auto-vectorized, 8 lanes", unsafe {
            k_foldmul_emul_avx512(black_box(&v), &mut out)
        });
    }
}
