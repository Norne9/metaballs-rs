use macroquad::texture::{FilterMode, Texture2D};

use crate::ball::Ball;

pub struct World {
    balls: Vec<Ball>,
    temp: Vec<Ball>,
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
        }
    }

    pub fn update(&mut self, dt: f32, aspect: f32) {
        self.temp.clear();
        for (i, ball) in self.balls.iter().enumerate() {
            self.temp.push(ball.update(&self.balls, i, dt, aspect));
        }
        self.balls.clear();
        self.balls.extend(&self.temp);
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

    pub fn change_count(&mut self, new_count: usize) {
        while self.balls.len() < new_count {
            self.balls.push(Ball::new());
        }
        while self.balls.len() > new_count {
            self.balls.pop();
        }
    }

    pub fn restart(&mut self) {
        let count = self.balls.len();
        self.balls.clear();
        for _ in 0..count {
            self.balls.push(Ball::new());
        }
    }
}