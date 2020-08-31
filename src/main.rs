use std::fs;
use std::io::prelude::*;
use std::ops::{Add, Sub, Neg, Div, Mul};

struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3{x, y, z}
    }

    pub fn length_sqr(&self) -> f32 {
        self.dot(&self)
    }

    #[allow(dead_code)]
    pub fn length(&self) -> f32 {
        self.length_sqr().sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        return self / self.length_sqr()
    }

    pub fn dot(&self, other: &Vec3) -> f32{
        self.x * other.x + self.y * other.y + self.z * other.z
    }
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

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Vec3::new(self.x*other, self.y*other, self.z*other)
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

struct Sphere {
    pub pos: Vec3,
    pub radius: f32,
}

struct Ray {
    pub start: Vec3,
    pub dir: Vec3,
}

struct IntersectionInfo {
    pub hitpoint: Vec3,
    pub normal: Vec3,
    pub point_on_ray: f32,
    pub inside: bool,
}

struct Camera {
    pub pos: Vec3,
    pub res: (u32, u32),
    screen: (Vec3, Vec3),
}

impl Camera {
    pub fn new(pos: Vec3, res: (u32, u32), screen_size: (f32, f32)) -> Self {
        Camera{
            pos,
            res,
            screen: (
                pos + Vec3::new(-screen_size.0/2., -screen_size.1/2., 1.),
                pos + Vec3::new(screen_size.0/2., screen_size.1/2., 1.)
            )
        }
    }

    pub fn generate_ray(&self, pix: (u32, u32)) -> Ray {
        let mut screen_size = (self.screen.1 - self.screen.0) / Vec3::new(self.res.0 as f32, self.res.1 as f32, 1.);
        screen_size.x *= pix.0 as f32;
        screen_size.y *= pix.1 as f32;

        Ray{
            start: self.pos,
            dir: (self.screen.0 + screen_size - self.pos).normalized()
        }
    }
}

trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo>;
}

impl Primitive for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
        let rr = self.radius.powi(2);
        let l = self.pos - ray.start;
        let ll = l.dot(&l);
        let s = l.dot(&ray.dir);

        if s < 0. && ll > rr {
            return None;
        }

        let mm = ll - s*s;
        if mm > rr {
            return None;
        }

        let u = (ll-rr).signum();
        let t = s - (rr - mm).sqrt() * u;

        if t < 0. {
            return None;
        }
        let p = ray.start + ray.dir*t;
        Some(IntersectionInfo{
            hitpoint: p,
            normal: (p - self.pos).normalized(),
            point_on_ray: t,
            inside: u<=0.
        })
    }
}

fn write_ppm(filename: &str, size: (u32, u32), data: &[u8]) -> std::io::Result<()> {
    let mut file = fs::File::create(filename)?;

    // write header
    write!(file, "P6\n{} {}\n255\n", size.0, size.1)?;

    file.write(data)?;

    Ok(())
}

fn fill_color(image: &mut [u8], pixel: (u32, u32), res: (u32, u32), color: &Vec3){
    let to_index = |c| (pixel.1*res.0*3 + pixel.0*3 + c) as usize;

    image[to_index(0)] = (color.x*255.) as u8;
    image[to_index(1)] = (color.y*255.) as u8;
    image[to_index(2)] = (color.z*255.) as u8;
}

fn main() {
    let cam = Camera::new(Vec3::new(0.,0.,-1.), (256, 256), (1., 1.));

    let scene = vec![
        Sphere{pos: Vec3::new(0., 0., 2.), radius: 1.0},
        Sphere{pos: Vec3::new(0.4, 0., 2.), radius: 0.9},
    ];

    let materials = vec![
        Vec3::new(1., 0., 0.),
        Vec3::new(0., 1., 0.),
    ];


    let mut data = Vec::new();
    data.resize((cam.res.0 * cam.res.1 * 3) as usize, 0);

    for x in 0..cam.res.0 {
        for y in 0..cam.res.1 {
            let ray = cam.generate_ray((x,y));

            let mut hit: Option<IntersectionInfo> = None;
            let mut mat: Option<Vec3> = None;

            for i in 0..scene.len() {
                let s = &scene[i];
                if let Some(info) = s.intersect(&ray) {
                    match &hit {
                        None => {hit = Some(info); mat = Some(materials[i])},
                        Some(prev_hit) => {
                            if info.point_on_ray < prev_hit.point_on_ray {
                                hit = Some(info);
                                mat = Some(materials[i]);
                            }
                        }
                    }
                }
            }
            if let Some(info) = hit {
                if info.inside {
                    continue;
                }
                let base_color = mat.unwrap();
                let color = base_color * (-ray.dir).dot(&info.normal);
                fill_color(&mut data, (x,y), cam.res, &color);
            }
        }
    }

    write_ppm("test.ppm", cam.res, &data).unwrap();
}
