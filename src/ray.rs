use crate::vec3d::Vector3D;

pub struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector3D {
        self.origin + (self.direction * t)
    }
}