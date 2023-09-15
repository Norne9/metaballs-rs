use std::cmp::Ordering;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::grid::{Grid, PhysicalObject};
use crate::utils::PushFloat;

pub const MAX_RADIUS: f32 = 0.15;
pub const BOUNCE_POWER: f32 = 5.0;

#[derive(Copy, Clone, Default)]
pub struct Ball {
    id: usize,
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: f32,
    pub radius: f32,
}

impl Ball {
    pub fn new(id: usize) -> Self {
        let radius = gen_range(0.05f32, MAX_RADIUS);
        Self {
            id,
            pos: vec2(
                gen_range(-1.0 + radius, 1.0 - radius),
                gen_range(-1.0 + radius, 1.0 - radius),
            ),
            vel: vec2(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0)).normalize() * gen_range(0.1, 0.6),
            radius,
            color: gen_range(0.0, 1.0),
        }
    }

    pub fn add_to_image(&self, bytes: &mut Vec<u8>) {
        bytes.push_float(self.pos.x);
        bytes.push_float(self.pos.y);
        bytes.push_float(self.radius);
        bytes.push_float(self.color);
    }

    fn update_position(&mut self, dt: f32) -> &mut Self {
        self.pos += self.vel * dt;
        self
    }

    fn bounce_walls(&mut self, aspect: f32) -> &mut Self {
        if self.pos.x + self.radius > aspect {
            self.vel.x = -self.vel.x.abs();
        }
        if self.pos.x - self.radius < -aspect {
            self.vel.x = self.vel.x.abs();
        }
        if self.pos.y + self.radius > 1.0 {
            self.vel.y = -self.vel.y.abs();
        }
        if self.pos.y - self.radius < -1.0 {
            self.vel.y = self.vel.y.abs();
        }
        self.pos = self.pos.max(vec2(-aspect, -1.0)).min(vec2(aspect, 1.0));
        self
    }

    fn bounce_balls(&mut self, grid: &Grid, dt: f32) -> &mut Self {
        if let Some(other) = grid.test(self.pos, self.radius, Some(self.id)) {
            let delta = self.pos - other.position;
            self.vel = Vec2::lerp(self.vel.normalize(), delta.normalize(), dt * BOUNCE_POWER).normalize()
                * self.vel.length();
        }
        self
    }

    pub fn update(&mut self, grid: &Grid, dt: f32, aspect: f32) {
        self.update_position(dt)
            .bounce_walls(aspect)
            .bounce_balls(grid, dt);
    }
}

impl PhysicalObject for Ball {
    fn id(&self) -> usize {
        self.id
    }

    fn radius(&self) -> f32 {
        self.radius
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
}

impl Eq for Ball {}

impl PartialEq<Self> for Ball {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd<Self> for Ball {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Ball {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}