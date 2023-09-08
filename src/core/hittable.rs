use std::borrow::Cow;

use crate::{acceleration::Aabb, core::Ray, core::Vec3d, material::MaterialConfig};

pub struct HitRecord<'mat> {
    pub point: Vec3d,
    pub normal: Vec3d,
    pub t: f64,
    pub front_face: bool,
    pub material: &'mat MaterialConfig,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Cow<Aabb>;
}

impl<'mat> HitRecord<'mat> {
    pub fn set_normal_face(&mut self, ray: &Ray, outward_normal: &Vec3d) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
