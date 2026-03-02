# ECS Baseline Before Packed Rewrite - 2026-03-02

## Context

This captures the last benchmark run before removing `hecs` and switching to a packed particle backend.

## Command

```bash
cargo bench --bench sim_bench
```

## Results

- `step_steady_10k`: `[35.531 us, 35.626 us, 35.709 us]`
- `step_steady_50k`: `[176.01 us, 176.50 us, 177.02 us]`
- `burst_100k_lifecycle`: `[3.3424 ms, 3.3597 ms, 3.3761 ms]`
- `spawn_50k_single`: `[3.0473 ms, 3.0534 ms, 3.0602 ms]`
- `spawn_50k_batch`: `[2.8606 ms, 2.8725 ms, 2.8862 ms]`

## Notes

- These are Criterion confidence intervals from the benchmark output.
- This file is the comparison anchor for the packed-backend rewrite.
