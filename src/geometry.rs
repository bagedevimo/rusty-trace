use std::fmt::Debug;

use crate::{position::Position, Ray, ray::Intersection};

pub trait Geometry: Debug {
    fn intersected_by(&self, ray: &Ray) -> Option<Intersection>;
}

#[derive(Debug)]
pub struct Sphere {
    center: Position,
    radius: f32,
}


impl Sphere {
    pub fn new(center: Position, radius: f32) -> Self {
        Sphere {
            center,
            radius,
        }
    }
}

impl Geometry for Sphere {
    fn intersected_by(&self, ray: &Ray) -> Option<Intersection> {
        let origin_to_center = &self.center - ray.origin();
        let tca: f32 = origin_to_center.dot(ray.direction());

        if tca < 0.0 {
            return None;
        }

        let d2 = origin_to_center.dot(&origin_to_center) - tca.powi(2);

        if d2 > (self.radius * self.radius) {
            return None;
        }

        let thc = ((self.radius * self.radius) - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;

        let tmin = t0.min(t1);
        let tmax = t0.max(t1);

        if tmin < 0.0 && tmax < 0.0 {
            return None;
        }

        Some(Intersection {
            closest_intersection: tmin,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    use super::*;

    #[test]
    fn test_intersection_sphere() {
        let sphere = Sphere::new(
            Position::new(0.0, 2.0, 0.0),
            1.0
        );

        let ray = Ray::new(
            Position::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );

        let intersection = sphere.intersected_by(&ray);
        assert!(intersection.is_some());

        let ray = Ray::new(
            Position::new(2.0, 2.0, 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );

        let intersection = sphere.intersected_by(&ray);
        assert!(intersection.is_none());

        let ray = Ray::new(
            Position::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 1.0, 1.0).normalise(),
        );

        let intersection = sphere.intersected_by(&ray);
        assert!(intersection.is_none());
    }
}
