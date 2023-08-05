use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

use camera::Camera;
use clap::Parser;

mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3d;

#[derive(Parser)]
#[command(version)]
struct Config {
    #[arg()]
    input_file: String,

    #[arg()]
    width: u16,

    #[arg(default_value = "100")]
    samples_per_pixel: u16
}

#[derive(serde::Deserialize)]
struct InputFile {
    camera: Camera,
    objects: Vec<sphere::Sphere>,
}

fn main() {
    let cfg = Config::parse(); // Parse arguments

    // Read input file
    let input: InputFile =
        serde_json::from_str(&fs::read_to_string(cfg.input_file).expect("Unable to read input file")).unwrap();
    let camera = input.camera;
    let image_width = cfg.width;

    let now = std::time::Instant::now();
    let img = camera.render(image_width, cfg.samples_per_pixel, &input.objects);
    println!("Rendering took {}\n", humantime::format_duration(now.elapsed()));

    let f = File::create("image.ppm").expect("Unable to create file");
    let mut f = BufWriter::new(f);

    let image_height = (image_width as f32 / camera.aspect_ratio) as u16;

    _ = f.write_fmt(format_args!("P3\n{image_width} {image_height}\n255\n"));

    for pixel in img {
        _ = f.write_fmt(format_args!(
            "{} {} {}\n",
            (pixel.x * 255.0) as u8,
            (pixel.y * 255.0) as u8,
            (pixel.z * 255.0) as u8
        ));
    }
}
