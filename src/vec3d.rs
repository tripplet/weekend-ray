use auto_ops::impl_op_ex;

#[derive(Copy, Clone)]
pub struct Vector3D{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl_op_ex!(+ #[inline] |a: &Vector3D, b: &Vector3D| -> Vector3D { Vector3D {
    x: a.x + b.x,
    y: a.y + b.y,
    z: a.z + b.z,
}});

impl_op_ex!(+= #[inline] |a: &mut Vector3D, b: &Vector3D| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

impl_op_ex!(- #[inline] |a: &Vector3D, b: &Vector3D| -> Vector3D { Vector3D {
    x: a.x - b.x,
    y: a.y - b.y,
    z: a.z - b.z,
}});

impl_op_ex!(-= #[inline] |a: &mut Vector3D, b: &Vector3D| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

impl_op_ex!(* #[inline] |a: &Vector3D, b: &Vector3D| -> Vector3D { Vector3D {
    x: a.x * b.x,
    y: a.y * b.y,
    z: a.z * b.z,
}});

impl_op_ex!(*= #[inline] |a: &mut Vector3D, b: &Vector3D| {
    a.x *= b.x;
    a.y *= b.y;
    a.z *= b.z;
});

impl_op_ex!(* #[inline] |a: &Vector3D, b: f32| -> Vector3D { Vector3D {
    x: a.x * b,
    y: a.y * b,
    z: a.z * b,
}});

impl_op_ex!(* #[inline] |b: f32, a: &Vector3D| -> Vector3D { Vector3D {
    x: a.x * b,
    y: a.y * b,
    z: a.z * b,
}});

impl_op_ex!(*= #[inline] |a: &mut Vector3D, b: f32| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
});

impl_op_ex!(/ #[inline] |a: &Vector3D, b: f32| -> Vector3D { Vector3D {
    x: a.x / b,
    y: a.y / b,
    z: a.z / b,
}});

impl_op_ex!(/= #[inline] |a: &mut Vector3D, b: f32| {
    a.x /= b;
    a.y /= b;
    a.z /= b;
});

impl Default for Vector3D {
    fn default() -> Self {
        Vector3D { x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[macro_export]
macro_rules! v3d {
    ($x:expr, $y:expr, $z:expr) => { crate::vec3d::Vector3D { x: $x, y: $y, z: $z } };
}

#[macro_export]
macro_rules! v3d_zero {
    () => { crate::vec3d::Vector3D::default() };
}

impl Vector3D {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vector3D {
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
}