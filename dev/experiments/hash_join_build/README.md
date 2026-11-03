# Hash join build experiments (not for merge)

Scratch code behind the "Hash join build performance — handover" doc. Nothing
here is meant to be merged into `main`.

- `../../../datafusion/physical-plan/benches/hash_join_build_phases.rs`: times
  each build phase (allocation, hashing, insert, concat) and the insert/probe
  prototypes: hashbrown, hashbrown grouped by home bucket, hashbrown with
  prefetch, an own linear-probing table, an own cache-line-bucket table, an
  unchained table. `HUGE=1` asks for transparent huge pages on allocations of
  4 MiB or more; `AHEAD=<n>` sets the prefetch distance; `PBITS=<n>` the
  partition bits.

  ```shell
  cargo bench --profile release-nonlto -p datafusion-physical-plan \
      --features test_utils --bench hash_join_build_phases
  ```

- `hashasm/`: standalone crate comparing hash kernels against DataFusion's
  `create_hashes` (foldhash, ahash, rapidhash, crc32, the 32-bit-split folded
  multiply in scalar, SSE2, AVX2 and AVX-512 forms). `bench4` and `bench5` are
  the reliable ones: interleaved rounds, median and p90.

  ```shell
  cd hashasm
  taskset -c 2 cargo run --release --bin bench4
  # same source, different target CPUs
  RUSTFLAGS='-C target-cpu=x86-64-v3' CARGO_TARGET_DIR=target-v3 \
      cargo run --release --bin bench5
  ```

- `create_hashes_i64_hash_single_array.s`: assembly of DataFusion's
  `hash_single_array` for one `Int64` column (default target).
