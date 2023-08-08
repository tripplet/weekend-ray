use crate::{
    hittable::{HitRecord, Hittable},
    material::MaterialConfig,
    ray::Ray,
    vec3d::Vec3d,
};

#[derive(serde::Deserialize)]
pub struct Sphere {
    pub origin: Vec3d,
    pub radius: f32,
    pub material: MaterialConfig,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.origin;

        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);

        let mut rec = HitRecord {
            t: root,
            normal: (point - self.origin) / self.radius,
            point,
            front_face: false,
            material: &self.material,
        };

        rec.set_normal_face(ray, &rec.normal.clone());

        Some(rec)
    }
}
