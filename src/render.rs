use macroquad::prelude::*;

use crate::ball_material::BallMaterial;
use crate::world::World;

const MAX_RENDER_SIZE: f32 = 768.0;

pub struct Render {
    material: BallMaterial,
    old_size: Vec2,
    small_sdf: RenderTarget,
}

impl Render {
    pub fn new(ball_count: usize) -> Self {
        let size = vec2(screen_width(), screen_height());
        Self {
            material: BallMaterial::new(ball_count),
            old_size: size,
            small_sdf: render_target_scaled(size),
        }
    }

    pub fn render(&mut self, world: &World) {
        if world.len() == 0 {
            clear_background(BLACK);
            return;
        }

        // Update screen size
        let size = vec2(screen_width(), screen_height());
        if (size - self.old_size).length_squared() > 0.1 {
            self.change_size(size);
        }

        // Render
        self.render_sdf(world);
        self.render_post();
    }

    fn render_post(&self) {
        self.material.apply_post(self.small_sdf.texture.size());
        draw_texture_ex(
            &self.small_sdf.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.old_size),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }

    fn render_sdf(&mut self, world: &World) {
        set_camera(&Camera2D {
            render_target: Some(self.small_sdf.clone()),
            ..Camera2D::from_display_rect(Rect::new(
                0.0,
                0.0,
                self.small_sdf.texture.width(),
                self.small_sdf.texture.height(),
            ))
        });

        let tex = world.make_texture();

        self.material.update_ball_count(world.len());
        self.material.apply_sdf(world.aspect);

        draw_texture_ex(
            &tex,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.small_sdf.texture.size()),
                source: None,
                flip_y: true,
                ..Default::default()
            },
        );

        gl_use_default_material();
        set_default_camera();
    }

    fn change_size(&mut self, new_size: Vec2) {
        self.old_size = new_size;
        self.small_sdf.delete();
        self.small_sdf = render_target_scaled(new_size);
    }
}

fn render_target_scaled(size: Vec2) -> RenderTarget {
    let internal_size = size.clamp_length_max(MAX_RENDER_SIZE);
    render_target(internal_size.x as u32, internal_size.y as u32)
}
