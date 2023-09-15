use std::ops::Not;

use macroquad::prelude::*;

use crate::render::Render;
use crate::world::World;

mod ball;
mod ball_material;
mod grid;
mod utils;
mod world;
mod render;

const START_BALL_COUNT: usize = 5;

#[macroquad::main("Metaballs")]
async fn main() {
    let mut world = World::new(START_BALL_COUNT);
    let mut renderer = Render::new(START_BALL_COUNT);
    let mut fps = 0.0f32;
    let mut show_hud = true;

    loop {
        let dt = get_frame_time();
        world.aspect = screen_width() / screen_height();

        process_input(&mut world, &mut show_hud);

        world.update(dt);

        renderer.render(&world);

        #[cfg(debug_assertions)]
        world.debug_draw();

        fps = fps * 0.99 + dt * 0.01;
        if show_hud {
            draw_ui(&world, fps);
        }

        next_frame().await
    }
}

fn draw_ui(world: &World, fps: f32) {
    draw_text(
        &format!(
            "FPS: {:.1} | BALLS: {} | SPEED: {:.2}",
            1.0 / fps,
            world.len(),
            world.speed
        ),
        10.0,
        20.0,
        24.0,
        WHITE,
    );
    draw_text(
        "Press SPACE to reload. LMB/RMB to add/remove balls.",
        10.0,
        screen_height() - 30.0,
        24.0,
        WHITE,
    );
    draw_text(
        "MOUSE WHEEL to change speed. SHIFT to toggle hud.",
        10.0,
        screen_height() - 10.0,
        24.0,
        WHITE,
    );
}

fn process_input(world: &mut World, show_hud: &mut bool) {
    if is_key_pressed(KeyCode::LeftShift) {
        *show_hud = show_hud.not();
    }
    if is_key_pressed(KeyCode::Space) {
        world.restart(START_BALL_COUNT);
    }

    let wheel = mouse_wheel().1;
    world.speed += world.speed * 0.0003 * wheel;

    let pos = mouse_position_local() * vec2(1.0, -1.0);
    if is_mouse_button_pressed(MouseButton::Left) {
        world.add_ball(pos);
    }
    if is_mouse_button_pressed(MouseButton::Right) {
        world.remove_ball(pos);
    }
}
