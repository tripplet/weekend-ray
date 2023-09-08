use crate::vec3d::Vec3d;

pub struct Ray {
    pub origin: Vec3d,
    pub direction: Vec3d,
    pub time: f64,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3d {
        self.origin + (self.direction * t)
    }
}
