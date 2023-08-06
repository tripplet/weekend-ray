use std::ops::Range;

use crate::{color::Color, vec3d::Vec3d, ray::Ray};

use super::material::Material;

#[derive(serde::Deserialize)]
struct Metal {
    albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, _rnd: &mut dyn FnMut(Range<f32>) -> f32, ray: &crate::ray::Ray, hit: &crate::hittable::HitRecord) -> Option<(Color, crate::ray::Ray)> {
        let reflected = reflect(&ray.direction.unit(), &hit.normal);
        let scattered = Ray { origin: hit.point, direction: reflected };

        if scattered.direction.dot(&hit.normal) > 0.0 {
            return Some((self.albedo, scattered));
        }

        None
    }
}

fn reflect(v: &Vec3d, n: &Vec3d) -> Vec3d {
    v - 2.0 * v.dot(n) * n
}
