use std::ops::Range;

use crate::{ray::Ray, vec3d::Vec3d};

/// Access aligned bounding box
#[derive(Clone)]
pub struct Aabb {
    x: Range<f64>,
    y: Range<f64>,
    z: Range<f64>,
}

#[inline]
const fn axis_for_vec(vec: &Vec3d, n: usize) -> f64 {
    match n {
        0 => vec.x,
        1 => vec.y,
        2 => vec.z,
        _ => panic!("Axis is not supported"),
    }
}

fn merge(a: &Range<f64>, b: &Range<f64>) -> Range<f64> {
    Range::<f64> {
        start: a.start.min(b.start),
        end: a.end.max(b.end),
    }
}

impl Aabb {
    pub fn from_aabb(box_a: &Aabb, box_b: &Aabb) -> Self {
        Self {
            x: merge(&box_a.x, &box_b.x),
            y: merge(&box_a.y, &box_b.y),
            z: merge(&box_a.z, &box_b.z),
        }
    }

    pub fn from_points(a: &Vec3d, b: &Vec3d) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.
        Self {
            x: a.x.min(b.x)..a.x.max(b.x),
            y: a.y.min(b.y)..a.y.max(b.y),
            z: a.z.min(b.z)..a.z.max(b.z),
        }
    }

    #[inline]
    pub const fn axis(&self, n: usize) -> &Range<f64> {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Axis is not supported"),
        }
    }

    pub fn hit(&self, ray: &Ray, mut interval: Range<f64>) -> bool {
        for axis in 0..3 {
            let inv_d = axis_for_vec(&(1.0 / ray.direction), axis);
            let orig = axis_for_vec(&ray.origin, axis);

            let mut t0 = (self.axis(axis).start - orig) * inv_d;
            let mut t1 = (self.axis(axis).end - orig) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > interval.start {
                interval.start = t0
            };
            if t1 < interval.end {
                interval.end = t1
            };

            if interval.end <= interval.start {
                return false;
            }
        }

        true
    }
}
