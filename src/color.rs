use crate::vec3d::Vec3d;

pub type Color = Vec3d;

#[macro_export]
macro_rules! color {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::vec3d::Vec3d { x: $x, y: $y, z: $z }
    };
}
