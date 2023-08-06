use std::ops::Range;

use serde::Deserialize;

use crate::{color::Color, hittable::HitRecord, ray::Ray};
use crate::color;

use super::lamertian::Lambertian;

pub trait Material: Send + Sync {
    fn scatter(&self, rnd: &mut dyn FnMut(Range<f32>) -> f32, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
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


        Ok(Box::new(Lambertian {
            albedo: color!(0.8, 0.8, 0.0),
        }))
    }
}
