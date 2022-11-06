use std::io::Write;

use crate::{Ray, Position, Vector, Scene};

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<(u8, u8, u8)>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            data: vec!((0, 0, 0); width * height),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> (u8, u8, u8) {
        self.data[self.index(x, y)]
    }

    pub fn put(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let index = self.index(x, y);
        self.data[index] = (r, g, b);

    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        (self.width * y) + x
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn write_to_ppm(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write("P3\n".as_bytes()).expect("");
        file.write(format!("{} {}\n", self.width, self.height).as_bytes())?;
        file.write(format!("{}\n", u8::MAX).as_bytes())?;

        for x in 0..self.width {
            for y in 0..self.height {
                let (r, g, b) = self.get(x, y);
                file.write(format!("{} {} {}\n", r, g, b).as_bytes())?;
            }
        }

        Ok(())
    }

    pub fn run(&mut self, scene: &Scene) {
        for x in 0..self.width {
            for y in 0..self.height {
                let px : f32 = ((x as f32 / self.width as f32) * 2.0) - 1.0;
                let py : f32 = ((y as f32 / self.height as f32) * 2.0) - 1.0;
                let ray = Ray::new(
                    Position::new(0.0, 0.0, 0.0),
                    Vector::new(px, 1.0, py).normalise(),
                );
                let intersection = scene.trace(&ray);

                match intersection {
                    Some(i) => { 
                        println!("i: {:?}", i);
                        self.put(x, y, 255, 255, 255);
                    },
                    None => {}
                };
            }
        }
    }
}
