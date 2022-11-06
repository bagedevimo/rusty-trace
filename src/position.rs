use std::ops::Sub;

use crate::Vector;

#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Position {
            x,
            y,
            z,
        }
    }

    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }
}

impl Sub<&Position> for &Position {
    type Output = Vector;

    fn sub(self, rhs: &Position) -> Vector {
        Vector::new(
            self.x - rhs.x(),
            self.y - rhs.y(),
            self.z - rhs.z(),
        )
    }
}
