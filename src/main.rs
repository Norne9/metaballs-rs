use std::ops::Not;

use macroquad::prelude::*;
use macroquad::window::miniquad::*;

use crate::world::World;

mod ball;
mod world;

#[macroquad::main("Metaballs")]
async fn main() {
    let mat = create_material();
    let mut world = World::new(3);
    let mut fps = 0.0f32;
    let mut show_hud = true;

    loop {
        let dt = get_frame_time();
        world.aspect = screen_width() / screen_height();

        process_input(&mut world, &mut show_hud);

        world.update(dt);
        draw_world(&world, &mat);

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

fn draw_world(world: &World, material: &Material) {
    if world.len() == 0 {
        clear_background(BLACK);
        return;
    }

    let tex = world.make_texture();

    gl_use_material(material);
    material.set_uniform("aspect", world.aspect);
    material.set_uniform("ballCount", world.len() as i32);

    draw_texture_ex(
        &tex,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            source: None,
            ..Default::default()
        },
    );

    gl_use_default_material();
}

fn process_input(world: &mut World, show_hud: &mut bool) {
    if is_key_pressed(KeyCode::LeftShift) {
        *show_hud = show_hud.not();
    }
    if is_key_pressed(KeyCode::Space) {
        world.restart();
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

fn create_material() -> Material {
    load_material(
        ShaderSource::Glsl {
            vertex: include_str!("vertex.glsl"),
            fragment: include_str!("fragment.glsl"),
        },
        MaterialParams {
            uniforms: vec![
                ("aspect".to_string(), UniformType::Float1),
                ("ballCount".to_string(), UniformType::Int1),
            ],
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    )
        .unwrap()
}
