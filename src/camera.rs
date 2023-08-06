use std::ops::Range;

use rayon::prelude::*;

use crate::color::Color;
use crate::{color, v3d_zero};

use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3d::Vec3d;

#[derive(serde::Deserialize)]
pub struct Camera {
    pub look_from: Vec3d,
    pub look_at: Vec3d,
    pub vup: Vec3d,

    /// Vertical field-of-view in degrees
    pub vfov: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn render(&self, image_width: u16, samples_per_pixel: u16, depth: i16, objects: &Vec<Sphere>) -> Vec<Color> {
        let theta = self.vfov.to_radians();
        let viewport_height = (theta / 2.0).tan() * 2.0;
        let viewport_width = self.aspect_ratio * viewport_height;

        let w = (self.look_from - self.look_at).unit();
        let u = self.vup.cross(&w).unit();
        let v = w.cross(&u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        let lower_left_corner = self.look_from - horizontal / 2.0 - vertical / 2.0 - w;

        let image_height = (image_width as f32 / self.aspect_ratio) as u16;

        let mut img = vec![v3d_zero!(); image_height as usize * image_width as usize];

        img.par_chunks_mut(image_width as usize)
            .enumerate()
            .for_each(|(h, line)| {
                self.render_line(
                    (image_height as usize - h) as u16,
                    line,
                    image_width,
                    image_height,
                    depth,
                    samples_per_pixel,
                    &horizontal,
                    &vertical,
                    &lower_left_corner,
                    objects,
                )
            });

        img
    }

    fn render_line(
        &self,
        h: u16,
        line: &mut [Vec3d],
        image_width: u16,
        image_height: u16,
        depth: i16,
        samples_per_pixel: u16,
        horizontal: &Vec3d,
        vertical: &Vec3d,
        lower_left_corner: &Vec3d,
        objects: &Vec<Sphere>,
    ) {
        use rand::prelude::*;

        // Create a big, expensive to initialize and slower, but unpredictable RNG.
        // This is cached and done only once per thread.
        let mut thread_rng = SmallRng::from_entropy();

        let scale = 1.0 / samples_per_pixel as f32;

        for w in 0..image_width {
            let mut color = v3d_zero!();

            for _ in 0..samples_per_pixel {
                let h_fraction = (h as f32 + thread_rng.gen::<f32>()) / (image_height - 1) as f32;
                let w_fraction = (w as f32 + thread_rng.gen::<f32>()) / (image_width - 1) as f32;

                let r = Ray {
                    origin: self.look_from,
                    direction: lower_left_corner + horizontal * w_fraction + vertical * h_fraction - self.look_from,
                };

                color += Camera::ray_color(&r, depth, &mut thread_rng, objects);
            }

            color.x = (color.x * scale).sqrt();
            color.y = (color.y * scale).sqrt();
            color.z = (color.z * scale).sqrt();

            line[w as usize] = color;
        }
    }

    fn ray_color(ray: &Ray, depth: i16, rng: &mut impl rand::Rng, world: &Vec<Sphere>) -> Color {
        use crate::hittable::Hittable;

        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return color!(0.0, 0.0, 0.0);
        }

        let mut rng_func = |range: Range<f32>| {
            rng.gen_range(range)
        };

        for sphere in world {
            if let Some(hit) = sphere.hit(ray, 0.0001, f32::INFINITY) {
                if let Some(scatter) = sphere.material.scatter(&mut rng_func, ray, &hit) {
                    return scatter.0 * Camera::ray_color(&scatter.1, depth-1, rng, world);
                }

                return color!(0.0, 0.0, 0.0);
                //return color!(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0) * 0.5;
            }
        }

        // blue sky background
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        color!(1.0, 1.0, 1.0) * (1.0 - t) + color!(0.5, 0.7, 1.0) * t
    }
}
