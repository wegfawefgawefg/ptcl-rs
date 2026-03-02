use glam::Vec2;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use raylib::prelude::*;

use ptcl_rs::core::{ParticleSpawn, ParticleSystem, SplineState};

use crate::demo_particles::{get_sample_region, ParticleType};

pub const FRAMES_PER_SECOND: u32 = 60;

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,
    pub sim_time: f32,
    pub sim_dims: Vec2,
    pub particle_system: ParticleSystem<ParticleType>,
    pub particle_effects_texture: Texture2D,
    rng: SmallRng,
    click_batch: Vec<ParticleSpawn<ParticleType>>,
    emitter_batch: Vec<ParticleSpawn<ParticleType>>,
}

impl State {
    pub fn new(rl: &mut RaylibHandle, rlt: &RaylibThread, sim_dims: Vec2) -> Self {
        let texture_error = "Error loading special effects texture";
        let path = "assets/particle_effects.png";
        let particle_effects_texture = rl.load_texture(rlt, path).expect(texture_error);

        let mut particle_system = ParticleSystem::new();
        particle_system.reserve_particles(120_000);

        Self {
            running: true,
            time_since_last_update: 0.0,
            sim_time: 0.0,
            sim_dims,
            particle_system,
            particle_effects_texture,
            rng: SmallRng::from_os_rng(),
            click_batch: Vec::with_capacity(1_600),
            emitter_batch: Vec::with_capacity(64),
        }
    }
}

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
        spawn_click_burst(state, rl.get_mouse_position());
    }
}

pub fn step(state: &mut State, dt: f32) {
    state.sim_time += dt;
    spawn_rotating_emitters(state);
    state.particle_system.step();
}

pub fn draw(state: &mut State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let mouse_pos = d.get_mouse_position();
    d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 6.0, Color::GREEN);
    draw_particles(state, d);
}

pub fn draw_particles(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    for particle in &state.particle_system.particles {
        let sample_region = get_sample_region(particle.particle_type, particle.counter);
        let color = Color::new(255, 255, 255, (particle.alpha * 255.0) as u8);
        d.draw_texture_pro(
            &state.particle_effects_texture,
            Rectangle::new(
                sample_region.pos.x as f32,
                sample_region.pos.y as f32,
                sample_region.size.x as f32,
                sample_region.size.y as f32,
            ),
            Rectangle::new(
                particle.pos.x,
                particle.pos.y,
                particle.size.x,
                particle.size.y,
            ),
            Vector2::new(particle.size.x / 2.0, particle.size.y / 2.0),
            particle.rotation,
            color,
        );
    }
}

fn spawn_click_burst(state: &mut State, mouse_pos: Vector2) {
    let a = Vec2::new(mouse_pos.x, mouse_pos.y);
    let center = state.sim_dims / 2.0;

    state.click_batch.clear();

    for _ in 0..1_000 {
        let counter = state.rng.random_range(50..100);
        let max_size = 42.0;
        let sprite_size = state
            .rng
            .random_range((max_size - max_size / 4.0)..max_size);
        let size = Vec2::new(sprite_size, sprite_size);

        let offset = 75.0;
        let b = Vec2::new(
            a.x + state.rng.random_range(-offset..offset),
            a.y + state.rng.random_range(-offset..offset),
        );

        state.click_batch.push(
            ParticleSpawn::new(ParticleType::Explosion, counter, a, size)
                .with_alpha(0.0)
                .with_alpha_velocity(0.005)
                .with_spline(SplineState {
                    t: 0.0,
                    strength: 1.0,
                    point_1: a,
                    point_2: b,
                    point_3: center,
                })
                .with_spline_velocity(state.rng.random_range(0.01..0.02))
                .with_spline_acceleration(state.rng.random_range(-0.0005..0.000))
                .with_size_velocity(state.rng.random_range(-0.5..0.0)),
        );
    }

    for _ in 0..500 {
        let counter = state.rng.random_range(50..100);
        let max_size = 16.0;
        let sprite_size = state.rng.random_range(8.0..max_size);
        let size = Vec2::new(sprite_size, sprite_size);

        let offset = 100.0;
        let b = Vec2::new(
            center.x + state.rng.random_range(-offset..offset),
            center.y + state.rng.random_range(-offset..offset),
        );

        state.click_batch.push(
            ParticleSpawn::new(ParticleType::Smoke, counter, a, size)
                .with_alpha(0.05)
                .with_alpha_velocity(-0.0008)
                .with_velocity(Vec2::new(0.0, -10.0))
                .with_acceleration(Vec2::new(0.0, 0.01))
                .with_size_velocity(state.rng.random_range(0.0..2.0))
                .with_spline(SplineState {
                    t: 0.0,
                    strength: 0.1,
                    point_1: a,
                    point_2: b,
                    point_3: center,
                })
                .with_spline_velocity(state.rng.random_range(0.01..0.02))
                .with_spline_acceleration(state.rng.random_range(-0.0005..0.000)),
        );
    }

    for _ in 0..100 {
        let counter = state.rng.random_range(8..32);
        let max_size = 16.0;
        let sprite_size = state.rng.random_range(1.0..max_size);
        let size = Vec2::new(sprite_size, sprite_size);

        let mag = 1.0;
        let vel = Vec2::new(
            state.rng.random_range(-mag..mag),
            state.rng.random_range(-mag..mag),
        );

        state.click_batch.push(
            ParticleSpawn::new(ParticleType::Explosion, counter, a, size)
                .with_velocity(vel)
                .with_acceleration(Vec2::new(0.0, 0.2)),
        );
    }

    state
        .particle_system
        .spawn_batch(state.click_batch.drain(..));
}

fn spawn_rotating_emitters(state: &mut State) {
    let angle = state.sim_time * 4.0;

    let mut center = state.sim_dims / 2.0;
    center.y += center.y / 2.0;
    let offset = center / 8.0;

    state.emitter_batch.clear();

    for i in 0..3 {
        let rot = glam::Mat2::from_angle(angle + i as f32 * 90.0);
        let rect_pos_rotated = rot * offset + center;

        let sprite_size = (((state.sim_time + i as f32) * 2.0).sin() + 1.0) / 2.0 * offset.y + 4.0;
        let rect_center = rect_pos_rotated + sprite_size / 2.0;

        for _ in 0..8 {
            let counter = state.rng.random_range(8..24);
            let max_size = sprite_size / 2.0;
            let size_v = state.rng.random_range(1.0..max_size);
            let size = Vec2::new(size_v, size_v);

            let mag = 0.1;
            let vel = Vec2::new(
                state.rng.random_range(-mag..mag),
                state.rng.random_range(-mag..mag),
            );

            state.emitter_batch.push(
                ParticleSpawn::new(ParticleType::Explosion, counter, rect_center, size)
                    .with_alpha_velocity(-0.05)
                    .with_velocity(vel)
                    .with_acceleration(Vec2::new(0.0, 0.1)),
            );
        }

        for _ in 0..4 {
            let counter = state.rng.random_range(60..1000);
            let max_size = sprite_size / 2.0;
            let size_v = state.rng.random_range(1.0..max_size);
            let size = Vec2::new(size_v, size_v);

            let x_mag = 0.1;
            let y_mag = 0.5;
            let vel = Vec2::new(
                state.rng.random_range(-x_mag..x_mag),
                state.rng.random_range(0.0..y_mag),
            );

            let spin_mag = 2.0;
            state.emitter_batch.push(
                ParticleSpawn::new(ParticleType::Smoke, counter, rect_center, size)
                    .with_alpha(0.1)
                    .with_alpha_velocity(-0.001)
                    .with_velocity(vel)
                    .with_acceleration(Vec2::new(0.0, -0.1))
                    .with_size_velocity(1.0)
                    .with_size_acceleration(-0.01)
                    .with_rotation_velocity(state.rng.random_range(-spin_mag..spin_mag))
                    .with_rotation_acceleration(-0.01),
            );
        }
    }

    state
        .particle_system
        .spawn_batch(state.emitter_batch.drain(..));
}
