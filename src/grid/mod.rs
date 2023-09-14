use std::collections::{HashMap, HashSet};

use macroquad::math::{IVec2, Vec2};

use physical_object::Cell;
pub use physical_object::PhysicalObject;

use crate::grid::physical_object::HitResult;

mod physical_object;

const NEIGHBORS_POSITIONS: [IVec2; 9] = [
    IVec2::new(0, 0),
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(1, 0),
    IVec2::new(1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, 0),
    IVec2::new(-1, 1),
    IVec2::new(-1, -1),
];

pub struct Grid {
    grid: HashMap<IVec2, HashSet<HitResult>>,
    resolution: f32,
}

impl Grid {
    pub fn new(resolution: f32) -> Self {
        Self {
            grid: HashMap::new(),
            resolution: resolution * 2.0,
        }
    }

    pub fn update<'a, Item: PhysicalObject + 'a, Container: IntoIterator<Item = &'a Item>>(
        &mut self,
        objects: Container,
    ) {
        for v in self.grid.values_mut() {
            v.clear();
        }
        for object in objects {
            let object = HitResult::new(object);
            let cell = object.cell(self.resolution);
            if let Some(v) = self.grid.get_mut(&cell) {
                v.insert(object);
            } else {
                self.grid.insert(cell, HashSet::from([object]));
            }
        }
    }

    pub fn test(&self, position: Vec2, radius: f32, ignore: Option<usize>) -> Option<HitResult> {
        let cell = position.cell(self.resolution);
        for offset in NEIGHBORS_POSITIONS {
            let cell = cell + offset;
            if let Some(set) = self.grid.get(&cell) {
                if let Some(hit) = set
                    .iter()
                    .find(|hit| Some(hit.id) != ignore && hit.test(position, radius))
                {
                    return Some(hit.clone());
                }
            }
        }
        None
    }
}

#[cfg(debug_assertions)]
impl Grid {
    pub fn debug_draw(&self, aspect: f32) {
        use crate::utils::dev::*;
        use macroquad::prelude::*;

        for cell in self.grid.values() {
            for ball in cell {
                let pos = world_to_screen(ball.position, aspect);
                draw_circle_lines(
                    pos.x,
                    pos.y,
                    world_radius_to_screen(ball.radius),
                    2.0,
                    Color::from_vec(WHITE.to_vec().lerp(BLACK.to_vec(), cell.len() as f32 / 5.0)),
                );
            }
        }
    }
}
