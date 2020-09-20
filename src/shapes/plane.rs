use crate::math::*;

pub struct Plane {
    pub normal: Norm3,
    pub distance_to_origin: f32,
}

impl Plane {
    pub fn new(normal: Norm3, distance_to_origin: f32) -> Self {
        Self{normal, distance_to_origin}
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let denom = self.normal.dot(&ray.dir);

        if denom.abs() > 0.0001 {
            let plane_center = self.normal * self.distance_to_origin;
            let diff = plane_center - ray.origin;
            let t = diff.dot(&self.normal) / denom;

            if t > 0. {
                return Some(Hit{
                    hitpoint: ray.walk(t),
                    normal: self.normal,
                    point_on_ray: t,
                    inside: false
                })
            }
        }
        None
    }
}
