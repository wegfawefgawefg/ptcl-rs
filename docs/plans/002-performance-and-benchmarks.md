# Plan 002: Performance Optimization and Benchmarking

## Goal
Increase particle-system performance with measurable outcomes and regression guards.

## Rule
No optimization is accepted without benchmark data.

## Benchmark Strategy
Use two benchmark layers:
- microbenchmarks for core operations
- scenario benchmarks for realistic workloads

## Tooling
- Add `criterion` benches for stable comparative throughput.
- Keep deterministic random seeds for reproducibility.
- Report results as particles/second and frame/update timing.

## Core Metrics
- Spawn throughput: entities spawned per second.
- Update throughput: particles stepped per second.
- Draw preparation cost: per-frame CPU time for visible particles.
- Lifetime churn cost: spawn + despawn heavy workload behavior.
- Stability metrics: p50/p95/p99 update time over long runs.

## Baseline Scenarios
1. `steady_10k`: hold ~10k active particles.
2. `burst_100k`: short burst spawn/despawn stress.
3. `mixed_archetypes`: multiple component combinations active.
4. `long_run`: 5+ minute stability and no pathological drift.

## Optimization Priorities
1. Remove spawn from draw path.
2. Reduce archetype churn (spawn full bundles, avoid repeated insertions).
3. Reduce total world passes in update loop.
4. Reuse temporary buffers in hot paths.
5. Preallocate capacity for known burst sizes.

## Regression Guard
- Keep benchmark baselines committed or archived.
- Add a CI perf check target for critical scenarios.
- Flag regressions above agreed thresholds (example: `>5%` slowdown).

## Acceptance Criteria
- Bench suite runs with one command.
- Baseline numbers exist before major refactors.
- Refactor PRs include before/after benchmark table.
- Core hot-path changes include at least one benchmark update.

## Deliverables
- `benches/` with documented benchmark cases.
- `docs/benchmarks/` with current baseline and methodology.
- repeatable command examples in README/docs.

