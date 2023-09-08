use std::{borrow::Cow, sync::OnceLock};

use crate::{
    acceleration::Aabb,
    hittable::{HitRecord, Hittable},
    material::MaterialConfig,
    ray::Ray,
    v3d,
    vec3d::Vec3d,
};

impl Sphere {
    pub fn new(origin: Vec3d, radius: f64, material: MaterialConfig) -> Self {
        Sphere {
            origin,
            radius,
            material,
            bounding_box: OnceLock::new(),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Sphere {
    pub origin: Vec3d,
    pub radius: f64,
    pub material: MaterialConfig,

    #[serde(skip)]
    bounding_box: OnceLock<Aabb>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        if root < t_min || t_max <= root {
            root = (-half_b + sqrtd) / a;
            if root <= t_min || t_max <= root {
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

    fn bounding_box(&self) -> Cow<crate::acceleration::Aabb> {
        Cow::Borrowed(self.bounding_box.get_or_init(|| {
            let rvec = v3d!(self.radius.abs(), self.radius.abs(), self.radius.abs());
            Aabb::from_points(&(self.origin - rvec), &(self.origin + rvec))
        }))
    }
}
