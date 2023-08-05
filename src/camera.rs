use crate::color::Color;
use crate::{color, v3d};

use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3d::Vector3D;

pub struct Camera {
    pub look_from: Vector3D,
    pub look_at: Vector3D,
    pub vup: Vector3D,

    /// Vertical field-of-view in degrees
    pub vfov: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn render(&self, image_width: u16) -> Vec<Color> {
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

        let mut img = Vec::with_capacity(image_height as usize * image_width as usize);

        for h in 0..image_height {
            for w in 0..image_width {
                let h_part = h as f32 / (image_height - 1) as f32;
                let w_part = w as f32 / (image_width - 1) as f32;

                let r = Ray {
                    origin: self.look_from,
                    direction: lower_left_corner + horizontal * w_part + vertical * h_part
                        - self.look_from,
                };

                img.push(Camera::ray_color(&r));
            }
        }

        img
    }

    fn ray_color(ray: &Ray) -> Color {
        let sphere = Sphere {
            origin: v3d!(0.0, 0.0, -1.0),
            radius: 0.5,
        };

        if let Some(t) = sphere.hit(&ray) {
            let normal_vec = (ray.at(t) - v3d!(0.0, 0.0, -1.0)).unit();
            return color!(normal_vec.x + 1.0, normal_vec.y + 1.0, normal_vec.z + 1.0) * 0.5;
        }

        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return color!(1.0, 1.0, 1.0) * (1.0 - t) + color!(0.5, 0.7, 1.0) * t;
    }
}
