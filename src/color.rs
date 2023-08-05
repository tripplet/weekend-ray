use crate::vec3d::Vector3D;

pub type Color = Vector3D;

#[macro_export]
macro_rules! color {
    ($x:expr, $y:expr, $z:expr) => { crate::vec3d::Vector3D { x: $x, y: $y, z: $z } };
}