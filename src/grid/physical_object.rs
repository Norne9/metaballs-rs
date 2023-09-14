use std::hash::{Hash, Hasher};

use macroquad::math::{IVec2, Vec2};

pub trait PhysicalObject {
    fn id(&self) -> usize;
    fn radius(&self) -> f32;
    fn position(&self) -> Vec2;
}

pub trait Cell {
    fn cell(&self, size: f32) -> IVec2;
}

#[derive(Copy, Clone)]
pub struct HitResult {
    pub id: usize,
    pub radius: f32,
    pub position: Vec2,
}

impl HitResult {
    pub fn new<T: PhysicalObject>(object: &T) -> Self {
        Self {
            id: object.id(),
            radius: object.radius(),
            position: object.position(),
        }
    }

    pub fn test(&self, position: Vec2, radius: f32) -> bool {
        let delta = self.position - position;
        let dist2 = delta.dot(delta);
        let rad2 = self.radius + radius;
        let rad2 = rad2 * rad2;
        dist2 < rad2
    }
}

impl Cell for HitResult {
    fn cell(&self, size: f32) -> IVec2 {
        self.position.cell(size)
    }
}

impl Cell for Vec2 {
    fn cell(&self, size: f32) -> IVec2 {
        let pos = *self / Vec2::new(size, size);
        IVec2::new(pos.x as i32, pos.y as i32)
    }
}

impl Hash for HitResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq<Self> for HitResult {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for HitResult {}
