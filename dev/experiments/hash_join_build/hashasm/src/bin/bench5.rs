//! Same source, different target CPUs. Interleaved rounds, median ns/row.
use hashasm::*;
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let rows_list = [8192usize, 4_000_000];
    const ROUNDS: usize = 31;
    for &rows in &rows_list {
        let v: Vec<i64> = (0..rows as i64)
            .map(|x| x.wrapping_mul(0x5851F42D4C957F2D))
            .collect();
        let arr = make_i64(v.clone());
        let mut a = vec![0u64; rows];
        let mut b = vec![0u64; rows];
        k_fold_scalar(&v, &mut a);
        k_split_plain(&v, &mut b);
        assert_eq!(a, b, "split must match the 128-bit multiply");
        let variants: Vec<(&str, Box<dyn Fn(&mut [u64])>)> = vec![
            (
                "df create_hashes i64 (today)",
                Box::new(|o: &mut [u64]| df_create_hashes_i64(black_box(&arr), o)),
            ),
            (
                "128-bit mul, plain loop",
                Box::new(|o: &mut [u64]| k_fold_scalar(black_box(&v), o)),
            ),
            (
                "32-bit split, plain loop",
                Box::new(|o: &mut [u64]| k_split_plain(black_box(&v), o)),
            ),
        ];
        let iters = (20_000_000 / rows).max(3);
        let mut out = vec![0u64; rows];
        let mut samples = vec![Vec::new(); variants.len()];
        for (_, f) in &variants {
            for _ in 0..iters {
                f(&mut out);
            }
        }
        for round in 0..ROUNDS {
            for k in 0..variants.len() {
                let i = (k + round) % variants.len();
                let t = Instant::now();
                for _ in 0..iters {
                    (variants[i].1)(&mut out);
                    black_box(&mut out);
                }
                samples[i].push(t.elapsed().as_nanos() as f64 / (iters * rows) as f64);
            }
        }
        for (i, (name, _)) in variants.iter().enumerate() {
            samples[i].sort_by(f64::total_cmp);
            println!(
                "  rows={rows:<8} {:<30} median {:.3}  p90 {:.3}",
                name,
                samples[i][ROUNDS / 2],
                samples[i][ROUNDS * 9 / 10]
            );
        }
    }
}
