#[derive(Debug)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector {
            x,
            y,
            z
        }
    }

    pub fn zero() -> Self {
        Vector::new(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn scale(&self, mag: f32) -> Self {
        Vector {
            x: self.x * mag,
            y: self.y * mag,
            z: self.z * mag,
        }
    }

    pub fn normalise(&self) -> Vector {
        let mag = self.magnitude();
        if mag < f32::EPSILON {
            Self::zero()
        } else {
            self.scale(1.0 / mag)
        }
    }

    pub fn dot(&self, rhs: &Vector) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x &&
            self.y == other.y &&
            self.z == other.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        let vec = Vector::new(0.0, 0.0, 0.0);
        assert_eq!(vec.magnitude(), 0.0);

        let vec = Vector::new(1.0, 1.0, 1.0);
        assert_eq!(vec.magnitude(), 1.7320508076);

        let vec = Vector::new(3.0, 4.0, 5.0);
        assert_eq!(vec.magnitude(), 7.0710678119);
    }

    #[test]
    fn test_normalise() {
        let vec = Vector::new(0.0, 0.0, 0.0);
        assert_eq!(vec.normalise(), Vector::new(0.0, 0.0, 0.0));

        let vec = Vector::new(1.0, 1.0, 1.0);
        assert_eq!(vec.normalise(), Vector::new(0.5773502692, 0.5773502692, 0.5773502692));

        let vec = Vector::new(3.0, 4.0, 5.0);
        assert_eq!(vec.normalise(), Vector::new(0.42426407, 0.56568545, 0.7071068));
    }
}
