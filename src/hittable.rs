use crate::{ray::Ray, vec3d::Vector3D};

pub struct HitRecord {
    pub point: Vector3D,
    pub normal: Vector3D,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn set_normal_face(&mut self, ray: &Ray, outward_normal: &Vector3D) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
