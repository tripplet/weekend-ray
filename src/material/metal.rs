use std::ops::Range;

use crate::{color::Color, ray::Ray, vec3d::Vec3d};

use super::{Material, ScatterResult};

#[derive(Clone, serde::Deserialize)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        mut rnd: &mut dyn FnMut(Range<f32>) -> f32,
        ray: &crate::ray::Ray,
        hit: &crate::hittable::HitRecord,
    ) -> Option<ScatterResult> {
        let reflected = reflect(&ray.direction.unit_vector(), &hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: reflected + self.fuzz * Vec3d::random_unit_vector2(&mut rnd),
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
