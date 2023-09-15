use std::collections::BTreeMap;

use macroquad::math::Vec2;
use macroquad::texture::{FilterMode, Texture2D};

use crate::ball::{Ball, MAX_RADIUS};
use crate::grid::{Grid, PhysicalObject};

pub struct World {
    last_id: usize,
    balls: BTreeMap<usize, Ball>,
    grid: Grid,
    start_ball_count: usize,
    pub speed: f32,
    pub aspect: f32,
}

impl World {
    pub fn new(ball_count: usize) -> Self {
        let mut world = Self {
            last_id: ball_count,
            balls: BTreeMap::new(),
            grid: Grid::new(MAX_RADIUS),
            start_ball_count: ball_count,
            speed: 1.0,
            aspect: 1.0,
        };
        world.restart();
        world
    }

    pub fn update(&mut self, dt: f32) {
        self.grid.update(self.balls.values());
        for ball in self.balls.values_mut() {
            ball.update(&self.grid, dt * self.speed, self.aspect);
        }
    }

    pub fn make_texture(&self) -> Texture2D {
        let mut bytes: Vec<u8> = vec![];
        for ball in self.balls.values() {
            ball.add_to_image(&mut bytes);
        }

        let texture = Texture2D::from_rgba8(4, self.balls.len() as u16, &bytes);
        texture.set_filter(FilterMode::Nearest);
        texture
    }

    pub fn len(&self) -> usize {
        self.balls.len()
    }

    pub fn restart(&mut self) {
        self.balls.clear();
        for id in 0..self.start_ball_count {
            self.add_bal(Ball::new(id));
        }
        self.last_id = self.start_ball_count;
    }

    pub fn add_ball(&mut self, position: Vec2) {
        let pos = position * Vec2::new(self.aspect, 1.0);
        let mut ball = Ball::new(self.last_id);
        self.last_id += 1;
        ball.pos = pos;
        self.add_bal(ball);
    }

    pub fn remove_ball(&mut self, position: Vec2) {
        let pos = position * Vec2::new(self.aspect, 1.0);
        if let Some(hit) = self.grid.test(pos, 0.0, None) {
            self.balls.remove(&hit.id);
        }
    }

    fn add_bal(&mut self, ball: Ball) {
        self.balls.insert(ball.id(), ball);
    }
}

impl World {
    pub fn debug_draw(&self) {
        self.grid.debug_draw(self.aspect);
    }
}
