use macroquad::prelude::*;

use crate::render::Render;
use crate::ui::Gui;
use crate::world::World;

mod ball;
mod ball_material;
mod grid;
mod render;
mod ui;
mod utils;
mod world;

const START_BALL_COUNT: usize = 5;

#[macroquad::main("Metaballs")]
async fn main() {
    let mut world = World::new(START_BALL_COUNT);
    let mut renderer = Render::new(START_BALL_COUNT);
    let mut gui = Gui::new();

    loop {
        let dt = get_frame_time().min(0.1);

        world.aspect = screen_width() / screen_height();
        world.update(dt);

        renderer.render(&world);

        if gui.show_debug_info() {
            world.debug_draw();
        }

        gui.update(dt);
        gui.draw(&mut world);

        process_input(&mut world, &gui);

        next_frame().await
    }
}

fn process_input(world: &mut World, gui: &Gui) {
    if gui.mouse_in_window() {
        return;
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        let pos = mouse_position_local() * vec2(1.0, -1.0);
        if gui.click_to_add() {
            world.add_ball(pos);
        } else {
            world.remove_ball(pos);
        }
    }
}
