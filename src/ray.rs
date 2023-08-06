use crate::vec3d::Vec3d;

pub struct Ray {
    pub origin: Vec3d,
    pub direction: Vec3d,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3d {
        self.origin + (self.direction * t)
    }
}
