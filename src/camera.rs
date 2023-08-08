use std::ops::Range;

use rand::prelude::*;
use rayon::prelude::*;

use crate::color::Color;
use crate::{color, v3d_zero};

use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3d::Vec3d;

#[derive(serde::Deserialize)]
pub struct Camera {
    pub cfg: CameraConfig,

    pub image_width: u16,
    pub image_height: u16,

    pub viewport_width: f32,
    pub viewport_height: f32,

    pub samples_per_pixel: u16,
    pub depth: u16,

    horizontal: Vec3d,
    vertical: Vec3d,
    lower_left_corner: Vec3d,
}

#[derive(Clone, serde::Deserialize)]
pub struct CameraConfig {
    pub look_from: Vec3d,
    pub look_at: Vec3d,
    pub vup: Vec3d,

    /// Vertical field-of-view in degrees
    pub vfov: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(cfg: &CameraConfig, image_width: u16, samples_per_pixel: u16, depth: u16) -> Self {
        let theta = cfg.vfov.to_radians();
        let viewport_height = (theta / 2.0).tan() * 2.0;
        let viewport_width = cfg.aspect_ratio * viewport_height;

        let w = (cfg.look_from - cfg.look_at).unit_vector();
        let u = cfg.vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        let lower_left_corner = cfg.look_from - horizontal / 2.0 - vertical / 2.0 - w;

        let image_height = (image_width as f32 / cfg.aspect_ratio) as u16;

        Self {
            cfg: cfg.clone(),
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            samples_per_pixel,
            depth,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn render(&self, objects: &Vec<Sphere>) -> Vec<Color> {
        let mut img = vec![v3d_zero!(); self.image_height as usize * self.image_width as usize];

        img.par_chunks_mut(self.image_width as usize).enumerate().for_each_init(
            || {
                // Create a big, expensive to initialize and slower, but unpredictable RNG.
                // This is cached and done only once per thread / rayon job.
                SmallRng::from_entropy()
            },
            |rng, (h, line)| self.render_line((self.image_height as usize - h) as u16, line, objects, rng),
        );

        img
    }

    fn render_line(&self, h: u16, line: &mut [Vec3d], objects: &Vec<Sphere>, mut thread_rng: &mut impl rand::Rng) {
        let scale = 1.0 / self.samples_per_pixel as f32;

        for w in 0..self.image_width {
            let mut color = v3d_zero!();

            for _ in 0..self.samples_per_pixel {
                let h_fraction = (h as f32 + thread_rng.gen::<f32>()) / (self.image_height - 1) as f32;
                let w_fraction = (w as f32 + thread_rng.gen::<f32>()) / (self.image_width - 1) as f32;

                let r = Ray {
                    origin: self.cfg.look_from,
                    direction: self.lower_left_corner + self.horizontal * w_fraction + self.vertical * h_fraction
                        - self.cfg.look_from,
                };

                color += Camera::ray_color(&r, self.depth, &mut thread_rng, objects);
            }

            color.x = (color.x * scale).sqrt();
            color.y = (color.y * scale).sqrt();
            color.z = (color.z * scale).sqrt();

            line[w as usize] = color;
        }
    }

    fn ray_color(ray: &Ray, depth: u16, rng: &mut impl rand::Rng, world: &Vec<Sphere>) -> Color {
        use crate::hittable::Hittable;

        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return color!(0.0, 0.0, 0.0);
        }

        let mut rng_func = |range: Range<f32>| rng.gen_range(range);

        if let Some(hit) = world.hit(ray, 0.0001, f32::INFINITY) {
            if let Some(scatter) = hit.material.scatter(&mut rng_func, ray, &hit) {
                return scatter.attenuation * Camera::ray_color(&scatter.ray, depth - 1, rng, world);
            }

            return color!(0.0, 0.0, 0.0);
            //return color!(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0) * 0.5;
        }

        // blue sky background
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        color!(1.0, 1.0, 1.0) * (1.0 - t) + color!(0.5, 0.7, 1.0) * t
    }
}
