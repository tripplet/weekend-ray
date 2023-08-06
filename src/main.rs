use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
};

use camera::Camera;
use clap::Parser;

mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3d;
mod material;

#[derive(Parser)]
#[command(version)]
struct Config {
    #[arg()]
    input_file: String,

    #[arg(long)]
    width: u16,

    #[arg(long)]
    depth: u16,

    #[arg(short, long, default_value = "100")]
    samples_per_pixel: u16,
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
    let img = camera.render(image_width, cfg.samples_per_pixel, cfg.depth as i16, &input.objects);
    println!("Rendering took {}\n", humantime::format_duration(now.elapsed()));

    let image_height = (image_width as f32 / camera.aspect_ratio) as u16;

    // let f = File::create("image.ppm").expect("Unable to create file");
    // let mut f = BufWriter::new(f);


    // _ = f.write_fmt(format_args!("P3\n{image_width} {image_height}\n255\n"));

    // for pixel in &img {
    //     _ = f.write_fmt(format_args!(
    //         "{} {} {}\n",
    //         (pixel.x * 255.0) as u8,
    //         (pixel.y * 255.0) as u8,
    //         (pixel.z * 255.0) as u8
    //     ));
    // }

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, image_width as u32, image_height as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    _ = writer.write_image_data(
        &img.iter().map(|pixel|
            [
                (pixel.x.clamp(0.0, 0.999) * 256.0) as u8,
                (pixel.y.clamp(0.0, 0.999) * 256.0) as u8,
                (pixel.z.clamp(0.0, 0.999) * 256.0) as u8
            ]).collect::<Vec<_>>().into_iter().flatten().collect::<Vec<_>>());

    _ = writer.finish();
}
