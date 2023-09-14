use macroquad::prelude::*;
use macroquad::rand::gen_range;

#[derive(Copy, Clone)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub radius: f32,
    pub alive: bool,
}

impl Ball {
    pub fn new() -> Self {
        let radius = gen_range(0.05f32, 0.15f32);
        Self {
            pos: vec2(
                gen_range(-1.0 + radius, 1.0 - radius),
                gen_range(-1.0 + radius, 1.0 - radius),
            ),
            vel: vec2(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0)).normalize() * gen_range(0.3, 0.8),
            radius,
            color: hsv_to_rgb(gen_range(0.0, 360.0), 1.0, 1.0),
            alive: true,
        }
    }

    pub fn add_to_image(&self, bytes: &mut Vec<u8>) {
        let mut bytes = bytes;
        push_float(&mut bytes, self.pos.x);
        push_float(&mut bytes, self.pos.y);
        push_float(&mut bytes, self.radius);
        bytes.push((self.color.r * 255.0) as u8);
        bytes.push((self.color.g * 255.0) as u8);
        bytes.push((self.color.b * 255.0) as u8);
        bytes.push((self.color.a * 255.0) as u8);
    }

    fn update_position(&self, dt: f32) -> Self {
        Self {
            pos: self.pos + self.vel * dt,
            ..*self
        }
    }

    fn bounce_walls(&self, aspect: f32) -> Self {
        let mut vel = self.vel;
        if self.pos.x + self.radius > aspect {
            vel.x = -vel.x.abs();
        }
        if self.pos.x - self.radius < -aspect {
            vel.x = vel.x.abs();
        }
        if self.pos.y + self.radius > 1.0 {
            vel.y = -vel.y.abs();
        }
        if self.pos.y - self.radius < -1.0 {
            vel.y = vel.y.abs();
        }
        Self { vel, ..*self }
    }

    fn bounce_balls(&self, others: &[Ball], my_index: usize) -> Self {
        let mut vel = self.vel;
        for j in 0..others.len() {
            if my_index == j {
                continue;
            }
            let delta = self.pos - others[j].pos;
            if self.check_collision(delta, others[j].radius) {
                vel = delta.normalize() * vel.length();
            }
        }
        Self { vel, ..*self }
    }

    pub fn update(self, others: &[Ball], my_index: usize, dt: f32, aspect: f32) -> Self {
        self.update_position(dt)
            .bounce_walls(aspect)
            .bounce_balls(others, my_index)
    }

    pub fn is_point_inside(&self, point: Vec2, radius: f32) -> bool {
        let delta = self.pos - point;
        self.check_collision(delta, radius)
    }

    fn check_collision(&self, delta: Vec2, radius: f32) -> bool {
        let dist2 = delta.dot(delta);
        let rad2 = self.radius + radius;
        let rad2 = rad2 * rad2;
        dist2 < rad2
    }
}

fn push_float(vec: &mut Vec<u8>, data: f32) {
    let data = data * 20.0;
    vec.push(data.abs() as u8);
    vec.push((data.abs().fract() * 255.0) as u8);
    vec.push(((data.abs().fract() * 255.0).fract() * 255.0) as u8);
    if data < 0.0 {
        vec.push(0u8)
    } else {
        vec.push(255u8)
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color {
        r: r + m,
        g: g + m,
        b: b + m,
        a: 1.0, // Alpha is set to 1.0 for full opacity
    }
}
