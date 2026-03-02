# Experiment 06: Spline Bezier Precompute - 2026-03-02

## Hypothesis

Precomputing quadratic bezier coefficients at spawn and evaluating via Horner form during `step` should reduce spline update cost.

## Variants

- Variant A: `SplineMotion` stores bezier coefficients (`a`, `b`, `c`) and uses `((a * t) + b) * t + c` in step.
- Variant B: step-time direct bezier evaluation from control points each frame.

## Command

```bash
cargo bench --bench sim_bench
```

## Results (median estimate, full-suite run)

- `step_100`: `280.68 ns` (A) vs `395.84 ns` (B) -> A faster `~29.1%`
- `step_1k`: `1.9903 us` (A) vs `2.3640 us` (B) -> A faster `~15.8%`
- `step_steady_10k`: `12.016 us` (A) vs `13.067 us` (B) -> A faster `~8.0%`
- `step_steady_50k`: `58.035 us` (A) vs `60.538 us` (B) -> A faster `~4.1%`
- `step_linear_50k`: `34.981 us` (A) vs `35.004 us` (B) -> roughly unchanged
- `step_rich_50k`: `63.524 us` (A) vs `63.292 us` (B) -> roughly unchanged
- `burst_100k_lifecycle`: `688.48 us` (A) vs `685.87 us` (B) -> roughly unchanged
- `spawn_50k_single`: `492.06 us` (A) vs `511.12 us` (B) -> A faster `~3.7%`
- `spawn_50k_batch`: `351.40 us` (A) vs `398.61 us` (B) -> A faster `~11.8%`

## Stability Notes

- Criterion significance in this run marked several metrics as "no significant change", so improvements should be treated as directional until corroborated by baseline-saved A/B runs.
- Follow-up targeted reruns (`step_linear_50k`, `step_rich_50k`, `step_steady_50k`) were consistent with neutral-to-positive behavior and no clear regressions.

## Decision

Keep Experiment 06 changes.

The implementation is low complexity, API-neutral, and showed no clear downside with a generally favorable trend in step workloads.
