# Optimization Pass 1 - 2026-03-02

## Scope

- Added core `spawn_batch` fast path.
- Added core `reserve_bundle` for archetype-specific preallocation.
- Batched demo emissions with reusable buffers.
- Switched demo RNG to `SmallRng`.
- Added spawn throughput benchmarks.

## Command

```bash
cargo bench --bench sim_bench
```

## Results

- `step_steady_10k`: `[35.338 us, 35.462 us, 35.579 us]`
- `step_steady_50k`: `[176.18 us, 176.58 us, 177.04 us]`
- `burst_100k_lifecycle`: `[3.3479 ms, 3.3666 ms, 3.3853 ms]`
- `spawn_50k_single`: `[3.1209 ms, 3.1237 ms, 3.1272 ms]`
- `spawn_50k_batch`: `[2.8335 ms, 2.8456 ms, 2.8585 ms]`

## Notes

- Relative to the previous baseline in this branch, criterion reported statistically significant improvements for all three existing step/lifecycle benchmarks.
- `spawn_50k_batch` is about 9% faster than `spawn_50k_single` in this run.
