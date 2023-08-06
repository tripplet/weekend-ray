use std::ops::Range;

use crate::{vec3d::Vec3d, color::Color, ray::Ray};

use super::Material;

#[derive(serde::Deserialize)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, mut rnd: &mut dyn FnMut(Range<f32>) -> f32, _ray: &crate::ray::Ray, hit: &crate::hittable::HitRecord) -> Option<(Color, crate::ray::Ray)> {
        let mut scatter_direction = hit.normal + Vec3d::random_unit_vector2(&mut rnd);

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        Some((self.albedo, Ray { origin: hit.point, direction: scatter_direction }))
    }
}