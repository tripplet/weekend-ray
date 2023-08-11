pub mod dielectric;
pub mod lamertian;
pub mod metal;

use std::ops::Range;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

use dielectric::Dielectric;
use lamertian::Lambertian;
use metal::Metal;

pub trait Material: Send + Sync {
    fn scatter(&self, rnd: &mut dyn FnMut(Range<f64>) -> f64, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult>;
}

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Color,
}

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
