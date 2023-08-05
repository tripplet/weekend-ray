use crate::{ray::Ray, vec3d::Vector3D};

pub struct Sphere {
    pub origin: Vector3D,
    pub radius: f32,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.origin;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}
