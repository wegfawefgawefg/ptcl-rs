# ptcl-rs
some particle system particle templates, part of a family of ports

## Build

- Library only (renderer-agnostic): `cargo check`
- Raylib demo: `cargo run --features demo-raylib`

## Notes

- Core particle simulation lives in `src/core` and does not depend on `raylib`.
- The demo binary is feature-gated behind `demo-raylib`.
