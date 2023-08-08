pub mod lamertian;
pub mod metal;
pub mod dielectric;

use std::ops::Range;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

use lamertian::Lambertian;
use metal::Metal;
use dielectric::Dielectric;

pub trait Material: Send + Sync {
    fn scatter(&self, rnd: &mut dyn FnMut(Range<f32>) -> f32, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult>;
}

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Color,
}

#[derive(Clone, serde::Deserialize)]
pub enum MaterialConfig {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material for MaterialConfig {
    fn scatter(&self, rnd: &mut dyn FnMut(Range<f32>) -> f32, ray: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        match &self {
            MaterialConfig::Lambertian(m) => m.scatter(rnd, ray, hit),
            MaterialConfig::Metal(m) => m.scatter(rnd, ray, hit),
            MaterialConfig::Dielectric(m) => m.scatter(rnd, ray, hit),
        }
    }
}
