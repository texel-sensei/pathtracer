use std::ops::{Add, Sub, Neg, Div, Mul};
use super::{Object3D, Norm3};

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3{x, y, z}
    }

    pub fn length_sqr(&self) -> f32 {
        self.dot(self)
    }

    pub fn length(&self) -> f32 {
        self.length_sqr().sqrt()
    }

    pub fn normalized(self) -> Norm3 {
        let tmp = self / self.length();
        Norm3::new(tmp.x, tmp.y, tmp.z)
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

impl Object3D for Vec3 {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3::new(self.x, self.y, self.z)
    }
}
impl Copy for Vec3 {}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self {
        Vec3::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self {
        self + (-other)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3::new(self.x*other.x, self.y*other.y, self.z*other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Vec3::new(self.x*other, self.y*other, self.z*other)
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(self.x*other, self.y*other, self.z*other)
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        other * self
    }
}


impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Vec3::new(self.x/other, self.y/other, self.z/other)
    }
}
impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Vec3) -> Self {
        Vec3::new(self.x/other.x, self.y/other.y, self.z/other.z)
    }
}
