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
        let arr = make_i64(v.clone());
        let arr2 = make_i64(v.iter().map(|x| x ^ 7).collect());
        let mut out = vec![0u64; rows];
        println!("rows = {rows}  (ns/row, best of 15)");
        macro_rules! r {
            ($n:expr, $e:expr) => {
                println!(
                    "  {:<28} {:.3}",
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
            "df create_hashes i64",
            df_create_hashes_i64(black_box(&arr), &mut out)
        );
        r!(
            "df create_hashes i64 x2col",
            df_create_hashes_i64x2(black_box(&arr), black_box(&arr2), &mut out)
        );
        r!(
            "foldhash fast (plain loop)",
            k_foldhash_fast(black_box(&v), &mut out)
        );
        r!(
            "foldhash quality",
            k_foldhash_quality(black_box(&v), &mut out)
        );
        r!("ahash", k_ahash(black_box(&v), &mut out));
        r!("rapidhash", k_rapidhash(black_box(&v), &mut out));
        r!("mul-xorshift (imul)", k_mulxor(black_box(&v), &mut out));
        r!("crc32 + imul", unsafe { k_crc32(black_box(&v), &mut out) });
    }
}
