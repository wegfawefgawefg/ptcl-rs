# Optimization Experiment Log

Track each optimization attempt and its measured impact.

| Date | Experiment | Key Change | Result Summary | Notes |
|---|---|---|---|---|
| 2026-03-02 | Packed rewrite | Removed ECS backend, packed storage | Large gains across step/lifecycle/spawn | See `2026-03-02-packed-rewrite-results.md` |
| 2026-03-02 | Lane split | Ballistic + spline runtime lanes | Faster update/lifecycle, slower spawn | See `2026-03-02-lane-split-results.md` |
| 2026-03-02 | Lane split + spawn specialization | Added specialized lane batch spawns | Strong `spawn_50k_batch` gain, mixed others | See `2026-03-02-lane-split-spawn-specialized-results.md` |
| 2026-03-02 | Experiment 01: compaction vs swap-remove | Compared death compaction strategies under corrected steady-state counters | `swap_remove` clearly better for update/lifecycle; compaction rejected | See `2026-03-02-experiment-01-compaction-vs-swap-remove.md` |
| 2026-03-02 | Experiment 02: ballistic SoA layout | Replaced ballistic AoS lane with SoA columns | Regressed all scenarios, often heavily; rejected | See `2026-03-02-experiment-02-ballistic-soa.md` |
| 2026-03-02 | Experiment 03: branch-minimized core step | Added flag-combo fast paths with generic fallback | Strong step/lifecycle gains, spawn neutral within noise; kept | See `2026-03-02-experiment-03-branch-minimization.md` |
| 2026-03-02 | Experiment 04: thresholded parallel step | Added high-count multithreaded update path | Severe regressions in 50k/lifecycle workloads; rejected | See `2026-03-02-experiment-04-thresholded-parallel-step.md` |
| 2026-03-02 | Experiment 05: linear pair SIMD step | Added pair-wise `Vec4` path for adjacent linear ballistic particles | Linear-only gain, mixed workloads mostly flat/regressive; rejected | See `2026-03-02-experiment-05-linear-pair-simd.md` |
| 2026-03-02 | Experiment 06: spline bezier precompute | Precomputed spline bezier coefficients and used Horner evaluation in step | Neutral-to-positive results, no clear regressions; kept | See `2026-03-02-experiment-06-spline-bezier-precompute.md` |
| 2026-03-02 | Profiling pass: callgrind step hotspots | Added dedicated `profile_step` binary and captured callgrind attribution | Spline path (bezier math) dominates; use profiling-guided optimization next | See `2026-03-02-profiling-callgrind-step.md` |
