# Experiment 02: Ballistic SoA Layout - 2026-03-02

## Hypothesis

Changing the ballistic lane from AoS (`Vec<ParticleCore<T>>`) to SoA (parallel vectors per field) would improve hot `step` throughput.

## Variants

- Variant A: ballistic lane SoA (`BallisticSoa<T>`) + existing spline AoS lane.
- Variant B: current baseline (lane-split AoS with `swap_remove` death handling).

## Command

```bash
cargo bench --bench sim_bench
```

## Results (median estimate)

- `step_100`: `814.97 ns` (A) vs `397.77 ns` (B) -> A slower `~104.9%`
- `step_1k`: `7.1640 us` (A) vs `2.5617 us` (B) -> A slower `~179.7%`
- `step_steady_10k`: `59.704 us` (A) vs `15.311 us` (B) -> A slower `~289.9%`
- `step_steady_50k`: `301.18 us` (A) vs `72.923 us` (B) -> A slower `~313.0%`
- `burst_100k_lifecycle`: `3.5251 ms` (A) vs `740.52 us` (B) -> A slower `~376.0%`
- `spawn_50k_single`: `717.72 us` (A) vs `501.36 us` (B) -> A slower `~43.2%`
- `spawn_50k_batch`: `685.29 us` (A) vs `386.55 us` (B) -> A slower `~77.3%`

## Decision

Reject Experiment 02 and keep AoS lane storage.

This SoA implementation regressed every measured scenario, with especially large regressions in update/lifecycle paths.
