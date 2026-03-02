# Packed Rewrite Results - 2026-03-02

## Scope

- Removed `hecs` dependency from core.
- Replaced ECS storage/update with packed contiguous `Vec<Particle<T>>` backend.
- Switched core API to spawn specs (`ParticleSpawn`) and batched spawn support.
- Updated demo to use spawn specs while preserving behavior features.

## Commands

```bash
cargo bench --bench sim_bench
cargo tree -e normal | rg hecs
```

`hecs` is no longer present in normal dependencies.

## Baseline Source

Baseline before rewrite is recorded in:

- `docs/benchmarks/2026-03-02-ecs-baseline-before-packed-rewrite.md`

## Before vs After (median estimate)

- `step_steady_10k`: `35.626 us` -> `1.7487 us` (`~95.1%` faster)
- `step_steady_50k`: `176.50 us` -> `77.529 us` (`~56.1%` faster)
- `burst_100k_lifecycle`: `3.3597 ms` -> `0.9745 ms` (`~71.0%` faster)
- `spawn_50k_single`: `3.0534 ms` -> `0.43475 ms` (`~85.8%` faster)
- `spawn_50k_batch`: `2.8725 ms` -> `0.44034 ms` (`~84.7%` faster)

## Notes

- These are benchmark-suite comparisons with identical benchmark names.
- The packed backend keeps current behavior features used by the demo (lifetime, alpha/size/rotation, velocity/acceleration, spline motion).
- `spawn_50k_single` and `spawn_50k_batch` are now very close because both paths are contiguous push-heavy operations over packed storage.
