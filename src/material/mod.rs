mod dielectric;
mod lamertian;
mod metal;

pub use dielectric::Dielectric;
pub use lamertian::Lambertian;
pub use metal::Metal;

use std::ops::Range;

use crate::core::{Color, HitRecord, Ray};

pub trait Material: Send + Sync {
    fn scatter(&self, rnd: &mut dyn FnMut(Range<f64>) -> f64, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult>;
}

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Color,
}

#[non_exhaustive]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum MaterialConfig {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material for MaterialConfig {
    #[inline]
    fn scatter(&self, rnd: &mut dyn FnMut(Range<f64>) -> f64, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        match &self {
            MaterialConfig::Lambertian(m) => m.scatter(rnd, ray, hit),
            MaterialConfig::Metal(m) => m.scatter(rnd, ray, hit),
            MaterialConfig::Dielectric(m) => m.scatter(rnd, ray, hit),
        }
    }
}
