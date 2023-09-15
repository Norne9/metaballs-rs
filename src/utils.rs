pub trait PushFloat {
    fn push_float(&mut self, value: f32);
}

impl PushFloat for Vec<u8> {
    fn push_float(&mut self, value: f32) {
        let value = value * 20.0;
        self.push(value.abs() as u8);
        self.push((value.abs().fract() * 255.0) as u8);
        self.push(((value.abs().fract() * 255.0).fract() * 255.0) as u8);
        if value < 0.0 {
            self.push(0u8)
        } else {
            self.push(255u8)
        }
    }
}

#[cfg(debug_assertions)]
pub mod dev {
    use macroquad::prelude::*;

    pub fn world_to_screen(pos: Vec2, aspect: f32) -> Vec2 {
        (pos * vec2(1.0, -1.0) / Vec2::new(aspect, 1.0) + vec2(1.0, 1.0)) / vec2(2.0, 2.0)
            * vec2(screen_width(), screen_height())
    }

    pub fn world_radius_to_screen(size: f32) -> f32 {
        size * screen_height() / 2.0
    }
}
