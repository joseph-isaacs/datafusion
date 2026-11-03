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
        let w: Vec<i64> = v.iter().map(|x| x ^ 7).collect();
        assert!(k_foldmul_check(&v), "emulated folded multiply must match");
        let arr = make_i64(v.clone());
        let arr2 = make_i64(w.clone());
        let strs = make_utf8(v.iter().map(|x| format!("key-{x:016x}")).collect());
        let mut out = vec![0u64; rows];
        println!("rows = {rows}  (ns/row, best of 15)");
        macro_rules! r {
            ($n:expr, $e:expr) => {
                println!(
                    "  {:<40} {:.3}",
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
        r!(
            "df create_hashes i64 (foldhash)",
            df_create_hashes_i64(black_box(&arr), &mut out)
        );
        r!("mix_lo scalar", k_mix_lo(black_box(&v), &mut out));
        r!("mix_lo AVX-512 (vpmullq)", unsafe {
            k_mix_lo_avx512(black_box(&v), &mut out)
        });
        r!("foldhash-identical, AVX-512 emulated mul", unsafe {
            k_foldmul_emul_avx512(black_box(&v), &mut out)
        });
        r!(
            "df create_hashes 2 x i64 (two passes)",
            df_create_hashes_i64x2(black_box(&arr), black_box(&arr2), &mut out)
        );
        r!(
            "fused 2 x i64 foldhash (one pass)",
            k_fused2_foldhash(black_box(&v), black_box(&w), &mut out)
        );
        r!(
            "df create_hashes utf8 (20-byte keys)",
            df_create_hashes_utf8(black_box(&strs), &mut out)
        );
    }
}
