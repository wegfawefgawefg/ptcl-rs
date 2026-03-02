# Experiment 04: Thresholded Parallel Step - 2026-03-02

## Hypothesis

Using a multithreaded `step` path only at high particle counts should improve large workloads while keeping small workloads unchanged.

## Variants

- Variant A: thresholded parallel step path for large total particle counts.
- Variant B: current branch-minimized single-threaded step path.

## Command

```bash
cargo bench --bench sim_bench
```

## Results (median estimate)

- `step_100`: `336.74 ns` (A) vs `395.84 ns` (B) -> A faster `~14.9%` (Criterion: noise threshold)
- `step_1k`: `2.2298 us` (A) vs `2.3640 us` (B) -> A faster `~5.7%` (Criterion: no significant change)
- `step_steady_10k`: `12.986 us` (A) vs `13.067 us` (B) -> A faster `~0.6%` (Criterion: no significant change)
- `step_steady_50k`: `265.45 us` (A) vs `60.538 us` (B) -> A slower `~338.5%`
- `step_linear_50k`: `243.41 us` (A) vs `35.004 us` (B) -> A slower `~595.4%`
- `step_rich_50k`: `273.26 us` (A) vs `63.292 us` (B) -> A slower `~331.7%`
- `burst_100k_lifecycle`: `1.9225 ms` (A) vs `685.87 us` (B) -> A slower `~180.3%`
- `spawn_50k_single`: `507.09 us` (A) vs `511.12 us` (B) -> A faster `~0.8%` (not meaningful)
- `spawn_50k_batch`: `385.02 us` (A) vs `398.61 us` (B) -> A faster `~3.4%` (not meaningful)

## Decision

Reject Experiment 04 and keep single-threaded step.

The parallel path introduced severe regressions in high-count update/lifecycle workloads, which are exactly the target scenarios for this optimization.
