# Optimization Experiment Log

Track each optimization attempt and its measured impact.

| Date | Experiment | Key Change | Result Summary | Notes |
|---|---|---|---|---|
| 2026-03-02 | Packed rewrite | Removed ECS backend, packed storage | Large gains across step/lifecycle/spawn | See `2026-03-02-packed-rewrite-results.md` |
| 2026-03-02 | Lane split | Ballistic + spline runtime lanes | Faster update/lifecycle, slower spawn | See `2026-03-02-lane-split-results.md` |
| 2026-03-02 | Lane split + spawn specialization | Added specialized lane batch spawns | Strong `spawn_50k_batch` gain, mixed others | See `2026-03-02-lane-split-spawn-specialized-results.md` |
| 2026-03-02 | Experiment 01: compaction vs swap-remove | Compared death compaction strategies under corrected steady-state counters | `swap_remove` clearly better for update/lifecycle; compaction rejected | See `2026-03-02-experiment-01-compaction-vs-swap-remove.md` |
| 2026-03-02 | Experiment 02: ballistic SoA layout | Replaced ballistic AoS lane with SoA columns | Regressed all scenarios, often heavily; rejected | See `2026-03-02-experiment-02-ballistic-soa.md` |
