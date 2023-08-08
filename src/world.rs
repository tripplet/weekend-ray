use crate::{hittable::Hittable, sphere::Sphere};

impl Hittable for Vec<Sphere> {
    fn hit<'a>(&'a self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<crate::hittable::HitRecord<'a>> {
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
}
