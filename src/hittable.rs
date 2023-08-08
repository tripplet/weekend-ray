use crate::{material::MaterialConfig, ray::Ray, vec3d::Vec3d};

pub struct HitRecord<'mat> {
    pub point: Vec3d,
    pub normal: Vec3d,
    pub t: f32,
    pub front_face: bool,
    pub material: &'mat MaterialConfig,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
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
