use std::collections::BTreeSet;
use std::mem;

use macroquad::math::Vec2;
use macroquad::texture::{FilterMode, Texture2D};

use crate::ball::{Ball, MAX_RADIUS};
use crate::grid::Grid;

pub struct World {
    last_id: usize,
    balls: BTreeSet<Ball>,
    temp: BTreeSet<Ball>,
    grid: Grid,
    pub speed: f32,
    pub aspect: f32,
}

impl World {
    pub fn new(ball_count: usize) -> Self {
        let mut world = Self {
            last_id: ball_count,
            balls: BTreeSet::new(),
            temp: BTreeSet::new(),
            grid: Grid::new(MAX_RADIUS),
            speed: 1.0,
            aspect: 1.0,
        };
        world.restart(ball_count);
        world
    }

    pub fn update(&mut self, dt: f32) {
        self.temp.clear();
        self.grid.update(&self.balls);
        self.temp.extend(
            self.balls
                .iter()
                .filter(|b| b.alive)
                .map(|b| b.update(&self.grid, dt * self.speed, self.aspect)),
        );
        mem::swap(&mut self.balls, &mut self.temp);
    }

    pub fn make_texture(&self) -> Texture2D {
        let mut bytes: Vec<u8> = vec![];
        for ball in &self.balls {
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
        for id in 0..ball_count {
            self.balls.insert(Ball::new(id));
        }
        self.last_id = ball_count;
    }

    pub fn add_ball(&mut self, position: Vec2) {
        let pos = position * Vec2::new(self.aspect, 1.0);
        let mut ball = Ball::new(self.last_id);
        self.last_id += 1;
        ball.pos = pos;
        self.balls.insert(ball);
    }

    pub fn remove_ball(&mut self, position: Vec2) {
        let pos = position * Vec2::new(self.aspect, 1.0);
        if let Some(hit) = self.grid.test(pos, 0.0, None) {
            self.balls.remove(&Ball::only_id(hit.id));
        }
    }
}

#[cfg(debug_assertions)]
impl World {
    pub fn debug_draw(&self) {
        self.grid.debug_draw(self.aspect);
    }
}
