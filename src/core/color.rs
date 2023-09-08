use std::ops::Range;

use crate::core::Vec3d;

pub type Color = Vec3d;

#[macro_export]
macro_rules! color {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::core::Vec3d { x: $x, y: $y, z: $z }
    };
}

pub fn random(rng: &mut impl rand::Rng) -> Color {
    color!(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
}

pub fn random_range(rng: &mut impl rand::Rng, range: Range<f64>) -> Color {
    color!(
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
        rng.gen_range(range)
    )
}
