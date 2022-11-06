use crate::{geometry::Geometry, Ray, ray::Intersection};

#[derive(Debug)]
pub struct Scene {
    geometries: Vec<Box<dyn Geometry>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            geometries: Vec::new(),
        }
    }

    pub fn add_geometry(&mut self, geom: Box<dyn Geometry>) {
        self.geometries.push(geom);
    }

    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let intersections = self.geometries.iter().filter_map(|geom| ray.intersects(&*(*geom)));
        intersections.max_by(|a, b| a.closest_intersection.total_cmp(&b.closest_intersection))
    }
}
