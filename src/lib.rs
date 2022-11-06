mod geometry;
mod position;
mod vector;
mod ray;
mod scene;
mod image;

pub use geometry::Sphere;
pub use position::Position;
pub use vector::Vector;
pub use ray::Ray;
pub use scene::Scene;
pub use image::Image;

pub fn sample_scene() -> Scene {
    let mut scene = Scene::new();
    scene.add_geometry(Box::new(Sphere::new(
        Position::new(0.0, 10.0, 0.0),
        2.0,
    )));

    scene
}
