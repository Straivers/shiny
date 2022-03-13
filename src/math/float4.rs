use std::ops::{Add, Sub};

#[cfg(target_arch = "x86_64")]
use super::x86::vector4::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Float4(pub(super) Vector4);

impl Float4 {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vector4::from_tuple(x, y, z, w))
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.0.extract().0
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.0.extract().1
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        self.0.extract().2
    }

    #[inline(always)]
    pub fn w(&self) -> f32 {
        self.0.extract().3
    }

    #[inline(always)]
    pub fn zwxy(&self) -> Self {
        Self(self.0.zwxy())
    }

    #[inline(always)]
    pub fn max(&self, rhs: &Self) -> Self {
        Self(self.0.max(rhs.0))
    }

    #[inline(always)]
    pub fn min(&self, rhs: &Self) -> Self {
        Self(self.0.min(rhs.0))
    }

    #[inline(always)]
    pub fn mul_elements(&self, rhs: &Self) -> Self {
        Self(self.0.mul(rhs.0))
    }

    #[inline(always)]
    pub fn div_elements(&self, rhs: &Self) -> Self {
        Self(self.0.div(rhs.0))
    }
}

impl Add for Float4 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl Sub for Float4 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}

impl PartialEq for Float4 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other.0)
    }
}

impl PartialEq<(f32, f32, f32, f32)> for Float4 {
    #[inline(always)]
    fn eq(&self, other: &(f32, f32, f32, f32)) -> bool {
        self.0
            .eq(Vector4::from_tuple(other.0, other.1, other.2, other.3))
    }
}

impl std::fmt::Debug for Float4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Float4")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .field("w", &self.w())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract() {
        let v = Float4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v, (1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn swizzle() {
        {
            // zwxy
            let v = Float4::new(1.0, 2.0, 3.0, 4.0);
            assert_eq!(v.zwxy(), Float4::new(3.0, 4.0, 1.0, 2.0));
        }
    }
}