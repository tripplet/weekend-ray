use std::{fs::File, io::{BufWriter, Write}};

mod vec3d;
mod ray;
mod camera;
mod color;
mod sphere;

fn main() {
    println!("Hello, world!");

    let camera = camera::Camera {
        look_from: v3d_zero!(),
        look_at: v3d!(0.0, 0.0, -1.0),

        vfov: 90.0,
        vup: v3d!(0.0, 1.0, 0.0),
        aspect_ratio: 16.0 / 9.0,
    };

    let image_width = 800_u16;
    let img = camera.render(image_width);

    let f = File::create("image.ppm").expect("Unable to create file");
    let mut f = BufWriter::new(f);

    let image_height = (image_width as f32 / camera.aspect_ratio) as u16;

    _ = f.write_fmt(format_args!("P3\n{image_width} {image_height}\n255\n"));

    for pixel in img {
        _ = f.write_fmt(format_args!("{} {} {}\n", (pixel.x*255.0) as u8, (pixel.y*255.0) as u8, (pixel.z*255.0) as u8));
    }
}
