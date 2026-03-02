use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use glam::Vec2;
use ptcl_rs::core::{ParticleSpawn, ParticleSystem, ParticleTypeTrait, SplineState};
use std::hint::black_box;

#[derive(Clone, Copy)]
enum BenchType {
    Burst,
    Smoke,
    Spline,
}

impl ParticleTypeTrait for BenchType {}

const STEADY_COUNTER: u32 = 1_000_000_000;

fn seed_steady_system(count: u32) -> ParticleSystem<BenchType> {
    let mut ps = ParticleSystem::new();
    ps.reserve_particles(count);

    for i in 0..count {
        let base = Vec2::new((i % 512) as f32, ((i / 512) % 512) as f32);
        let size = Vec2::splat(6.0 + (i % 8) as f32);

        match i % 3 {
            0 => {
                ps.spawn(
                    ParticleSpawn::new(BenchType::Burst, STEADY_COUNTER, base, size)
                        .with_velocity(Vec2::new(0.05, -0.02))
                        .with_acceleration(Vec2::new(0.0, 0.0005)),
                );
            }
            1 => {
                ps.spawn(
                    ParticleSpawn::new(BenchType::Smoke, STEADY_COUNTER, base, size)
                        .with_alpha(0.8)
                        .with_alpha_velocity(-0.0001)
                        .with_velocity(Vec2::new(0.02, -0.03))
                        .with_acceleration(Vec2::new(0.0, 0.0002))
                        .with_size_velocity(0.02)
                        .with_size_acceleration(-0.0001)
                        .with_rotation_velocity(0.15)
                        .with_rotation_acceleration(-0.0002),
                );
            }
            _ => {
                ps.spawn(
                    ParticleSpawn::new(BenchType::Spline, STEADY_COUNTER, base, size)
                        .with_alpha(0.4)
                        .with_alpha_velocity(0.0002)
                        .with_spline(SplineState {
                            t: 0.0,
                            strength: 0.35,
                            point_1: base,
                            point_2: base + Vec2::new(30.0, -40.0),
                            point_3: base + Vec2::new(80.0, 30.0),
                        })
                        .with_spline_velocity(0.008)
                        .with_spline_acceleration(-0.00001),
                );
            }
        }
    }

    ps
}

fn seed_linear_system(count: u32) -> ParticleSystem<BenchType> {
    let mut ps = ParticleSystem::new();
    ps.reserve_particles(count);

    for i in 0..count {
        let base = Vec2::new((i % 512) as f32, ((i / 512) % 512) as f32);
        ps.spawn(
            ParticleSpawn::new(BenchType::Burst, STEADY_COUNTER, base, Vec2::splat(6.0))
                .with_velocity(Vec2::new(0.05, -0.02))
                .with_acceleration(Vec2::new(0.0, 0.0005)),
        );
    }

    ps
}

fn seed_rich_system(count: u32) -> ParticleSystem<BenchType> {
    let mut ps = ParticleSystem::new();
    ps.reserve_particles(count);

    for i in 0..count {
        let base = Vec2::new((i % 512) as f32, ((i / 512) % 512) as f32);
        let size = Vec2::splat(6.0 + (i % 8) as f32);
        ps.spawn(
            ParticleSpawn::new(BenchType::Smoke, STEADY_COUNTER, base, size)
                .with_alpha(0.8)
                .with_alpha_velocity(-0.0001)
                .with_velocity(Vec2::new(0.02, -0.03))
                .with_acceleration(Vec2::new(0.0, 0.0002))
                .with_size_velocity(0.02)
                .with_size_acceleration(-0.0001)
                .with_rotation_velocity(0.15)
                .with_rotation_acceleration(-0.0002),
        );
    }

    ps
}

fn seed_burst_system() -> ParticleSystem<BenchType> {
    let mut ps = ParticleSystem::new();
    ps.reserve_particles(100_000);

    for i in 0..100_000u32 {
        let pos = Vec2::new((i % 1024) as f32, ((i / 1024) % 1024) as f32);
        ps.spawn(
            ParticleSpawn::new(BenchType::Burst, 3 + (i % 4), pos, Vec2::splat(4.0))
                .with_velocity(Vec2::new(0.1, -0.08))
                .with_acceleration(Vec2::new(0.0, 0.001)),
        );
    }

    ps
}

fn build_burst_spawns(count: u32) -> Vec<ParticleSpawn<BenchType>> {
    let mut spawns = Vec::with_capacity(count as usize);
    for i in 0..count {
        let pos = Vec2::new((i % 1024) as f32, ((i / 1024) % 1024) as f32);
        spawns.push(
            ParticleSpawn::new(BenchType::Burst, 3 + (i % 4), pos, Vec2::splat(4.0))
                .with_velocity(Vec2::new(0.1, -0.08))
                .with_acceleration(Vec2::new(0.0, 0.001)),
        );
    }
    spawns
}

fn bench_step_steady_10k(c: &mut Criterion) {
    c.bench_function("step_steady_10k", |b| {
        let mut ps = seed_steady_system(10_000);
        b.iter(|| {
            ps.step();
            black_box(ps.len());
        });
    });
}

fn bench_step_100(c: &mut Criterion) {
    c.bench_function("step_100", |b| {
        let mut ps = seed_steady_system(100);
        b.iter(|| {
            ps.step();
            black_box(ps.len());
        });
    });
}

fn bench_step_1k(c: &mut Criterion) {
    c.bench_function("step_1k", |b| {
        let mut ps = seed_steady_system(1_000);
        b.iter(|| {
            ps.step();
            black_box(ps.len());
        });
    });
}

fn bench_step_steady_50k(c: &mut Criterion) {
    c.bench_function("step_steady_50k", |b| {
        let mut ps = seed_steady_system(50_000);
        b.iter(|| {
            ps.step();
            black_box(ps.len());
        });
    });
}

fn bench_step_linear_50k(c: &mut Criterion) {
    c.bench_function("step_linear_50k", |b| {
        let mut ps = seed_linear_system(50_000);
        b.iter(|| {
            ps.step();
            black_box(ps.len());
        });
    });
}

fn bench_step_rich_50k(c: &mut Criterion) {
    c.bench_function("step_rich_50k", |b| {
        let mut ps = seed_rich_system(50_000);
        b.iter(|| {
            ps.step();
            black_box(ps.len());
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
                black_box(ps.len());
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_spawn_50k_single(c: &mut Criterion) {
    c.bench_function("spawn_50k_single", |b| {
        b.iter_batched(
            || build_burst_spawns(50_000),
            |spawns| {
                let mut ps = ParticleSystem::<BenchType>::new();
                ps.reserve_particles(spawns.len() as u32);
                for spawn in spawns {
                    ps.spawn(spawn);
                }
                black_box(ps.len());
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_spawn_50k_batch(c: &mut Criterion) {
    c.bench_function("spawn_50k_batch", |b| {
        b.iter_batched(
            || build_burst_spawns(50_000),
            |spawns| {
                let mut ps = ParticleSystem::<BenchType>::new();
                ps.reserve_particles(spawns.len() as u32);
                ps.spawn_ballistic_batch(spawns);
                black_box(ps.len());
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    bench_step_100,
    bench_step_1k,
    bench_step_steady_10k,
    bench_step_steady_50k,
    bench_step_linear_50k,
    bench_step_rich_50k,
    bench_burst_100k_lifecycle,
    bench_spawn_50k_single,
    bench_spawn_50k_batch
);
criterion_main!(benches);
