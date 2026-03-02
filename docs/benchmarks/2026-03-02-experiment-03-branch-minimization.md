# Experiment 03: Branch-Minimized Core Step - 2026-03-02

## Hypothesis

Adding specialized fast paths for common particle flag combinations should improve hot update throughput by reducing branch work in `step_core_particle`.

## Variants

- Variant A: specialized flag-combo fast paths + generic fallback.
- Variant B: prior generic branch-per-flag core step (baseline from Experiment 01 kept variant).

## Command

```bash
cargo bench --bench sim_bench
```

## Results (median estimate)

- `step_100`: `395.84 ns` (A) vs `397.77 ns` (B) -> A faster `~0.5%`
- `step_1k`: `2.3640 us` (A) vs `2.5617 us` (B) -> A faster `~7.7%`
- `step_steady_10k`: `13.067 us` (A) vs `15.311 us` (B) -> A faster `~14.7%`
- `step_steady_50k`: `60.538 us` (A) vs `72.923 us` (B) -> A faster `~17.0%`
- `burst_100k_lifecycle`: `685.87 us` (A) vs `740.52 us` (B) -> A faster `~7.4%`
- `spawn_50k_single`: `511.12 us` (A) vs `501.36 us` (B) -> A slower `~1.9%`
- `spawn_50k_batch`: `398.61 us` (A) vs `386.55 us` (B) -> A slower `~3.1%`

## Spawn Rerun Check

Targeted reruns after the full suite:

- `spawn_50k_single`: `496.17 us`, Criterion change `[-4.3604% .. +1.9921%]`, `p = 0.50` (no significant change)
- `spawn_50k_batch`: `399.14 us`, Criterion change `[-4.1452% .. +7.4625%]`, `p = 0.55` (no significant change)

## Archetype Diagnostics

Added targeted benchmarks to separate linear and rich ballistic particles:

- `step_linear_50k`: `35.004 us`
- `step_rich_50k`: `63.292 us`

Rich particles are about `1.81x` the per-step cost of linear particles in this setup, which is expected from additional size/rotation/alpha work.

## Decision

Keep Experiment 03 changes.

Update/lifecycle throughput improved meaningfully. Initial full-suite run showed small spawn regressions, but targeted reruns of `spawn_50k_single` and `spawn_50k_batch` reported no statistically significant change.
