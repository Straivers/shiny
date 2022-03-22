use std::ops::{Add, Div, Mul, Sub};

/// A vector in 2D space.
#[derive(Clone, Copy, Default, PartialEq)]
#[allow(non_camel_case_types)]
pub struct Float2(f32, f32);

impl Float2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    pub fn x(self) -> f32 {
        self.0
    }

    pub fn y(self) -> f32 {
        self.1
    }

    pub fn length(self) -> f32 {
        self.length2().sqrt()
    }

    pub fn length2(self) -> f32 {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        let x = self.x() * rhs.x();
        let y = self.y() * rhs.y();
        x + y
    }
}

impl Mul<Float2> for f32 {
    type Output = Float2;
    fn mul(self, rhs: Float2) -> Self::Output {
        rhs.mul(self)
    }
}

impl Mul<f32> for Float2 {
    type Output = Float2;

    fn mul(self, rhs: f32) -> Self::Output {
        Float2(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<f32> for Float2 {
    type Output = Float2;
    fn div(self, rhs: f32) -> Self::Output {
        Float2(self.0 / rhs, self.1 / rhs)
    }
}

impl Add<Float2> for Float2 {
    type Output = Float2;
    fn add(self, rhs: Float2) -> Self::Output {
        Float2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Float2> for Float2 {
    type Output = Float2;
    fn sub(self, rhs: Float2) -> Self::Output {
        Float2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::fmt::Debug for Float2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("vec2")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}
