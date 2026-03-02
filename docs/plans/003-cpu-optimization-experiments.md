# Plan 003: CPU Optimization Experiments

## Goal

Push CPU particle throughput further while keeping the library API simple and renderer-agnostic.

## Experiment Rules

- Change one major optimization at a time when possible.
- Benchmark before and after each change.
- Record both wins and regressions.
- Keep notes on tradeoffs (small particle counts, spawn-heavy workloads, few-core machines).

## Benchmark Matrix

Primary scenarios:

- `step_100`
- `step_1k`
- `step_steady_10k`
- `step_steady_50k`
- `step_linear_50k` (diagnostic)
- `step_rich_50k` (diagnostic)
- `burst_100k_lifecycle`
- `spawn_50k_single`
- `spawn_50k_batch`

Hardware context checks:

- Standard run on full machine.
- Optional reduced-core run (for parallel experiments) with:
  - `taskset` / affinity pinning
  - thread-count limits

## Candidate Optimizations

1. Lane compaction strategy:
   - compare `swap_remove` loop vs write-index compaction.
2. Spawn path specialization:
   - keep lane routing explicit and benchmark direct lane batch APIs.
3. Lane-specific data layout:
   - evaluate SoA for hottest lanes.
4. Branch minimization:
   - split by common flag combinations where practical.
5. Parallel step:
   - optional threshold-based multithreaded update for large counts.
6. SIMD-focused math path:
   - investigate `std::simd` for bulk math updates.

## Progress Notes

- Experiment 01 (`swap_remove` vs compaction): compaction rejected.
- Experiment 02 (ballistic SoA lane): rejected.
- Experiment 03 (branch-minimized core step): kept.
- Experiment 04 (thresholded parallel step): rejected.
- Experiment 05 (linear pair SIMD step): rejected.
- Experiment 06 (spline bezier precompute): kept.
- Profiling pass (callgrind): spline path and bezier math are dominant hotspots.

## Success Criteria

- Measurable improvement in target scenarios with stable behavior.
- No large regressions in small-count scenarios unless explicitly accepted.
- Full record exists for each attempt in benchmark docs/log.
