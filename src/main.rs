use std::fs;
use std::rc::Rc;
use std::io::prelude::*;

use pathtracer::math::*;
use pathtracer::shapes::sphere::Sphere;


trait Material {
    fn sample(&self, hit_info: &Hit, incomming: &Ray) -> Vec3;
    fn total_emission(&self, sampled_color: Vec3) -> Vec3; }


struct IntersectionInfo<'a> {
    pub hit: Hit,
    pub material: &'a Material,
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
        let screen_size = self.screen.1 - self.screen.0;
        let mult = Vec3::new(pix.0 as f32, pix.1 as f32, 1.) / Vec3::new(self.res.0 as f32, self.res.1 as f32, 1.);

        Ray{
            start: self.pos,
            dir: ((self.screen.0 + screen_size*mult) - self.pos).normalized()
        }
    }
}

trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo>;
}

struct SpherePrimitive {
    collider: Sphere,
    material: Rc::<Material>,
}

impl Primitive for SpherePrimitive {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
        self.collider.intersect(ray).map(|hit|{
            IntersectionInfo{hit: hit, material: self.material.as_ref()}
        })
    }
}

impl Material for Vec3 {
    fn sample(&self, hit_info: &Hit, _incomming: &Ray) -> Vec3 {
        let light_dir = Vec3::new(1., 1., 1.).normalized();
        let color = self * (-light_dir).dot(&hit_info.normal).max(0.);
        color
        //hit_info.normal
    }
    fn total_emission(&self, sampled_color: Vec3) -> Vec3 {
        return sampled_color;
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
    let cam = Camera::new(Vec3::new(0.,0.,-2.0), (256, 256), (1., 1.));

    let materials = vec![
        Rc::new(Vec3::new(1., 0., 0.)),
        Rc::new(Vec3::new(0., 1., 0.)),
        Rc::new(Vec3::new(0., 0., 1.)),
    ];

    let scene = vec![
        SpherePrimitive{collider: Sphere::new(Vec3::new(-0.45, 0., 0.5), 0.4), material: materials[0].clone()},
        SpherePrimitive{collider: Sphere::new(Vec3::new(0.45, 0., 0.5), 0.4), material: materials[1].clone()},
        SpherePrimitive{collider: Sphere::new(Vec3::new(0., 0.45, 0.5), 0.4), material: materials[2].clone()},
    ];


    let mut data = Vec::new();
    data.resize((cam.res.0 * cam.res.1 * 3) as usize, 0);

    for x in 0..cam.res.0 {
        for y in 0..cam.res.1 {
            let ray = cam.generate_ray((x,y));

            let mut hit: Option<IntersectionInfo> = None;

            for i in 0..scene.len() {
                let s = &scene[i];
                if let Some(info) = s.intersect(&ray) {
                    match &hit {
                        None => {hit = Some(info);},
                        Some(prev_hit) => {
                            if info.hit.point_on_ray < prev_hit.hit.point_on_ray {
                                hit = Some(info);
                            }
                        }
                    }
                }
            }
            if let Some(info) = hit {
                if info.hit.inside {
                    continue;
                }
                let color = info.material.sample(&info.hit, &ray);
                fill_color(&mut data, (x,y), cam.res, &color);
            }
        }
    }

    write_ppm("test.ppm", cam.res, &data).unwrap();
}
