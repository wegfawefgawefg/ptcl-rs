use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use glam::Vec2;
use ptcl_rs::core::{
    Acceleration, Alpha, AlphaVelocity, Counter, DrawLayer, ParticleSystem, ParticleTypeTrait,
    Position, Rotation, RotationAcceleration, RotationVelocity, Size, SizeAcceleration,
    SizeVelocity, Spline, SplineAcceleration, SplineVelocity, Velocity,
};
use std::hint::black_box;

#[derive(Clone, Copy)]
enum BenchType {
    Burst,
    Smoke,
    Spline,
}

impl ParticleTypeTrait for BenchType {}

type BurstBundle = (
    BenchType,
    Counter,
    Position,
    Size,
    Rotation,
    DrawLayer,
    Alpha,
    Velocity,
    Acceleration,
);

fn seed_steady_system(count: u32) -> ParticleSystem<BenchType> {
    let mut ps = ParticleSystem::new();
    ps.reserve_particles(count);

    for i in 0..count {
        let base = Vec2::new((i % 512) as f32, ((i / 512) % 512) as f32);
        let size = Vec2::splat(6.0 + (i % 8) as f32);

        match i % 3 {
            0 => {
                ps.world.spawn((
                    BenchType::Burst,
                    Counter { counter: 20_000 },
                    Position { pos: base },
                    Size { size },
                    Rotation { rot: 0.0 },
                    DrawLayer { draw_layer: 0 },
                    Alpha { alpha: 1.0 },
                    Velocity {
                        vel: Vec2::new(0.05, -0.02),
                    },
                    Acceleration {
                        acc: Vec2::new(0.0, 0.0005),
                    },
                ));
            }
            1 => {
                ps.world.spawn((
                    BenchType::Smoke,
                    Counter { counter: 20_000 },
                    Position { pos: base },
                    Size { size },
                    Rotation { rot: 0.0 },
                    DrawLayer { draw_layer: 0 },
                    Alpha { alpha: 0.8 },
                    AlphaVelocity { alpha_vel: -0.0001 },
                    Velocity {
                        vel: Vec2::new(0.02, -0.03),
                    },
                    Acceleration {
                        acc: Vec2::new(0.0, 0.0002),
                    },
                    SizeVelocity { size_vel: 0.02 },
                    SizeAcceleration { size_acc: -0.0001 },
                    RotationVelocity { rot_vel: 0.15 },
                    RotationAcceleration { rot_acc: -0.0002 },
                ));
            }
            _ => {
                ps.world.spawn((
                    BenchType::Spline,
                    Counter { counter: 20_000 },
                    Position { pos: base },
                    Size { size },
                    Rotation { rot: 0.0 },
                    DrawLayer { draw_layer: 0 },
                    Alpha { alpha: 0.4 },
                    AlphaVelocity { alpha_vel: 0.0002 },
                    Spline {
                        t: 0.0,
                        strength: 0.35,
                        point_1: base,
                        point_2: base + Vec2::new(30.0, -40.0),
                        point_3: base + Vec2::new(80.0, 30.0),
                    },
                    SplineVelocity { tvel: 0.008 },
                    SplineAcceleration { tacc: -0.00001 },
                ));
            }
        }
    }

    ps
}

fn seed_burst_system() -> ParticleSystem<BenchType> {
    let mut ps = ParticleSystem::new();
    ps.reserve_particles(100_000);

    for i in 0..100_000u32 {
        let pos = Vec2::new((i % 1024) as f32, ((i / 1024) % 1024) as f32);
        ps.world.spawn((
            BenchType::Burst,
            Counter {
                counter: 3 + (i % 4),
            },
            Position { pos },
            Size {
                size: Vec2::splat(4.0),
            },
            Rotation { rot: 0.0 },
            DrawLayer { draw_layer: 0 },
            Alpha { alpha: 1.0 },
            Velocity {
                vel: Vec2::new(0.1, -0.08),
            },
            Acceleration {
                acc: Vec2::new(0.0, 0.001),
            },
        ));
    }

    ps
}

fn build_burst_bundles(count: u32) -> Vec<BurstBundle> {
    let mut bundles = Vec::with_capacity(count as usize);
    for i in 0..count {
        let pos = Vec2::new((i % 1024) as f32, ((i / 1024) % 1024) as f32);
        bundles.push((
            BenchType::Burst,
            Counter {
                counter: 3 + (i % 4),
            },
            Position { pos },
            Size {
                size: Vec2::splat(4.0),
            },
            Rotation { rot: 0.0 },
            DrawLayer { draw_layer: 0 },
            Alpha { alpha: 1.0 },
            Velocity {
                vel: Vec2::new(0.1, -0.08),
            },
            Acceleration {
                acc: Vec2::new(0.0, 0.001),
            },
        ));
    }
    bundles
}

fn bench_step_steady_10k(c: &mut Criterion) {
    c.bench_function("step_steady_10k", |b| {
        let mut ps = seed_steady_system(10_000);
        b.iter(|| {
            ps.step();
            black_box(ps.world.len());
        });
    });
}

fn bench_step_steady_50k(c: &mut Criterion) {
    c.bench_function("step_steady_50k", |b| {
        let mut ps = seed_steady_system(50_000);
        b.iter(|| {
            ps.step();
            black_box(ps.world.len());
        });
    });
}

fn bench_burst_100k_lifecycle(c: &mut Criterion) {
    c.bench_function("burst_100k_lifecycle", |b| {
        b.iter_batched(
            seed_burst_system,
            |mut ps| {
                for _ in 0..8 {
                    ps.step();
                }
                black_box(ps.world.len());
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_spawn_50k_single(c: &mut Criterion) {
    c.bench_function("spawn_50k_single", |b| {
        b.iter_batched(
            || build_burst_bundles(50_000),
            |bundles| {
                let mut ps = ParticleSystem::<BenchType>::new();
                ps.reserve_bundle::<BurstBundle>(bundles.len() as u32);
                for bundle in bundles {
                    ps.world.spawn(bundle);
                }
                black_box(ps.world.len());
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_spawn_50k_batch(c: &mut Criterion) {
    c.bench_function("spawn_50k_batch", |b| {
        b.iter_batched(
            || build_burst_bundles(50_000),
            |bundles| {
                let mut ps = ParticleSystem::<BenchType>::new();
                ps.reserve_bundle::<BurstBundle>(bundles.len() as u32);
                ps.spawn_batch(bundles);
                black_box(ps.world.len());
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    bench_step_steady_10k,
    bench_step_steady_50k,
    bench_burst_100k_lifecycle,
    bench_spawn_50k_single,
    bench_spawn_50k_batch
);
criterion_main!(benches);
