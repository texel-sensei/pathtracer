use std::ops::{Neg, Mul};
use super::{Vec3, Object3D};

pub struct Norm3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Norm3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let n = Norm3{x, y, z};
        assert!(n.dot(&n) -  1. < 0.001);
        n
    }

    pub fn dot(&self, other: &Object3D) -> f32{
        self.x * other.x() + self.y * other.y() + self.z * other.z()
    }

    pub fn cross(&self, other: &Object3D) -> Vec3 {
        Vec3::new(
            self.y*other.z() - self.z*other.y(),
            self.z*other.x() - self.x*other.z(),
            self.x*other.y() - self.y*other.x()
        )
    }

}

impl Object3D for Norm3 {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }
}

impl Clone for Norm3 {
    fn clone(&self) -> Self {
        Norm3::new(self.x, self.y, self.z)
    }
}
impl Copy for Norm3 {}

impl Neg for Norm3 {
    type Output = Self;

    fn neg(self) -> Self {
        Norm3::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Norm3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Self::Output {
        Self::Output::new(self.x*other, self.y*other, self.z*other)
    }
}

impl Mul<f32> for &Norm3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Self::Output {
        Self::Output::new(self.x*other, self.y*other, self.z*other)
    }
}

impl Mul<&Norm3> for f32 {
    type Output = Vec3;

    fn mul(self, other: &Norm3) -> Self::Output {
        other * self
    }
}

impl Into<Vec3> for Norm3 {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}
