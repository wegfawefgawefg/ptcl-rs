# Benchmarks

This folder tracks benchmark methodology and baseline outputs.

## Run

```bash
cargo bench --bench sim_bench
```

For stable experiment A/B tracking across code changes:

```bash
cargo bench --bench sim_bench --save-baseline <name>
cargo bench --bench sim_bench --baseline <name>
```

## Current Suite

- `step_100`
- `step_1k`
- `step_steady_10k`
- `step_steady_50k`
- `step_linear_50k`
- `step_rich_50k`
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

## Recorded Runs

- `2026-03-02-ecs-baseline-before-packed-rewrite.md`
- `2026-03-02-packed-rewrite-results.md`
- `2026-03-02-lane-split-results.md`
- `2026-03-02-lane-split-spawn-specialized-results.md`
- `2026-03-02-experiment-01-compaction-vs-swap-remove.md`
- `2026-03-02-experiment-02-ballistic-soa.md`
- `2026-03-02-experiment-03-branch-minimization.md`
- `2026-03-02-experiment-04-thresholded-parallel-step.md`
- `2026-03-02-experiment-05-linear-pair-simd.md`
- `2026-03-02-experiment-06-spline-bezier-precompute.md`
- `2026-03-02-profiling-callgrind-step.md`
- `optimization-experiment-log.md`

Include:

- CPU model
- Rust toolchain version
- commit hash
- benchmark output table
