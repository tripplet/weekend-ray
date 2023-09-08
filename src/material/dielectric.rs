use crate::color;
use crate::ray::Ray;
use crate::vec3d::Vec3d;

use super::{Material, ScatterResult};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        rnd: &mut dyn FnMut(std::ops::Range<f64>) -> f64,
        ray: &crate::ray::Ray,
        hit: &crate::hittable::HitRecord,
    ) -> Option<ScatterResult> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray.direction.unit_vector();

        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let new_direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rnd(0.0..1.0) {
            reflect(&unit_direction, &hit.normal)
        } else {
            refract(&unit_direction, &hit.normal, refraction_ratio, cos_theta)
        };

        Some(ScatterResult {
            attenuation: color!(1.0, 1.0, 1.0),
            ray: Ray {
                origin: hit.point,
                direction: new_direction,
                time: ray.time,
            },
        })
    }
}

#[inline(always)]
fn reflect(ray_in: &Vec3d, normal: &Vec3d) -> Vec3d {
    ray_in - 2.0 * ray_in.dot(normal) * normal
}

#[inline(always)]
fn refract(uv: &Vec3d, normal: &Vec3d, etai_over_etat: f64, cos_theta: f64) -> Vec3d {
    let ray_out_perp = etai_over_etat * (uv + cos_theta * normal);
    let ray_out_parallel = -(1.0 - ray_out_perp.length_squared()).abs().sqrt() * normal;
    ray_out_perp + ray_out_parallel
}

/// Calculate the reflectance
///
/// Use Schlick's approximation for reflectance.
#[inline(always)]
fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
    let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
