use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
};

use camera::{Camera, CameraConfig};
use clap::{Parser, Subcommand};
use material::{dielectric::Dielectric, lamertian::Lambertian, metal::Metal};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use vec3d::Vec3d;

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3d;
mod world;

#[derive(Parser)]
#[command(version)]
struct Config {
    #[command(subcommand)]
    command: InputFormat,

    #[arg(long)]
    width: u16,

    #[arg(long)]
    depth: u16,

    #[arg(short, long, default_value = "100")]
    samples_per_pixel: u16,
}

#[derive(Subcommand)]
enum InputFormat {
    Random,
    File {
        #[arg()]
        file_path: String,
    },
}

#[derive(serde::Deserialize)]
struct InputData {
    camera: CameraConfig,
    objects: Vec<sphere::Sphere>,
}

fn main() {
    let cfg = Config::parse(); // Parse arguments

    let input: InputData = match cfg.command {
        InputFormat::Random => generate_random_demo_scene(),
        InputFormat::File { file_path } => {
            serde_json::from_str(&fs::read_to_string(file_path).expect("Unable to read input file")).unwrap()
        }
    };

    // Read input file

    let image_width = cfg.width;

    let now = std::time::Instant::now();

    let camera = Camera::new(&input.camera, image_width, cfg.samples_per_pixel, cfg.depth);

    let img = camera.render(&input.objects);
    println!("Rendering took {}\n", humantime::format_duration(now.elapsed()));

    let image_height = (image_width as f32 / camera.cfg.aspect_ratio) as usize;

    // write_ppm("image.ppm", image_width as usize, image_height, &img);
    write_png("image.png", image_width as usize, image_height, &img);
}

fn generate_random_demo_scene() -> InputData {
    let mut world = vec![];

    let ground_material = Lambertian {
        albedo: color!(0.5, 0.5, 0.5),
    };

    world.push(sphere::Sphere {
        material: material::MaterialConfig::Lambertian(ground_material),
        origin: v3d!(0.0, -1000.0, 0.0),
        radius: 1000.0,
    });

    let mut rng = SmallRng::from_entropy();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = v3d!(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>()
            );

            if (center - v3d!(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    world.push(sphere::Sphere {
                        material: material::MaterialConfig::Lambertian(Lambertian {
                            albedo: color::random(&mut rng) * color::random(&mut rng),
                        }),
                        origin: center,
                        radius: 0.2,
                    });
                } else if choose_mat < 0.95 {
                    // metal
                    world.push(sphere::Sphere {
                        material: material::MaterialConfig::Metal(Metal {
                            fuzz: rng.gen_range(0.0..0.5),
                            albedo: color::random_range(&mut rng, 0.5..1.0) * color::random(&mut rng),
                        }),
                        origin: center,
                        radius: 0.2,
                    });
                } else {
                    // glass
                    world.push(sphere::Sphere {
                        material: material::MaterialConfig::Dielectric(Dielectric {
                            index_of_refraction: 1.5,
                        }),
                        origin: center,
                        radius: 0.2,
                    });
                }
            }
        }
    }

    world.push(sphere::Sphere {
        material: material::MaterialConfig::Dielectric(Dielectric {
            index_of_refraction: 1.5,
        }),
        origin: v3d!(0.0, 1.0, 0.0),
        radius: 1.0,
    });

    world.push(sphere::Sphere {
        material: material::MaterialConfig::Lambertian(Lambertian {
            albedo: color!(0.4, 0.2, 0.1),
        }),
        origin: v3d!(-4.0, 1.0, 0.0),
        radius: 1.0,
    });

    world.push(sphere::Sphere {
        material: material::MaterialConfig::Lambertian(Lambertian {
            albedo: color!(0.4, 0.2, 0.1),
        }),
        origin: v3d!(-4.0, 1.0, 0.0),
        radius: 1.0,
    });

    world.push(sphere::Sphere {
        material: material::MaterialConfig::Metal(Metal {
            fuzz: rng.gen_range(0.0..0.5),
            albedo: color!(0.7, 0.6, 0.5),
        }),
        origin: v3d!(4.0, 1.0, 0.0),
        radius: 1.0,
    });

    let cam = CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        vfov: 20.0,
        look_from: v3d!(13.0, 2.0, 3.0),
        look_at: v3d!(0.0, 0.0, 0.0),
        vup: v3d!(0.0, 1.0, 0.0),

        defocus_angle: -0.6,
        focus_dist: 10.0,
    };

    InputData {
        camera: cam,
        objects: world,
    }
}

fn write_png(file_path: &str, image_width: usize, image_height: usize, pixels: &[Vec3d]) {
    let path = Path::new(file_path);
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, image_width as u32, image_height as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    _ = writer.write_image_data(
        &pixels
            .iter()
            .map(|pixel| {
                [
                    (pixel.x.clamp(0.0, 0.999) * 256.0) as u8,
                    (pixel.y.clamp(0.0, 0.999) * 256.0) as u8,
                    (pixel.z.clamp(0.0, 0.999) * 256.0) as u8,
                ]
            })
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
    );

    _ = writer.finish();
}

#[allow(dead_code)]
fn write_ppm(file_path: &str, image_width: usize, image_height: usize, pixels: &[Vec3d]) {
    use std::io::Write;

    let f = File::create(file_path).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    _ = f.write_fmt(format_args!("P3\n{image_width} {image_height}\n255\n"));

    for pixel in pixels {
        _ = f.write_fmt(format_args!(
            "{} {} {}\n",
            (pixel.x * 255.0) as u8,
            (pixel.y * 255.0) as u8,
            (pixel.z * 255.0) as u8
        ));
    }
}
