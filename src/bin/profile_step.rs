use glam::Vec2;
use ptcl_rs::core::{ParticleSpawn, ParticleSystem, ParticleTypeTrait, SplineState};
use std::hint::black_box;

#[derive(Clone, Copy)]
enum ProfileType {
    Burst,
    Smoke,
    Spline,
}

impl ParticleTypeTrait for ProfileType {}

const STEADY_COUNTER: u32 = 1_000_000_000;

fn seed_steady_system(count: u32) -> ParticleSystem<ProfileType> {
    let mut ps = ParticleSystem::new();
    ps.reserve_particles(count);

    for i in 0..count {
        let base = Vec2::new((i % 512) as f32, ((i / 512) % 512) as f32);
        let size = Vec2::splat(6.0 + (i % 8) as f32);

        match i % 3 {
            0 => {
                ps.spawn(
                    ParticleSpawn::new(ProfileType::Burst, STEADY_COUNTER, base, size)
                        .with_velocity(Vec2::new(0.05, -0.02))
                        .with_acceleration(Vec2::new(0.0, 0.0005)),
                );
            }
            1 => {
                ps.spawn(
                    ParticleSpawn::new(ProfileType::Smoke, STEADY_COUNTER, base, size)
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
                    ParticleSpawn::new(ProfileType::Spline, STEADY_COUNTER, base, size)
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

fn main() {
    let steps = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(4_000);
    let mut ps = seed_steady_system(50_000);
    for _ in 0..steps {
        ps.step();
        black_box(ps.len());
    }
    println!("{}", ps.len());
}
