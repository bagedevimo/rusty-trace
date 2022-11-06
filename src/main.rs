use std::fs::File;

use rusty_trace::sample_scene;
use rusty_trace::Image;

fn main() -> std::io::Result<()> {
    let scene = sample_scene();
    let mut image = Image::new(100, 100);

    let mut file = File::create("image.ppm")?;

    image.run(&scene);
    image.write_to_ppm(&mut file)?;

    Ok(())
}
