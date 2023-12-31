use std::ops::Range;

use crate::core::{
    Color, Vec3d, {HitRecord, Ray},
};

use super::{Material, ScatterResult};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, mut rnd: &mut dyn FnMut(Range<f64>) -> f64, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let reflected = reflect(&ray.direction.unit_vector(), &hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: reflected + self.fuzz * Vec3d::random_unit_vector_rng_fn(&mut rnd),
            time: ray.time,
        };

        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some(ScatterResult {
                attenuation: self.albedo,
                ray: scattered,
            })
        } else {
            None
        }
    }
}

#[inline(always)]
fn reflect(v: &Vec3d, n: &Vec3d) -> Vec3d {
    v - 2.0 * v.dot(n) * n
}
