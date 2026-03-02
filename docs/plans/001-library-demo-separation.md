# Plan 001: Separate Library and Demo

## Goal
Split the current mixed codebase into:
- a reusable particle library (core)
- one or more demo applications (consumers of core)

The demo should showcase usage patterns, not define core architecture.

## Current Problem
- Core simulation and demo behavior are conflated.
- Demo-specific particle archetypes and spawn logic are in shared paths.
- Render-time behavior currently influences simulation behavior.

## Target Structure
- `src/lib.rs`: public core API.
- `src/core/*`: ECS/system internals, components, update loop, spawn internals.
- `src/api/*`: ergonomic builder/emitter interfaces.
- `examples/` or `src/bin/`: demo executables and showcase scenes.
- `assets/`: demo assets only (core does not require them).

If needed later, migrate to a workspace split:
- `crates/ptcl-core`
- `crates/ptcl-demo-raylib`

## Milestones
1. Define core public API boundary.
2. Move demo-specific archetypes (`Explosion`, `Smoke`, textures) out of core modules.
3. Remove spawning side effects from rendering paths.
4. Rebuild demo as a pure consumer of core API.
5. Add documentation examples showing "minimal usage" and "advanced usage."

## Acceptance Criteria
- Core crate can compile and run tests without loading demo assets.
- Demo crate/app compiles using only public core APIs.
- No demo archetype types are required by core internals.
- Update/simulation behavior is independent from render FPS.

## Risks
- API churn while extracting layers.
- Hidden coupling between rendering code and spawn/update behavior.

## Mitigations
- Keep a compatibility layer during transition.
- Refactor in small steps with frequent compile/test checkpoints.

