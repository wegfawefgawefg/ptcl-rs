use glam::Vec2;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use raylib::prelude::*;

use ptcl_rs::core::{
    Acceleration, Alpha, AlphaVelocity, Counter, DrawLayer, ParticleSystem, Position, Rotation,
    RotationAcceleration, RotationVelocity, Size, SizeAcceleration, SizeVelocity, Spline,
    SplineAcceleration, SplineVelocity, Velocity,
};

use crate::demo_particles::{get_sample_region, ParticleType};

pub const FRAMES_PER_SECOND: u32 = 60;

type ExplosionSplineBundle = (
    ParticleType,
    Counter,
    Position,
    Size,
    Rotation,
    DrawLayer,
    Alpha,
    AlphaVelocity,
    Spline,
    SplineVelocity,
    SplineAcceleration,
    SizeVelocity,
);

type SmokeSplineBundle = (
    ParticleType,
    Counter,
    Position,
    Size,
    Rotation,
    DrawLayer,
    Alpha,
    AlphaVelocity,
    Velocity,
    Acceleration,
    SizeVelocity,
    Spline,
    SplineVelocity,
    SplineAcceleration,
);

type ExplosionBallisticBundle = (
    ParticleType,
    Counter,
    Position,
    Size,
    Rotation,
    DrawLayer,
    Alpha,
    Velocity,
    Acceleration,
);

type EmitterSmokeBundle = (
    ParticleType,
    Counter,
    Position,
    Size,
    Rotation,
    DrawLayer,
    Alpha,
    AlphaVelocity,
    Velocity,
    Acceleration,
    SizeVelocity,
    SizeAcceleration,
    RotationVelocity,
    RotationAcceleration,
);

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,
    pub sim_time: f32,
    pub sim_dims: Vec2,
    pub particle_system: ParticleSystem<ParticleType>,
    pub particle_effects_texture: Texture2D,
    rng: SmallRng,
    click_explosion_spline_batch: Vec<ExplosionSplineBundle>,
    click_smoke_spline_batch: Vec<SmokeSplineBundle>,
    click_explosion_ballistic_batch: Vec<ExplosionBallisticBundle>,
    emitter_explosion_batch: Vec<ExplosionBallisticBundle>,
    emitter_smoke_batch: Vec<EmitterSmokeBundle>,
}

impl State {
    pub fn new(rl: &mut RaylibHandle, rlt: &RaylibThread, sim_dims: Vec2) -> Self {
        let texture_error = "Error loading special effects texture";
        let path = "assets/particle_effects.png";
        let particle_effects_texture = rl.load_texture(rlt, path).expect(texture_error);

        let mut particle_system = ParticleSystem::new();
        particle_system.reserve_particles(120_000);
        particle_system.reserve_bundle::<ExplosionSplineBundle>(120_000);
        particle_system.reserve_bundle::<SmokeSplineBundle>(120_000);
        particle_system.reserve_bundle::<ExplosionBallisticBundle>(120_000);
        particle_system.reserve_bundle::<EmitterSmokeBundle>(120_000);

        Self {
            running: true,
            time_since_last_update: 0.0,
            sim_time: 0.0,
            sim_dims,
            particle_system,
            particle_effects_texture,
            rng: SmallRng::from_os_rng(),
            click_explosion_spline_batch: Vec::with_capacity(1_000),
            click_smoke_spline_batch: Vec::with_capacity(500),
            click_explosion_ballistic_batch: Vec::with_capacity(100),
            emitter_explosion_batch: Vec::with_capacity(24),
            emitter_smoke_batch: Vec::with_capacity(12),
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
    for (particle_type, counter, pos, size, rot, alpha) in state
        .particle_system
        .world
        .query::<(&ParticleType, &Counter, &Position, &Size, &Rotation, &Alpha)>()
        .iter()
    {
        let sample_region = get_sample_region(*particle_type, counter.counter);
        let color = Color::new(255, 255, 255, (alpha.alpha * 255.0) as u8);
        d.draw_texture_pro(
            &state.particle_effects_texture,
            Rectangle::new(
                sample_region.pos.x as f32,
                sample_region.pos.y as f32,
                sample_region.size.x as f32,
                sample_region.size.y as f32,
            ),
            Rectangle::new(pos.pos.x, pos.pos.y, size.size.x, size.size.y),
            Vector2::new(size.size.x / 2.0, size.size.y / 2.0),
            rot.rot,
            color,
        );
    }
}

fn spawn_click_burst(state: &mut State, mouse_pos: Vector2) {
    let a = Vec2::new(mouse_pos.x, mouse_pos.y);
    let center = state.sim_dims / 2.0;

    state.click_explosion_spline_batch.clear();
    state.click_smoke_spline_batch.clear();
    state.click_explosion_ballistic_batch.clear();

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

        state.click_explosion_spline_batch.push((
            ParticleType::Explosion,
            Counter { counter },
            Position { pos: a },
            Size { size },
            Rotation { rot: 0.0 },
            DrawLayer { draw_layer: 0 },
            Alpha { alpha: 0.0 },
            AlphaVelocity { alpha_vel: 0.005 },
            Spline {
                t: 0.0,
                strength: 1.0,
                point_1: a,
                point_2: b,
                point_3: center,
            },
            SplineVelocity {
                tvel: state.rng.random_range(0.01..0.02),
            },
            SplineAcceleration {
                tacc: state.rng.random_range(-0.0005..0.000),
            },
            SizeVelocity {
                size_vel: state.rng.random_range(-0.5..0.0),
            },
        ));
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

        state.click_smoke_spline_batch.push((
            ParticleType::Smoke,
            Counter { counter },
            Position { pos: a },
            Size { size },
            Rotation { rot: 0.0 },
            DrawLayer { draw_layer: 0 },
            Alpha { alpha: 0.05 },
            AlphaVelocity { alpha_vel: -0.0008 },
            Velocity {
                vel: Vec2::new(0.0, -10.0),
            },
            Acceleration {
                acc: Vec2::new(0.0, 0.01),
            },
            SizeVelocity {
                size_vel: state.rng.random_range(0.0..2.0),
            },
            Spline {
                t: 0.0,
                strength: 0.1,
                point_1: a,
                point_2: b,
                point_3: center,
            },
            SplineVelocity {
                tvel: state.rng.random_range(0.01..0.02),
            },
            SplineAcceleration {
                tacc: state.rng.random_range(-0.0005..0.000),
            },
        ));
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

        state.click_explosion_ballistic_batch.push((
            ParticleType::Explosion,
            Counter { counter },
            Position { pos: a },
            Size { size },
            Rotation { rot: 0.0 },
            DrawLayer { draw_layer: 0 },
            Alpha { alpha: 1.0 },
            Velocity { vel },
            Acceleration {
                acc: Vec2::new(0.0, 0.2),
            },
        ));
    }

    flush_batch(
        &mut state.particle_system,
        &mut state.click_explosion_spline_batch,
    );
    flush_batch(
        &mut state.particle_system,
        &mut state.click_smoke_spline_batch,
    );
    flush_batch(
        &mut state.particle_system,
        &mut state.click_explosion_ballistic_batch,
    );
}

fn spawn_rotating_emitters(state: &mut State) {
    let angle = state.sim_time * 4.0;

    let mut center = state.sim_dims / 2.0;
    center.y += center.y / 2.0;
    let offset = center / 8.0;

    state.emitter_explosion_batch.clear();
    state.emitter_smoke_batch.clear();

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

            state.emitter_explosion_batch.push((
                ParticleType::Explosion,
                Counter { counter },
                Position { pos: rect_center },
                Size { size },
                Rotation { rot: 0.0 },
                DrawLayer { draw_layer: 0 },
                Alpha { alpha: 1.0 },
                Velocity { vel },
                Acceleration {
                    acc: Vec2::new(0.0, 0.1),
                },
            ));
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
            state.emitter_smoke_batch.push((
                ParticleType::Smoke,
                Counter { counter },
                Position { pos: rect_center },
                Size { size },
                Rotation { rot: 0.0 },
                DrawLayer { draw_layer: 0 },
                Alpha { alpha: 0.1 },
                AlphaVelocity { alpha_vel: -0.001 },
                Velocity { vel },
                Acceleration {
                    acc: Vec2::new(0.0, -0.1),
                },
                SizeVelocity { size_vel: 1.0 },
                SizeAcceleration { size_acc: -0.01 },
                RotationVelocity {
                    rot_vel: state.rng.random_range(-spin_mag..spin_mag),
                },
                RotationAcceleration { rot_acc: -0.01 },
            ));
        }
    }

    flush_batch(
        &mut state.particle_system,
        &mut state.emitter_explosion_batch,
    );
    flush_batch(&mut state.particle_system, &mut state.emitter_smoke_batch);
}

fn flush_batch<B>(particle_system: &mut ParticleSystem<ParticleType>, batch: &mut Vec<B>)
where
    B: hecs::Bundle + 'static,
{
    if batch.is_empty() {
        return;
    }

    particle_system.spawn_batch(batch.drain(..));
}
