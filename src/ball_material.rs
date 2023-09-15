use macroquad::prelude::*;

pub struct BallMaterial {
    sdf_material: Material,
    post_material: Material,
    ball_count: usize,
}

impl BallMaterial {
    pub fn new(ball_count: usize) -> Self {
        Self {
            ball_count,
            sdf_material: create_sdf_material(ball_count),
            post_material: create_post_material(),
        }
    }

    pub fn update_ball_count(&mut self, ball_count: usize) {
        if self.ball_count == ball_count {
            return;
        }
        self.sdf_material = create_sdf_material(ball_count);
        self.ball_count = ball_count;
    }

    pub fn apply_sdf(&self, aspect: f32) {
        gl_use_material(&self.sdf_material);
        self.sdf_material.set_uniform("aspect", aspect);
    }

    pub fn apply_post(&self, size: Vec2) {
        gl_use_material(&self.post_material);
        self.post_material.set_uniform("resolution", size);
    }
}

fn create_sdf_material(ball_count: usize) -> Material {
    let fragment = format!(
        r#"
#version 100
#define BALL_COUNT {}
{}"#,
        ball_count,
        include_str!("sdf.glsl")
    );
    load_material(
        ShaderSource::Glsl {
            vertex: include_str!("vertex.glsl"),
            fragment: &fragment,
        },
        MaterialParams {
            uniforms: vec![
                ("aspect".to_string(), UniformType::Float1),
            ],
            pipeline_params: PipelineParams {
                color_blend: None,
                ..Default::default()
            },
            ..Default::default()
        },
    )
        .unwrap()
}

fn create_post_material() -> Material {
    load_material(
        ShaderSource::Glsl {
            vertex: include_str!("vertex.glsl"),
            fragment: include_str!("draw.glsl"),
        },
        MaterialParams {
            uniforms: vec![
                ("resolution".to_string(), UniformType::Float2),
            ],
            pipeline_params: PipelineParams {
                color_blend: None,
                ..Default::default()
            },
            ..Default::default()
        },
    )
        .unwrap()
}