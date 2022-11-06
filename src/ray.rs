use crate::{Position, Vector};
use crate::geometry::Geometry;

pub struct Ray {
    origin: Position,
    direction: Vector,
}

#[derive(Debug)]
pub struct Intersection {
    pub closest_intersection: f32,
}

impl Ray {
    pub fn new(origin: Position, direction: Vector) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn origin(&self) -> &Position { &self.origin }
    pub fn direction(&self) -> &Vector { &self.direction }

    pub fn intersects(&self, geometry: &dyn Geometry) -> Option<Intersection> {
        geometry.intersected_by(self)
    }
}

