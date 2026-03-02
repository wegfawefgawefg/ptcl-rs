# Benchmarks

This folder tracks benchmark methodology and baseline outputs.

## Run

```bash
cargo bench --bench sim_bench
```

## Current Suite

- `step_steady_10k`
- `step_steady_50k`
- `burst_100k_lifecycle`
- `spawn_50k_single`
- `spawn_50k_batch`

## Notes

- Benchmarks are CPU-side simulation benchmarks.
- Use release mode (default for `cargo bench`).
- Compare against prior runs on the same hardware for meaningful trends.

## Baseline Capture

After running benchmarks, copy key means and confidence intervals into a dated file:

- `docs/benchmarks/YYYY-MM-DD-baseline.md`

Include:

- CPU model
- Rust toolchain version
- commit hash
- benchmark output table
