use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

type Real = f32;
const EPSILON: Real = 0.0001;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    // oooh, alignment shenannies -WINK-
    _pad: Real,
}

impl Vector3 {
    // Constructors
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self { x, y, z, _pad: 0. }
    }

    pub fn splat(val: Real) -> Self {
        Self::new(val, val, val)
    }

    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn magnitude(&self) -> Real {
        let Self { x, y, z, .. } = self;
        (x * x + y * y + z * z).sqrt()
    }

    pub fn square_magnitude(&self) -> Real {
        let Self { x, y, z, .. } = self;
        x * x + y * y + z * z
    }

    pub fn normalise(&self) -> Option<Self> {
        let magnitude = self.magnitude();
        if magnitude <= 0. {
            return None;
        }

        Some(self * (1. / magnitude))
    }

    pub fn add_scaled(&self, other: Vector3, scalar: Real) -> Vector3 {
        let Vector3 { x, y, z, .. } = other;
        Self::new(
            self.x + x * scalar,
            self.y + y * scalar,
            self.z + z * scalar,
        )
    }

    pub fn component_product(&self, other: Vector3) -> Vector3 {
        let Vector3 { x, y, z, .. } = other;
        Self::new(self.x * x, self.y * y, self.z * z)
    }

    pub fn dot(&self, other: Vector3) -> Real {
        let Vector3 {
            x: ax,
            y: ay,
            z: az,
            ..
        } = self;
        let Vector3 {
            x: bx,
            y: by,
            z: bz,
            ..
        } = other;

        ax * bx + ay * by + az * bz
    }

    pub fn cross(&self, other: Vector3) -> Vector3 {
        let Vector3 {
            x: ax,
            y: ay,
            z: az,
            ..
        } = self;
        let Vector3 {
            x: bx,
            y: by,
            z: bz,
            ..
        } = other;

        Vector3::new(ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }
}

impl Mul<Real> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output {
        let Self { x, y, z, .. } = self;
        Self::new(x * rhs, y * rhs, z * rhs)
    }
}

impl Mul<Real> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Real) -> Self::Output {
        let Vector3 { x, y, z, .. } = self;
        Vector3::new(x * rhs, y * rhs, z * rhs)
    }
}

impl Add<Self> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Vector3 { x, y, z, .. } = self;
        let Vector3 {
            x: ax,
            y: ay,
            z: az,
            ..
        } = rhs;

        Self::new(x + ax, y + ay, z + az)
    }
}

impl Add<&Self> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Self) -> Self::Output {
        let Vector3 { x, y, z, .. } = self;
        let Vector3 {
            x: ax,
            y: ay,
            z: az,
            ..
        } = rhs;

        Vector3::new(x + ax, y + ay, z + az)
    }
}

impl Add<Self> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        let Vector3 { x, y, z, .. } = self;
        let Vector3 {
            x: ax,
            y: ay,
            z: az,
            ..
        } = rhs;

        Vector3::new(x + ax, y + ay, z + az)
    }
}

impl Add<Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        let Vector3 { x, y, z, .. } = self;
        let Vector3 {
            x: ax,
            y: ay,
            z: az,
            ..
        } = rhs;

        Vector3::new(x + ax, y + ay, z + az)
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        *self = *self + rhs;
    }
}

impl Sub<Self> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let Vector3 { x, y, z, .. } = self;
        let Vector3 {
            x: ax,
            y: ay,
            z: az,
            ..
        } = rhs;

        Self::new(x - ax, y - ay, z - az)
    }
}

impl Sub<&Self> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Self) -> Self::Output {
        let Vector3 { x, y, z, .. } = self;
        let Vector3 {
            x: ax,
            y: ay,
            z: az,
            ..
        } = rhs;

        Vector3::new(x - ax, y - ay, z - az)
    }
}

impl Sub<Self> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        let Vector3 { x, y, z, .. } = self;
        let Vector3 {
            x: ax,
            y: ay,
            z: az,
            ..
        } = rhs;

        Vector3::new(x - ax, y - ay, z - az)
    }
}

impl Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        let Vector3 { x, y, z, .. } = self;
        let Vector3 {
            x: ax,
            y: ay,
            z: az,
            ..
        } = rhs;

        Vector3::new(x - ax, y - ay, z - az)
    }
}

impl SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        *self = *self - rhs;
    }
}

impl From<[Real; 3]> for Vector3 {
    fn from([x, y, z]: [Real; 3]) -> Self {
        Vector3::new(x, y, z)
    }
}

impl From<&[Real; 3]> for Vector3 {
    fn from([x, y, z]: &[Real; 3]) -> Self {
        Vector3::new(*x, *y, *z)
    }
}

impl From<&[Real]> for Vector3 {
    fn from(value: &[Real]) -> Self {
        match value {
            &[x, y, z] => Vector3::new(x, y, z),
            _ => {
                let len = value.len();
                panic!("Cannot create vector from {len} length slice")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Real, Vector3, EPSILON};

    #[test]
    fn invert() {
        let mut v = Vector3::new(1., 2., 3.);
        assert_eq!(v.x, 1.);
        assert_eq!(v.y, 2.);
        assert_eq!(v.z, 3.);

        v.invert();
        assert_eq!(v.x, -1.0);
        assert_eq!(v.y, -2.0);
        assert_eq!(v.z, -3.0);
    }

    #[test]
    fn magnitude() {
        let v = Vector3::new(1., 1., 1.);
        let magnitude = v.magnitude();
        assert_approx(magnitude, 1.732);

        let v = Vector3::new(3., 4., 0.);
        let magnitude = v.magnitude();
        assert_approx(magnitude, 5.);

        let v = Vector3::new(0., 0., 0.);
        let magnitude = v.magnitude();
        assert_approx(magnitude, 0.);
    }

    #[test]
    fn normalise() {
        let normalised = Vector3::new(1., 1., 1.).normalise().unwrap();
        assert!(
            normalised
                == Vector3::new(
                    1.0 / 3.0_f32.sqrt(),
                    1.0 / 3.0_f32.sqrt(),
                    1.0 / 3.0_f32.sqrt(),
                )
        );

        let normalised = Vector3::new(3., 4., 0.).normalise().unwrap();
        assert!(normalised == Vector3::new(3.0 / 5.0, 4.0 / 5.0, 0.,));

        assert!(Vector3::new(0., 0., 0.).normalise().is_none());
    }

    #[test]
    fn addition() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(1.0, 2.0, 3.0);
        let summed = a + b;

        assert_eq!(summed, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn subtraction() {
        let a = Vector3::new(2.0, 4.0, 6.0);
        let b = Vector3::new(1.0, 2.0, 3.0);
        let subtracted = a - b;

        assert_eq!(subtracted, Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn dot() {
        let a = Vector3::new(1., 2., 3.);
        let b = Vector3::new(4., -5., 6.);
        assert_approx(a.dot(b), 12.);

        let a = Vector3::new(3., -2., 1.);
        let b = Vector3::new(1., 4., 5.);
        assert_approx(a.dot(b), 0.);
    }

    #[test]
    fn cross() {
        let a = Vector3::new(1., 2., 3.);
        let b = Vector3::new(4., 5., 6.);
        assert_eq!(a.cross(b), Vector3::new(-3., 6., -3.));
    }

    fn assert_approx(a: Real, b: Real) {
        assert!((a - b).abs() < EPSILON);
    }
}
