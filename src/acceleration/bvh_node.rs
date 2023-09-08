use std::sync::Arc;

use rand::Rng;

use super::Aabb;
use crate::hittable::Hittable;

/// Bounding volume hierarchy node
#[derive(Clone)]
pub struct BvhNode<'a> {
    left: Arc<Box<dyn Hittable + 'a + Send + Sync>>,
    right: Arc<Box<dyn Hittable + 'a + Send + Sync>>,
    bounding_box: Aabb,
}

impl<'a> BvhNode<'a> {
    /// Build a bounding volume hierarchy from the list of `Hittable` `objects`
    pub fn build<S: Hittable + Clone + 'a + Send + Sync>(objects: &[&S]) -> BvhNode<'a> {
        let mut sorted_objects = Vec::from(objects);

        // Sort by random axis
        let axis = rand::thread_rng().gen_range(0..=2);
        sorted_objects.sort_unstable_by(|a, b| {
            a.bounding_box()
                .axis(axis)
                .start
                .total_cmp(&b.bounding_box().axis(axis).start)
        });

        if sorted_objects.len() == 1 {
            let left = Arc::new(Box::new(sorted_objects[0].clone()) as Box<dyn Hittable + 'a + Send + Sync>);

            BvhNode {
                bounding_box: Aabb::from_aabb(&left.bounding_box(), &left.bounding_box()),
                right: left.clone(),
                left,
            }
        } else {
            let (ll, rr) = sorted_objects.split_at(sorted_objects.len() / 2);

            let left = Arc::new(Box::new(BvhNode::build(ll)) as Box<dyn Hittable + Send + Sync>);
            let right = Arc::new(Box::new(BvhNode::build(rr)) as Box<dyn Hittable + Send + Sync>);

            BvhNode {
                bounding_box: Aabb::from_aabb(&left.bounding_box(), &right.bounding_box()),
                left,
                right,
            }
        }
    }
}

impl<'a> Hittable for BvhNode<'a> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        if !self.bounding_box.hit(ray, t_min..t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = self.right.hit(
            ray,
            t_min,
            if let Some(hit_left) = &hit_left {
                hit_left.t
            } else {
                t_max
            },
        );

        if let Some(r) = hit_right {
            return Some(r);
        } else if let Some(l) = hit_left {
            return Some(l);
        }

        None
    }

    fn bounding_box(&self) -> std::borrow::Cow<Aabb> {
        std::borrow::Cow::Borrowed(&self.bounding_box)
    }
}
