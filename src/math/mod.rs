pub trait Object3D {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

mod vec3;
pub use vec3::Vec3;

mod norm3;
pub use norm3::Norm3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Norm3,
}

impl Ray {
    pub fn distance_to(&self, other: &Vec3) -> f32 {
        let d = *other - self.origin;
        let projection = self.origin.dot(&d) * &d / d.length();
        let compare_point = self.origin + projection;

        (compare_point - *other).length()
    }

    pub fn walk(&self, t: f32) -> Vec3 {
        self.origin + t * &self.dir
    }
}

pub struct Hit {
    pub hitpoint: Vec3,
    pub normal: Norm3,
    pub point_on_ray: f32,
    pub inside: bool,
}
