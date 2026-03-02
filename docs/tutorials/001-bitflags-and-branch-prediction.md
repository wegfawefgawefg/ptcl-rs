# Tutorial: Bitflags, Branch Prediction, and Grouping Hot Objects

This is a general optimization pattern for game/sim loops, not specific to this library.

## What Problem This Solves

In hot update loops, objects often have optional behaviors:

- some move
- some fade
- some spin
- some do all three

If your update path is random and branch-heavy, CPU branch prediction and instruction flow can suffer.

Bitflags let you encode enabled behaviors compactly, and grouping objects by similar flags helps the predictor see stable patterns.

## Core Idea

1. Store optional behavior as bits in an integer (`u8`, `u16`, `u32`).
2. Use fast paths for common bit combinations (straight-line math).
3. Keep a fallback path for uncommon combinations.
4. Group objects by behavior/archetype so branch outcomes are less random.

## Self-Contained Rust Example

```rust
use std::time::Instant;

const HAS_VELOCITY: u8 = 1 << 0;
const HAS_ACCEL: u8 = 1 << 1;
const HAS_FADE: u8 = 1 << 2;
const HAS_SPIN: u8 = 1 << 3;

const FLAGS_LINEAR: u8 = HAS_VELOCITY | HAS_ACCEL;
const FLAGS_RICH: u8 = HAS_VELOCITY | HAS_ACCEL | HAS_FADE | HAS_SPIN;

#[derive(Clone, Copy)]
struct Obj {
    flags: u8,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,
    alpha: f32,
    alpha_v: f32,
    rot: f32,
    rot_v: f32,
}

fn step_generic(objs: &mut [Obj]) {
    for o in objs {
        let f = o.flags;
        if (f & HAS_VELOCITY) != 0 {
            if (f & HAS_ACCEL) != 0 {
                o.vx += o.ax;
                o.vy += o.ay;
            }
            o.x += o.vx;
            o.y += o.vy;
        }
        if (f & HAS_FADE) != 0 {
            o.alpha = (o.alpha + o.alpha_v).clamp(0.0, 1.0);
        }
        if (f & HAS_SPIN) != 0 {
            o.rot += o.rot_v;
        }
    }
}

fn step_fast_path(objs: &mut [Obj]) {
    for o in objs {
        match o.flags {
            FLAGS_LINEAR => {
                o.vx += o.ax;
                o.vy += o.ay;
                o.x += o.vx;
                o.y += o.vy;
            }
            FLAGS_RICH => {
                o.vx += o.ax;
                o.vy += o.ay;
                o.x += o.vx;
                o.y += o.vy;
                o.alpha = (o.alpha + o.alpha_v).clamp(0.0, 1.0);
                o.rot += o.rot_v;
            }
            _ => {
                // Fallback for unusual combinations.
                let f = o.flags;
                if (f & HAS_VELOCITY) != 0 {
                    if (f & HAS_ACCEL) != 0 {
                        o.vx += o.ax;
                        o.vy += o.ay;
                    }
                    o.x += o.vx;
                    o.y += o.vy;
                }
                if (f & HAS_FADE) != 0 {
                    o.alpha = (o.alpha + o.alpha_v).clamp(0.0, 1.0);
                }
                if (f & HAS_SPIN) != 0 {
                    o.rot += o.rot_v;
                }
            }
        }
    }
}

fn step_grouped(linear: &mut [Obj], rich: &mut [Obj]) {
    // No per-object behavior branches inside each loop.
    for o in linear {
        o.vx += o.ax;
        o.vy += o.ay;
        o.x += o.vx;
        o.y += o.vy;
    }
    for o in rich {
        o.vx += o.ax;
        o.vy += o.ay;
        o.x += o.vx;
        o.y += o.vy;
        o.alpha = (o.alpha + o.alpha_v).clamp(0.0, 1.0);
        o.rot += o.rot_v;
    }
}

fn make_obj(flags: u8, seed: u32) -> Obj {
    let s = seed as f32;
    Obj {
        flags,
        x: s * 0.001,
        y: s * 0.002,
        vx: 0.03,
        vy: -0.02,
        ax: 0.0002,
        ay: 0.0003,
        alpha: 0.8,
        alpha_v: -0.0001,
        rot: 0.0,
        rot_v: 0.01,
    }
}

fn main() {
    let count = 200_000usize;
    let steps = 200usize;

    // Mixed order: worst case for branch predictability.
    let mut mixed = Vec::with_capacity(count);
    for i in 0..count {
        let flags = if i % 2 == 0 { FLAGS_LINEAR } else { FLAGS_RICH };
        mixed.push(make_obj(flags, i as u32));
    }
    // Poor-man shuffle to make branch outcomes less regular.
    for i in 0..mixed.len() {
        let j = (i * 1103515245 + 12345) % mixed.len();
        mixed.swap(i, j);
    }

    let mut mixed_generic = mixed.clone();
    let mut mixed_fast = mixed.clone();

    let mut linear = Vec::new();
    let mut rich = Vec::new();
    for o in mixed {
        if o.flags == FLAGS_LINEAR {
            linear.push(o);
        } else {
            rich.push(o);
        }
    }

    let t0 = Instant::now();
    for _ in 0..steps {
        step_generic(&mut mixed_generic);
    }
    let generic_time = t0.elapsed();

    let t1 = Instant::now();
    for _ in 0..steps {
        step_fast_path(&mut mixed_fast);
    }
    let fast_time = t1.elapsed();

    let t2 = Instant::now();
    for _ in 0..steps {
        step_grouped(&mut linear, &mut rich);
    }
    let grouped_time = t2.elapsed();

    println!("generic  : {:?}", generic_time);
    println!("fastpath : {:?}", fast_time);
    println!("grouped  : {:?}", grouped_time);
}
```

## Why This Can Be Faster

- Bitflags make feature checks cheap.
- Fast paths reduce branch count for common cases.
- Grouped objects make branch outcomes less random.
- Straight-line code is easier for modern CPUs to pipeline.

## Important Clarification

The win is not "branches are always good."

The win is:

- fewer unnecessary operations
- fewer unpredictable branches
- more repeated patterns the branch predictor can learn

## Practical Rules

- Do this only in measured hot loops.
- Keep uncommon behavior in a fallback path.
- Keep external API simple; make complexity internal.
- Re-measure after each change. Reject optimizations that don't hold up.

## How To Apply In Your Own Engine/Sim

1. Identify top 2-3 object behavior combinations.
2. Add bitflags for optional behaviors.
3. Add fast paths for those combinations.
4. Group objects by behavior lane/archetype.
5. Benchmark mixed + grouped workloads before/after.
