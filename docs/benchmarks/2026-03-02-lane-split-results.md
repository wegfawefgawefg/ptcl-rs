# Lane Split Results - 2026-03-02

## Scope

- Kept the same spawn-facing API (`ParticleSpawn`, `spawn`, `spawn_batch`).
- Internally split simulation into two lanes:
  - ballistic lane (no spline data)
  - spline lane (always spline)
- Added unified render iteration with `for_each_particle`.

## Command

```bash
cargo bench --bench sim_bench
```

## Comparison Baseline

Compared against packed single-lane baseline from:

- `docs/benchmarks/2026-03-02-packed-rewrite-results.md`

## Results (median estimate)

- `step_steady_10k`: `1.7487 us` -> `0.91298 us` (`~47.8%` faster)
- `step_steady_50k`: `77.529 us` -> `71.430 us` (`~7.9%` faster)
- `burst_100k_lifecycle`: `974.50 us` -> `758.74 us` (`~22.1%` faster)
- `spawn_50k_single`: `434.75 us` -> `466.11 us` (`~7.2%` slower)
- `spawn_50k_batch`: `440.34 us` -> `474.93 us` (`~7.9%` slower)

## Interpretation

- Lane split improves update/lifecycle throughput.
- Spawn throughput regressed moderately due lane routing overhead in the current implementation.
- If spawn-heavy workloads matter, next optimization should target spawn path specialization.
