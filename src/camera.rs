use rayon::prelude::*;
use rand::prelude::*;

use crate::color::Color;
use crate::{color, v3d_zero};

use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3d::Vector3D;

#[derive(serde::Deserialize)]
pub struct Camera {
    pub look_from: Vector3D,
    pub look_at: Vector3D,
    pub vup: Vector3D,

    /// Vertical field-of-view in degrees
    pub vfov: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn render(&self, image_width: u16, samples_per_pixel: u16, objects: &Vec<Sphere>) -> Vec<Color> {

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

        img.par_chunks_mut(image_width as usize).enumerate().for_each(
            |(h, line)|  {
                self.render_line((image_height as usize - h) as u16, line, image_width, image_height, samples_per_pixel, &horizontal, &vertical, &lower_left_corner, objects)
            }
        );

        img
    }

    fn render_line(
        &self,
        h: u16,
        line: &mut [Vector3D],
        image_width: u16,
        image_height: u16,
        samples_per_pixel: u16,
        horizontal: &Vector3D,
        vertical: &Vector3D,
        lower_left_corner: &Vector3D,
        objects: &Vec<Sphere>,
    ) {
        let mut rng = rand::thread_rng();

        for w in 0..image_width {
            let mut color = v3d_zero!();

            for _ in 0..samples_per_pixel {
                let h_fraction = (h as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;
                let w_fraction = (w as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;

                let r = Ray {
                    origin: self.look_from,
                    direction: lower_left_corner + horizontal * w_fraction + vertical * h_fraction - self.look_from,
                };

                color += Camera::ray_color(&r, objects);
            }

            line[w as usize] = color / samples_per_pixel as f32;
        }
    }

    fn ray_color(ray: &Ray, objects: &Vec<Sphere>) -> Color {
        use crate::hittable::Hittable;

        for sphere in objects {
            if let Some(hit) = sphere.hit(ray, 0.0, f32::INFINITY) {
                return color!(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0) * 0.5;
            }
        }

        // blue sky background
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        color!(1.0, 1.0, 1.0) * (1.0 - t) + color!(0.5, 0.7, 1.0) * t
    }
}
