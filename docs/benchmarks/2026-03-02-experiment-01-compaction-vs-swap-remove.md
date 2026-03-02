# Experiment 01: Compaction vs Swap-Remove - 2026-03-02

## Hypothesis

Replacing `swap_remove` death handling with write-index compaction might improve lifecycle/update throughput.

## Benchmark Methodology Fix

Before this experiment comparison, steady-state seeds were updated to use very large counters (`1_000_000_000`) so step benchmarks do not collapse into empty-world timing during long Criterion runs.

## Variants

- Variant A: write-index compaction + step in a single pass
- Variant B: existing `swap_remove` loop (kept)

## Command

```bash
cargo bench --bench sim_bench
```

## Results (median estimate)

- `step_100`: `609.96 ns` (A) vs `397.77 ns` (B) -> B faster `~34.8%`
- `step_1k`: `4.1671 us` (A) vs `2.5617 us` (B) -> B faster `~38.5%`
- `step_steady_10k`: `27.665 us` (A) vs `15.311 us` (B) -> B faster `~44.7%`
- `step_steady_50k`: `134.05 us` (A) vs `72.923 us` (B) -> B faster `~45.6%`
- `burst_100k_lifecycle`: `957.86 us` (A) vs `740.52 us` (B) -> B faster `~22.7%`
- `spawn_50k_single`: `467.93 us` (A) vs `501.36 us` (B) -> A faster `~6.7%`
- `spawn_50k_batch`: `396.14 us` (A) vs `386.55 us` (B) -> B faster `~2.4%`

## Decision

Keep `swap_remove` path.

Compaction regressed the critical update/lifecycle paths significantly. Single-spawn improved slightly under compaction, but not enough to justify the broader regressions.
