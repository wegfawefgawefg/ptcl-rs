# Profiling Pass: Callgrind Step Hotspots - 2026-03-02

## Why

After multiple mixed optimization outcomes, we needed hotspot data to guide work instead of guessing.

## Tooling and Constraints

- `perf` was blocked on this machine (`perf_event_paranoid=3`), so kernel sampling was unavailable.
- Used `valgrind --tool=callgrind` with a dedicated profiling binary:
  - `src/bin/profile_step.rs`
  - workload: steady mixed lane system, `50_000` particles, repeated `step()`

## Commands

```bash
cargo build --bin profile_step
valgrind --tool=callgrind --callgrind-out-file=/tmp/callgrind.profile_step.dev.150.out target/debug/profile_step 150
callgrind_annotate --auto=yes --threshold=0.2 /tmp/callgrind.profile_step.dev.150.out
```

## Main Findings

From callgrind (debug build, 150 steps):

- `ParticleSystem::step` is the primary runtime hotspot (`~20.66%`).
- `calculate_bezier_point` inside spline motion is very expensive (`~25.51%` attributable via call path).
- Spline motion update (`step_spline_motion`) is a large share due:
  - bezier evaluation
  - vector math (`sub`, `mul`, `add_assign`)
  - `f32::clamp`
- Core particle update math also contributes materially:
  - `Vec2` add-assign ops
  - repeated `f32::clamp` on alpha/t

## Interpretation

- The biggest compute concentration is in spline behavior math, not spawn APIs.
- This aligns with benchmark patterns where mixed/rich workloads are harder to improve than linear-only paths.

## Caveats

- Callgrind data above is from a debug binary to preserve function attribution.
- Absolute percentages are not release-equivalent; use these as directional hotspots.
- Release callgrind runs heavily inlined into `main`, so attribution quality was poor.

## Actionable Next Step

Focus next optimization attempts on spline-path cost before trying broad structural rewrites:

1. Cache/reuse bezier intermediate terms where possible.
2. Reduce `clamp` frequency or gate clamping where safe.
3. Add spline-only benchmarks to isolate improvements (`step_spline_*` variants).
