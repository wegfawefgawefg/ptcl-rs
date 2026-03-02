use glam::{IVec2, UVec2};
use raylib::prelude::*;
use raylib::{ffi::SetTraceLogLevel, prelude::TraceLogLevel};

mod demo_particles;
mod demo_scene;

const TIMESTEP: f32 = 1.0 / demo_scene::FRAMES_PER_SECOND as f32;

fn main() {
    let (mut rl, rlt) = raylib::init().title("ptcl-rs demo").build();

    unsafe {
        SetTraceLogLevel(TraceLogLevel::LOG_WARNING as i32);
    }

    let window_dims = UVec2::new(1280, 720);
    let dims = window_dims;
    let fullscreen = false;

    rl.set_window_size(window_dims.x as i32, window_dims.y as i32);
    if fullscreen {
        rl.toggle_fullscreen();
        rl.set_window_size(rl.get_screen_width(), rl.get_screen_height());
    }

    center_window(&mut rl, window_dims);
    let mouse_scale = dims.as_vec2() / window_dims.as_vec2();
    rl.set_mouse_scale(mouse_scale.x, mouse_scale.y);

    let mut state = demo_scene::State::new(&mut rl, &rlt, dims.as_vec2());
    let mut render_texture = rl
        .load_render_texture(&rlt, dims.x, dims.y)
        .unwrap_or_else(|e| {
            println!("Error creating render texture: {}", e);
            std::process::exit(1);
        });

    while state.running && !rl.window_should_close() {
        demo_scene::process_events_and_input(&mut rl, &mut state);

        let dt = rl.get_frame_time();
        state.time_since_last_update += dt;

        // Prevent unbounded catch-up spirals under heavy load.
        let mut sim_steps = 0;
        while state.time_since_last_update > TIMESTEP && sim_steps < 4 {
            state.time_since_last_update -= TIMESTEP;
            demo_scene::step(&mut state, TIMESTEP);
            sim_steps += 1;
        }

        if sim_steps == 4 {
            state.time_since_last_update = 0.0;
        }

        let mut draw_handle = rl.begin_drawing(&rlt);
        {
            let low_res_draw_handle =
                &mut draw_handle.begin_texture_mode(&rlt, &mut render_texture);
            low_res_draw_handle.clear_background(Color::BLACK);
            demo_scene::draw(&mut state, low_res_draw_handle);
        }

        scale_and_blit_render_texture_to_window(
            &mut draw_handle,
            &mut render_texture,
            fullscreen,
            window_dims,
        );
    }
}

fn center_window(rl: &mut raylib::RaylibHandle, window_dims: UVec2) {
    let screen_dims = IVec2::new(rl.get_screen_width(), rl.get_screen_height());
    let screen_center = screen_dims / 2;
    let window_center = window_dims.as_ivec2() / 2;
    let offset = screen_center - window_center;
    rl.set_window_position(offset.x, offset.y);
    rl.set_target_fps(144);
}

fn scale_and_blit_render_texture_to_window(
    draw_handle: &mut RaylibDrawHandle,
    render_texture: &mut RenderTexture2D,
    fullscreen: bool,
    window_dims: UVec2,
) {
    let source_rec = Rectangle::new(
        0.0,
        0.0,
        render_texture.texture.width as f32,
        -render_texture.texture.height as f32,
    );

    let dest_rec = if fullscreen {
        let screen_width = draw_handle.get_screen_width();
        let screen_height = draw_handle.get_screen_height();
        Rectangle::new(0.0, 0.0, screen_width as f32, screen_height as f32)
    } else {
        Rectangle::new(0.0, 0.0, window_dims.x as f32, window_dims.y as f32)
    };

    draw_handle.draw_texture_pro(
        render_texture,
        source_rec,
        dest_rec,
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}
