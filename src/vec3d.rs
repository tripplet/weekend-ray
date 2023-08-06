use std::ops::Neg;

use auto_ops::impl_op_ex;

#[derive(Copy, Clone, serde::Deserialize)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl_op_ex!(+ #[inline] |a: &Vec3d, b: &Vec3d| -> Vec3d { Vec3d {
    x: a.x + b.x,
    y: a.y + b.y,
    z: a.z + b.z,
}});

impl_op_ex!(+= #[inline] |a: &mut Vec3d, b: &Vec3d| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

impl_op_ex!(
    -#[inline]
    |a: &Vec3d, b: &Vec3d| -> Vec3d {
        Vec3d {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }
);

impl_op_ex!(-= #[inline] |a: &mut Vec3d, b: &Vec3d| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

impl_op_ex!(
    *#[inline]
    |a: &Vec3d, b: &Vec3d| -> Vec3d {
        Vec3d {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
        }
    }
);

impl_op_ex!(*= #[inline] |a: &mut Vec3d, b: &Vec3d| {
    a.x *= b.x;
    a.y *= b.y;
    a.z *= b.z;
});

impl_op_ex!(
    *#[inline]
    |a: &Vec3d, b: f32| -> Vec3d {
        Vec3d {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
);

impl_op_ex!(
    *#[inline]
    |b: f32, a: &Vec3d| -> Vec3d {
        Vec3d {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
);

impl_op_ex!(*= #[inline] |a: &mut Vec3d, b: f32| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
});

impl_op_ex!(/ #[inline] |a: &Vec3d, b: f32| -> Vec3d { Vec3d {
    x: a.x / b,
    y: a.y / b,
    z: a.z / b,
}});

impl_op_ex!(/= #[inline] |a: &mut Vec3d, b: f32| {
    a.x /= b;
    a.y /= b;
    a.z /= b;
});

impl Neg for Vec3d {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3d {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Default for Vec3d {
    fn default() -> Self {
        Vec3d { x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[macro_export]
macro_rules! v3d {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::vec3d::Vec3d { x: $x, y: $y, z: $z }
    };
}

#[macro_export]
macro_rules! v3d_zero {
    () => {
        $crate::vec3d::Vec3d::default()
    };
}

impl Vec3d {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3d {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn unit(&self) -> Self {
        self / self.length()
    }

    pub fn random(mut rng: impl rand::Rng) -> Self {
        v3d!(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range(mut rng: impl rand::Rng, min: f32, max: f32) -> Self {
        v3d!(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }

    /// Generate a vector to a point on the unit sphere
    ///
    /// https://math.stackexchange.com/questions/1585975/how-to-generate-random-points-on-a-sphere
    /// https://mathworld.wolfram.com/SpherePointPicking.html
    pub fn random_unit_vector(rng: &mut impl rand::Rng) -> Self {
        let x: f32 = rng.gen_range(-1.0..1.0);
        let y: f32 = rng.gen_range(-1.0..1.0);
        let z: f32 = rng.gen_range(-1.0..1.0);

        let factor = 1.0 / (x*x + y*y + z*z).sqrt();
        v3d!(x * factor, y * factor, z * factor)
    }

    pub fn random_in_hemisphere(rng: &mut impl rand::Rng, normal: &Vec3d) -> Self {
        let in_unit_sphere = Vec3d::random_unit_vector(rng);
        if in_unit_sphere.dot(normal) > 0.0 { // In the same hemisphere as the normal
            in_unit_sphere
        }
        else {
            -in_unit_sphere
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::*;
    use super::*;

    #[test]
    fn test_random_in_unit_sphere() {
        assert_relative_eq!(1.0, Vec3d::random_in_unit_sphere(&mut rand::thread_rng()).length(), epsilon = 2.0 * f32::EPSILON);
    }
}