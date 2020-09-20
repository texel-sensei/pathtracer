use crate::math::*;
use std::mem;

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f32) -> Self {
        Self{pos, radius}
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let l = self.pos - ray.origin;
        let tca = l.dot(&ray.dir);

        if tca < 0. {
            return None;
        }

        let d2 = l.dot(&l) - tca * tca;
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }

        let thc = (radius2 - d2).sqrt();

        let mut t0 = tca - thc;
        let mut t1 = tca + thc;

        if t0 > t1 {
            mem::swap(&mut t0, &mut t1);
        }

        if t0 < 0. {
            t0 = t1;
            if t0 < 0. {
                return None;
            }
        }

        let t = t0;

        let hitpoint = ray.origin + t * &ray.dir;
        let normal = (hitpoint - self.pos).normalized();

        Some(Hit{
            hitpoint: hitpoint,
            normal: normal,
            point_on_ray: t,
            inside: false
        })
    }
}
