use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
};

use camera::{Camera, CameraConfig};
use clap::{Parser, Subcommand};
use hittable::Hittable;
use material::{Dielectric, Lambertian, Metal};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use ray::Ray;
use vec3d::Vec3d;

use crate::acceleration::BvhNode;

mod acceleration;
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
}

#[derive(Subcommand)]
enum InputFormat {
    /// Generate the book cover scene input data
    Cover {
        #[arg()]
        file_path: String,
    },

    /// Use the file as input data
    File(RenderOptions),
}

#[derive(clap::Args)]
struct RenderOptions {
    #[arg()]
    file_path: String,

    #[arg(long)]
    width: u16,

    #[arg(long)]
    depth: u16,

    /// Don't show any progress or measured times
    #[arg(long)]
    quiet: bool,

    /// Use bounding volume hierarchy as acceleration structure
    #[arg(long)]
    use_bvh: bool,

    #[arg(short, long, default_value = "100")]
    samples_per_pixel: u16,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct InputData {
    camera: CameraConfig,
    objects: Vec<sphere::Sphere>,
}

fn main() {
    let cfg = Config::parse(); // Parse arguments

    let input: InputData = match cfg.command {
        InputFormat::Cover { file_path } => {
            generate_random_cover_scene(&file_path);
            return;
        }
        InputFormat::File(RenderOptions { ref file_path, .. }) => {
            // Read input file
            serde_json::from_str(&fs::read_to_string(file_path).expect("Unable to read input file")).unwrap()
        }
    };

    let cfg = match cfg.command {
        InputFormat::File(r) => r,
        _ => panic!(),
    };

    let start = std::time::Instant::now();

    let image_width = cfg.width;
    let camera = Camera::new(&input.camera, image_width, cfg.samples_per_pixel, cfg.depth);

    let img = if cfg.use_bvh {
        let bvh = BvhNode::build(&input.objects.iter().collect::<Vec<_>>());
        camera.render(&bvh, cfg.quiet)
    } else {
        camera.render(&input.objects, cfg.quiet)
    };

    if !cfg.quiet {
        println!("Rendering took {}", humantime::format_duration(start.elapsed()));
    }

    let image_height = (image_width as f64 / camera.cfg.aspect_ratio) as usize;

    // write_ppm("image.ppm", image_width as usize, image_height, &img);
    write_png("image.png", image_width as usize, image_height, &img);
}

fn generate_random_cover_scene(file_path: &str) {
    let mut world = vec![];

    let ground_material = Lambertian {
        albedo: color!(0.5, 0.5, 0.5),
    };

    world.push(sphere::Sphere::new(
        v3d!(0.0, -1000.0, 0.0),
        1000.0,
        material::MaterialConfig::Lambertian(ground_material),
    ));

    let mut rng = SmallRng::from_entropy();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let mut center = v3d!(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>()
            );

            // Find the point where the spheres hit the ground
            // to put them directly on the ground (no hovering)
            let intersection = world[0]
                .hit(
                    &Ray {
                        origin: center,
                        direction: world[0].origin - center,
                        time: 0.0,
                    },
                    0.0,
                    f64::INFINITY,
                )
                .unwrap();

            center = intersection.point + intersection.normal * 0.2;

            if (center - v3d!(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse

                    let center2 = center + v3d!(0.0, rng.gen_range(0.0..0.5), 0.0);

                    world.push(sphere::Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        material::MaterialConfig::Lambertian(Lambertian {
                            albedo: color::random(&mut rng) * color::random(&mut rng),
                        }),
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    world.push(sphere::Sphere::new(
                        center,
                        0.2,
                        material::MaterialConfig::Metal(Metal {
                            fuzz: rng.gen_range(0.0..0.5),
                            albedo: color::random_range(&mut rng, 0.5..1.0) * color::random(&mut rng),
                        }),
                    ));
                } else {
                    // glass
                    world.push(sphere::Sphere::new(
                        center,
                        0.2,
                        material::MaterialConfig::Dielectric(Dielectric {
                            index_of_refraction: 1.5,
                        }),
                    ));
                }
            }
        }
    }

    world.push(sphere::Sphere::new(
        v3d!(0.0, 1.0, 0.0),
        1.0,
        material::MaterialConfig::Dielectric(Dielectric {
            index_of_refraction: 1.5,
        }),
    ));

    world.push(sphere::Sphere::new(
        v3d!(-4.0, 1.0, 0.0),
        1.0,
        material::MaterialConfig::Lambertian(Lambertian {
            albedo: color!(0.4, 0.2, 0.1),
        }),
    ));

    world.push(sphere::Sphere::new(
        v3d!(-4.0, 1.0, 0.0),
        1.0,
        material::MaterialConfig::Lambertian(Lambertian {
            albedo: color!(0.4, 0.2, 0.1),
        }),
    ));

    world.push(sphere::Sphere::new(
        v3d!(4.0, 1.0, 0.0),
        1.0,
        material::MaterialConfig::Metal(Metal {
            fuzz: rng.gen_range(0.0..0.5),
            albedo: color!(0.7, 0.6, 0.5),
        }),
    ));

    let cam = CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        vfov: 20.0,
        look_from: v3d!(13.0, 2.0, 3.0),
        look_at: v3d!(0.0, 0.0, 0.0),
        vup: v3d!(0.0, 1.0, 0.0),

        defocus_angle: -0.6,
        focus_dist: 10.0,
    };

    let data = InputData {
        camera: cam,
        objects: world,
    };

    let path = Path::new(file_path);
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);
    serde_json::to_writer(w, &data).unwrap();
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
