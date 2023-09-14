use macroquad::prelude::*;
use macroquad::window::miniquad::*;

pub struct BallMaterial {
    material: Material,
    ball_count: usize,
}

impl BallMaterial {
    pub fn new(ball_count: usize) -> Self {
        Self {
            ball_count,
            material: create_material(ball_count),
        }
    }

    pub fn update_ball_count(&mut self, ball_count: usize) {
        if self.ball_count == ball_count {
            return;
        }
        self.material = create_material(ball_count);
        self.ball_count = ball_count;
    }

    pub fn apply(&self, aspect: f32) {
        gl_use_material(&self.material);
        self.material
            .set_uniform("ballCount", self.ball_count as i32);
        self.material.set_uniform("aspect", aspect);
    }
}

fn create_material(ball_count: usize) -> Material {
    let fragment = format!(
        r#"
#version 100
#define BALL_COUNT {}
{}"#,
        ball_count,
        include_str!("fragment.glsl")
    );
    load_material(
        ShaderSource::Glsl {
            vertex: include_str!("vertex.glsl"),
            fragment: &fragment,
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
