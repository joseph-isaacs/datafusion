# hashbrown-pipelined

Experimental fork of [hashbrown](https://github.com/rust-lang/hashbrown)
0.17.1 (MIT OR Apache-2.0, see `LICENSE-MIT` and `LICENSE-APACHE`).

The only change is `HashTable::prefetch(hash, data)` (and
`RawTable::prefetch`), which prefetches the control group and optionally the
first data slot that `HashTable::find(hash, ..)` reads first. It is used by
`datafusion/physical-plan/src/joins/pipelined_hash_map.rs` to software
pipeline hash join probes.

`rustfmt.toml` keeps upstream's 100 column width so that `cargo fmt --all` in
the DataFusion workspace leaves the sources unchanged.
