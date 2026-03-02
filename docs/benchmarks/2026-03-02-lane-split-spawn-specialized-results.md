# Lane Split + Spawn Specialization - 2026-03-02

## Scope

- Kept lane-split runtime (ballistic + spline lanes).
- Added spawn-lane specialized APIs:
  - `spawn_ballistic_batch`
  - `spawn_spline_batch`
- Updated demo to route spawn batches to the matching lane.

## Command

```bash
cargo bench --bench sim_bench
```

## Comparison Baseline

Compared against packed single-lane baseline from:

- `docs/benchmarks/2026-03-02-packed-rewrite-results.md`

## Results (median estimate)

- `step_steady_10k`: `1.7487 us` -> `1.7182 us` (`~1.7%` faster)
- `step_steady_50k`: `77.529 us` -> `72.216 us` (`~6.9%` faster)
- `burst_100k_lifecycle`: `974.50 us` -> `732.94 us` (`~24.8%` faster)
- `spawn_50k_single`: `434.75 us` -> `462.22 us` (`~6.3%` slower)
- `spawn_50k_batch`: `440.34 us` -> `344.47 us` (`~21.8%` faster)

## Notes

- `step_steady_10k` remains somewhat noisy due very small absolute timing.
- Spawn specialization strongly improved batch spawn throughput while preserving lane-based update gains.
