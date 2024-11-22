type Real = f32;

#[derive(Debug, Default, Clone)]
pub struct Vector3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    // oooh, alignment shenannies -WINK-
    _pad: Real,
}

impl Vector3 {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self { x, y, z, _pad: 0. }
    }

    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
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
    use crate::Vector3;

    #[test]
    fn it_works() {
        let mut v = Vector3::new(1., 2., 3.);
        assert_eq!(v.x, 1.);
        assert_eq!(v.y, 2.);
        assert_eq!(v.z, 3.);

        v.invert();
        assert_eq!(v.x, -1.0);
        assert_eq!(v.y, -2.0);
        assert_eq!(v.z, -3.0);
    }
}
