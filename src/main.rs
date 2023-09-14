use macroquad::prelude::*;
use macroquad::window::miniquad::*;

use crate::world::World;

mod ball;
mod world;

#[macroquad::main("Metaballs")]
async fn main() {
    request_new_screen_size(1280.0, 720.0);

    let mat = load_material(
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
    .unwrap();

    let mut ball_count = 3;
    let mut world = World::new(ball_count);
    let mut fps = 0.0f32;

    loop {
        let aspect = screen_width() / screen_height();
        let dt = get_frame_time();

        if is_key_pressed(KeyCode::Space) {
            world.restart();
        }
        if is_key_pressed(KeyCode::Up) {
            ball_count += 1;
            world.change_count(ball_count);
        }
        if is_key_pressed(KeyCode::Down) {
            ball_count = (ball_count - 1).max(1);
            world.change_count(ball_count);
        }

        world.update(dt, aspect);

        let tex = world.make_texture();

        gl_use_material(&mat);
        mat.set_uniform("aspect", aspect);
        mat.set_uniform("ballCount", world.len() as i32);

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

        fps = fps * 0.99 + dt * 0.01;
        draw_text(
            &format!("FPS: {:.1} | BALLS: {}", 1.0 / fps, ball_count),
            10.0,
            20.0,
            24.0,
            WHITE,
        );
        draw_text(
            "PRESS SPACE TO RELOAD. UP TO INCREASE BALL AMOUNT. DOWN TO REDUCE BALL AMOUNT",
            10.0,
            screen_height() - 10.0,
            24.0,
            WHITE,
        );

        next_frame().await
    }
}
