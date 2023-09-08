use std::ops::{Neg, Range};

use auto_ops::impl_op_ex;

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
    |a: &Vec3d, b: f64| -> Vec3d {
        Vec3d {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
);

impl_op_ex!(
    *#[inline]
    |b: f64, a: &Vec3d| -> Vec3d {
        Vec3d {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }
);

impl_op_ex!(*= #[inline] |a: &mut Vec3d, b: f64| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
});

impl_op_ex!(/ #[inline] |a: &Vec3d, b: f64| -> Vec3d { Vec3d {
    x: a.x / b,
    y: a.y / b,
    z: a.z / b,
}});

impl_op_ex!(/ #[inline] |a: f64, b: Vec3d| -> Vec3d { Vec3d {
    x: a / b.x,
    y: a / b.y,
    z: a / b.z,
}});

impl_op_ex!(/= #[inline] |a: &mut Vec3d, b: f64| {
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

impl Neg for &Vec3d {
    type Output = Vec3d;

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
    #[inline]
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3d {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Return true if the vector is close to zero in all dimensions.
    #[inline]
    pub fn is_near_zero(&self) -> bool {
        let threshold = 1e-8;
        (self.x.abs() < threshold) && (self.y.abs() < threshold) && (self.z.abs() < threshold)
    }

    #[inline]
    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn random(mut rng: impl rand::Rng) -> Self {
        v3d!(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range(mut rng: impl rand::Rng, min: f64, max: f64) -> Self {
        v3d!(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max)
        )
    }

    /// Generate a vector to a point on the unit sphere
    ///
    /// https://math.stackexchange.com/questions/1585975/how-to-generate-random-points-on-a-sphere
    /// https://mathworld.wolfram.com/SpherePointPicking.html
    pub fn random_in_unit_circle(rng: &mut impl rand::Rng) -> Self {
        let x: f64 = rng.gen_range(-1.0..1.0);
        let y: f64 = rng.gen_range(-1.0..1.0);

        let factor = 1.0 / (x * x + y * y).sqrt();
        v3d!(x * factor, y * factor, 0.0)
    }

    /// Generate a vector to a point on the unit circle
    ///
    /// https://math.stackexchange.com/questions/1585975/how-to-generate-random-points-on-a-sphere
    /// https://mathworld.wolfram.com/SpherePointPicking.html
    pub fn random_unit_vector(rng: &mut impl rand::Rng) -> Self {
        let x: f64 = rng.gen_range(-1.0..1.0);
        let y: f64 = rng.gen_range(-1.0..1.0);
        let z: f64 = rng.gen_range(-1.0..1.0);

        let factor = 1.0 / (x * x + y * y + z * z).sqrt();
        v3d!(x * factor, y * factor, z * factor)
    }

    /// Generate a vector to a point on the unit circle
    ///
    /// https://math.stackexchange.com/questions/1585975/how-to-generate-random-points-on-a-sphere
    /// https://mathworld.wolfram.com/SpherePointPicking.html
    pub fn random_unit_vector_rng_fn(rng: &mut dyn FnMut(Range<f64>) -> f64) -> Self {
        let x: f64 = rng(-1.0..1.0);
        let y: f64 = rng(-1.0..1.0);
        let z: f64 = rng(-1.0..1.0);

        let factor = 1.0 / (x * x + y * y + z * z).sqrt();
        v3d!(x * factor, y * factor, z * factor)
    }

    pub fn random_in_hemisphere(rng: &mut impl rand::Rng, normal: &Vec3d) -> Self {
        let in_unit_sphere = Vec3d::random_unit_vector(rng);
        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;
    use rand::{rngs::SmallRng, Rng, SeedableRng};

    #[test]
    fn test_random_in_unit_sphere() {
        assert_relative_eq!(
            1.0,
            Vec3d::random_unit_vector(&mut rand::thread_rng()).length(),
            epsilon = 2.0 * f64::EPSILON
        );
    }

    #[test]
    fn test_random_in_unit_circle() {
        let mut rng = SmallRng::from_entropy();

        assert_relative_eq!(
            1.0,
            Vec3d::random_in_unit_circle(&mut rng).length(),
            epsilon = 2.0 * f64::EPSILON
        );
    }

    #[test]
    fn test_random_unit_vector_rng_fn() {
        let mut rng = SmallRng::from_entropy();

        let mut rng_func = |range| rng.gen_range(range);

        assert_relative_eq!(
            1.0,
            Vec3d::random_unit_vector_rng_fn(&mut rng_func).length(),
            epsilon = 2.0 * f64::EPSILON
        );
    }
}
