use std::ops::Range;

use crate::vec3d::Vec3d;

pub type Color = Vec3d;

#[macro_export]
macro_rules! color {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::vec3d::Vec3d { x: $x, y: $y, z: $z }
    };
}

pub fn random(rng: &mut impl rand::Rng) -> Color {
    color!(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

pub fn random_range(rng: &mut impl rand::Rng, range: Range<f32>) -> Color {
    color!(
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
        rng.gen_range(range)
    )
}
