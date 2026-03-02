# Experiment 05: Linear Pair SIMD Step - 2026-03-02

## Hypothesis

Using a SIMD-style pair update for adjacent ballistic particles with `HAS_VELOCITY | HAS_ACCELERATION` should improve step throughput.

## Variants

- Variant A: pair-wise `Vec4` update for adjacent linear ballistic particles.
- Variant B: current branch-minimized scalar step path.

## Command

```bash
cargo bench --bench sim_bench
```

## Results (median estimate)

- `step_100`: `372.19 ns` (A) vs `395.84 ns` (B) -> A faster `~6.0%` (no significant change)
- `step_1k`: `2.3879 us` (A) vs `2.3640 us` (B) -> A slower `~1.0%` (no significant change)
- `step_steady_10k`: `13.205 us` (A) vs `13.067 us` (B) -> A slower `~1.1%` (noise threshold)
- `step_steady_50k`: `61.481 us` (A) vs `60.538 us` (B) -> A slower `~1.6%` (no significant change)
- `step_linear_50k`: `31.680 us` (A) vs `35.004 us` (B) -> A faster `~9.5%`
- `step_rich_50k`: `65.519 us` (A) vs `63.292 us` (B) -> A slower `~3.5%`
- `burst_100k_lifecycle`: `674.80 us` (A) vs `685.87 us` (B) -> A faster `~1.6%`
- `spawn_50k_single`: `480.43 us` (A) vs `511.12 us` (B) -> A faster `~6.0%` (no significant change)
- `spawn_50k_batch`: `397.51 us` (A) vs `398.61 us` (B) -> A faster `~0.3%` (no significant change)

## Decision

Reject Experiment 05 and keep scalar branch-minimized step.

The SIMD pair path improved the linear-only diagnostic workload, but mixed real workloads were mostly flat and introduced regressions in rich-heavy updates. The extra complexity is not justified by the net result.
