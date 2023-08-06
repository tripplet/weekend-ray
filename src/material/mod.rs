pub mod lamertian;
pub mod metal;

use std::ops::Range;

use serde::Deserialize;

use crate::{color::Color, hittable::HitRecord, ray::Ray};
use crate::color;

use lamertian::Lambertian;
use metal::Metal;

pub trait Material: Send + Sync {
    fn scatter(&self, rnd: &mut dyn FnMut(Range<f32>) -> f32, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(serde::Deserialize)]
pub enum MaterialConfig {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material for MaterialConfig {
    fn scatter(&self, rnd: &mut dyn FnMut(Range<f32>) -> f32, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        match &self {
            MaterialConfig::Lambertian(m) => m.scatter(rnd, ray, hit),
            MaterialConfig::Metal(m) => m.scatter(rnd, ray, hit),
        }
    }
}

impl<'de> Deserialize<'de> for Box<dyn Material> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // #[derive(Debug, Deserialize)]
        // struct Inner {
        //     topics: Vec<String>,
        // }

        // let a = deserializer.deserialize_map(HashMap<String, Inner>).unwrap();
        // deserializer.deser

        Ok(Box::new(Lambertian {
            albedo: color!(0.8, 0.8, 0.0),
        }))
    }
}
