use std::mem;

use macroquad::math::Vec2;
use macroquad::texture::{FilterMode, Texture2D};

use crate::ball::Ball;

pub struct World {
    balls: Vec<Ball>,
    temp: Vec<Ball>,
    pub speed: f32,
    pub aspect: f32,
}

impl World {
    pub fn new(ball_count: usize) -> Self {
        let mut balls = Vec::with_capacity(ball_count);
        for _ in 0..ball_count {
            balls.push(Ball::new());
        }
        Self {
            balls,
            temp: Vec::with_capacity(ball_count),
            speed: 1.0,
            aspect: 1.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.temp.clear();
        for (i, ball) in self.balls.iter().enumerate() {
            if !ball.alive {
                continue;
            }
            self.temp
                .push(ball.update(&self.balls, i, dt * self.speed, self.aspect));
        }

        mem::swap(&mut self.balls, &mut self.temp);
    }

    pub fn make_texture(&self) -> Texture2D {
        let mut bytes: Vec<u8> = vec![];
        for ball in self.balls.iter() {
            ball.add_to_image(&mut bytes);
        }

        let texture = Texture2D::from_rgba8(4, self.balls.len() as u16, &bytes);
        texture.set_filter(FilterMode::Nearest);
        texture
    }

    pub fn len(&self) -> usize {
        self.balls.len()
    }

    pub fn restart(&mut self, ball_count: usize) {
        self.balls.clear();
        for _ in 0..ball_count {
            self.balls.push(Ball::new());
        }
    }

    pub fn add_ball(&mut self, position: Vec2) {
        let pos = position * Vec2::new(self.aspect, 1.0);
        let mut ball = Ball::new();
        ball.pos = pos;
        self.balls.push(ball);
    }

    pub fn remove_ball(&mut self, position: Vec2) {
        let pos = position * Vec2::new(self.aspect, 1.0);
        for ball in self.balls.iter_mut() {
            if ball.is_point_inside(pos, 0.0) {
                ball.alive = false;
            }
        }
    }
}
