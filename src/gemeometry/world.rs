use std::borrow::Cow;

use crate::{
    acceleration::Aabb,
    core::{HitRecord, Hittable, Ray},
    gemeometry::Sphere,
};

impl Hittable for Vec<Sphere> {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let mut hit = None;
        let mut closest_so_far = t_max;

        for obj in self {
            if let Some(new_hit) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = new_hit.t;
                hit = Some(new_hit);
            }
        }

        hit
    }

    fn bounding_box(&self) -> Cow<'_, Aabb> {
        todo!()
    }
}
