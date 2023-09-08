use std::ops::Range;

use crate::{
    core::Color,
    core::{HitRecord, Ray, Vec3d},
};

use super::{Material, ScatterResult};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, mut rnd: &mut dyn FnMut(Range<f64>) -> f64, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = hit.normal + Vec3d::random_unit_vector_rng_fn(&mut rnd);

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        Some(ScatterResult {
            attenuation: self.albedo,
            ray: Ray {
                origin: hit.point,
                direction: scatter_direction,
                time: ray.time,
            },
        })
    }
}
