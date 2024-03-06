use glam::Vec2;
use rand::Rng;
use raylib::prelude::*;

use crate::{
    particle_system::ParticleSystem,
    particles::{get_sample_region, ParticleType},
};

pub const FRAMES_PER_SECOND: u32 = 60;

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,
    pub particle_system: ParticleSystem<ParticleType>,
    pub particle_effects_texture: Texture2D,
}

impl State {
    pub fn new(rl: &mut RaylibHandle, rlt: &mut RaylibThread) -> Self {
        // LOAD PARTICLE_EFFECTS_TEXTURE
        let texture_error = "Error loading special effects texture";
        let path = "assets/particle_effects.png";
        let particle_effects_texture = rl.load_texture(rlt, path).expect(texture_error);

        Self {
            running: true,
            time_since_last_update: 0.0,
            particle_system: ParticleSystem::new(),
            particle_effects_texture,
        }
    }
}

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    let mut rng = rand::thread_rng();
    if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) {
        let mouse_pos = rl.get_mouse_position();

        let size = Vec2::new(1280.0, 720.0);
        let center = size / 2.0;

        // make some spline particles
        let a = mouse_pos;

        // explosions
        for _ in 0..1000 {
            let particle_type = ParticleType::Explosion;
            let counter = rng.gen_range(50..100);
            let pos = Vec2::new(mouse_pos.x, mouse_pos.y);
            let max_size = 42.0;
            let size = rng.gen_range((max_size - max_size / 4.0)..max_size);
            let size = Vec2::new(size, size);
            let particle = state
                .particle_system
                .new_particle(particle_type, counter, pos, size);

            let offset = 75.0;

            let b = Vec2::new(
                a.x + rng.gen_range(-offset..offset),
                a.y + rng.gen_range(-offset..offset),
            );
            let c = center;

            state.particle_system.add_spline(
                particle,
                Vec2::new(a.x, a.y),
                Vec2::new(b.x, b.y),
                Vec2::new(c.x, c.y),
                1.0,
            );
            state.particle_system.add_alpha(particle, 0.0);
            state.particle_system.add_alpha_velocity(particle, 0.005);
            state
                .particle_system
                .add_spline_velocity(particle, rng.gen_range(0.01..0.02));
            state
                .particle_system
                .add_spline_acceleration(particle, rng.gen_range(-0.0005..0.000));

            state
                .particle_system
                .add_size_velocity(particle, rng.gen_range(-0.5..0.0));
        }

        for _ in 0..500 {
            let particle_type = ParticleType::Smoke;
            let counter = rng.gen_range(50..100);
            let pos = Vec2::new(mouse_pos.x, mouse_pos.y);
            let max_size = 16.0;
            let size = rng.gen_range(8.0..max_size);
            let size = Vec2::new(size, size);
            let particle = state
                .particle_system
                .new_particle(particle_type, counter, pos, size);

            let offset = 100.0;

            let b = Vec2::new(
                center.x + rng.gen_range(-offset..offset),
                center.y + rng.gen_range(-offset..offset),
            );
            let c = center;

            state.particle_system.add_spline(
                particle,
                Vec2::new(a.x, a.y),
                Vec2::new(b.x, b.y),
                Vec2::new(c.x, c.y),
                0.1,
            );
            state.particle_system.add_alpha(particle, 0.05);
            state.particle_system.add_alpha_velocity(particle, -0.0008);
            state
                .particle_system
                .add_spline_velocity(particle, rng.gen_range(0.01..0.02));
            state
                .particle_system
                .add_spline_acceleration(particle, rng.gen_range(-0.0005..0.000));

            state
                .particle_system
                .add_size_velocity(particle, rng.gen_range(0.0..2.0));
            state
                .particle_system
                .add_velocity(particle, Vec2::new(0.0, -10.0));
            state
                .particle_system
                .add_acceleration(particle, Vec2::new(0.0, 0.01));
        }

        for _ in 0..100 {
            let particle_type = ParticleType::Explosion;
            let counter = rng.gen_range(8..32);
            let pos = Vec2::new(mouse_pos.x, mouse_pos.y);
            let max_size = 16.0;
            let size = rng.gen_range(1.0..max_size);
            let size = Vec2::new(size, size);
            let particle = state
                .particle_system
                .new_particle(particle_type, counter, pos, size);

            // add random velocity
            let mag = 1.0;
            let vel = Vec2::new(rng.gen_range(-mag..mag), rng.gen_range(-mag..mag));

            state.particle_system.add_velocity(particle, vel);
            state
                .particle_system
                .add_acceleration(particle, Vec2::new(0.0, 0.2));
        }
    }
}

pub fn step(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {
    state.particle_system.step();
}

pub fn draw(state: &mut State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    // d.draw_text("Low Res Sketch!", 12, 12, 12, Color::WHITE);
    let mouse_pos = d.get_mouse_position();
    d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 6.0, Color::GREEN);

    let angle = d.get_time() as f32 * 4.0;

    let mut center = Vec2::new(d.get_screen_width() as f32, d.get_screen_height() as f32) / 2.0;
    center.y += center.y / 2.0;
    let offset = center / 8.0;

    for i in 0..3 {
        let rot = glam::Mat2::from_angle(angle + i as f32 * 90.0);
        let rect_pos_rotated = rot * offset + center;

        let size =
            (((d.get_time() as f32 + i as f32 * 1.0) * 2.0).sin() + 1.0) / 2.0 * offset.y + 4.0;
        // d.draw_rectangle(
        //     rect_pos_rotated.x as i32,
        //     rect_pos_rotated.y as i32,
        //     size as i32,
        //     size as i32,
        //     Color::RED,
        // );

        // spawn 10 particles at the rect center
        let rect_center = rect_pos_rotated + size / 2.0;

        let mut rng = rand::thread_rng();

        for _ in 0..8 {
            let particle_type = ParticleType::Explosion;
            let counter = rng.gen_range(8..24);
            let pos = rect_center;
            let max_size = size / 2.0;
            let size = rng.gen_range(1.0..max_size);
            let size = Vec2::new(size, size);
            let particle = state
                .particle_system
                .new_particle(particle_type, counter, pos, size);

            // add random velocity
            let mag = 0.1;
            let vel = Vec2::new(rng.gen_range(-mag..mag), rng.gen_range(-mag..mag));

            state.particle_system.add_velocity(particle, vel);
            state
                .particle_system
                .add_acceleration(particle, Vec2::new(0.0, 0.1));

            state.particle_system.add_alpha_velocity(particle, -0.05);
        }

        // smoke
        for _ in 0..4 {
            let particle_type = ParticleType::Smoke;
            let counter = rng.gen_range(60..1000);
            let pos = rect_center;
            let max_size = size / 2.0;
            let size = rng.gen_range(1.0..max_size);
            let size = Vec2::new(size, size);
            let particle = state
                .particle_system
                .new_particle(particle_type, counter, pos, size);

            // add random velocity
            let x_mag = 0.1;
            let y_mag = 0.5;
            let vel = Vec2::new(rng.gen_range(-x_mag..x_mag), rng.gen_range(0.0..y_mag));

            state.particle_system.add_velocity(particle, vel);
            state
                .particle_system
                .add_acceleration(particle, Vec2::new(0.0, -0.1));
            state.particle_system.add_alpha(particle, 0.1);
            state.particle_system.add_alpha_velocity(particle, -0.001);
            state.particle_system.add_size_velocity(particle, 1.0);
            state.particle_system.add_size_acceleration(particle, -0.01);

            // spin
            let spin_mag = 2.0;
            state
                .particle_system
                .add_rotation_velocity(particle, rng.gen_range(-spin_mag..spin_mag));
            // // slow spin
            state
                .particle_system
                .add_rotation_acceleration(particle, -0.01);
        }
    }

    draw_particles(state, d);
}
pub fn draw_particles(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    for (_, (particle_type, counter, pos, size, rot, alpha)) in state
        .particle_system
        .world
        .query::<(
            &ParticleType,
            &crate::particle_system::Counter,
            &crate::particle_system::Position,
            &crate::particle_system::Size,
            &crate::particle_system::Rotation,
            &crate::particle_system::Alpha,
        )>()
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
