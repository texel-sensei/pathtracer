use std::rc::Rc;
use std::f32::consts::PI;

use pathtracer::io;
use pathtracer::math::*;
use pathtracer::shapes::sphere::Sphere;


trait Material {
    fn sample(&self, hit_info: &Hit, incomming: &Ray, integrator: &Integrator) -> Vec3;
    fn total_emission(&self, sampled_color: Vec3) -> Vec3; }


struct IntersectionInfo<'a> {
    pub hit: Hit,
    pub material: &'a Material, }

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

struct Integrator<'a> {
    pub scene: &'a Scene,
}

impl<'a> Integrator<'a> {
    pub fn new(scene: &'a Scene) -> Self {
        Self{scene}
    }

    pub fn send_ray(&self, ray: &Ray) -> Option<IntersectionInfo> {
        let mut hit: Option<IntersectionInfo> = None;

        for s in self.scene.iter() {
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
        return hit;
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

fn concentric_sample_disc(p: (f32, f32)) -> (f32, f32) {
    let mut p = p;
    p.0 = 2. * p.0 - 1.;
    p.1 = 2. * p.1 - 1.;

    if p.0 == 0. && p.1 == 0. {
        return (0., 0.);
    }

    let (r, theta) = if p.0.abs() < p.1.abs() {
        (p.0, PI/4. * (p.1/p.0))
    } else {
        (p.1, PI/2. - PI/4. * (p.0/p.1))
    };

    (r * theta.cos(), r * theta.sin())
}

fn cosine_sample_hemisphere(p: (f32, f32)) -> Vec3 {
    let p = concentric_sample_disc(p);
    let z = (1. - p.0*p.0 - p.1*p.1).max(0.).sqrt();

    Vec3::new(p.0, p.1, z)
}

fn hemisphere_sample(n: &Vec3, p: (f32, f32)) -> Vec3 {
    // build orthonormal basis
    let xx = if n.dot(&Vec3::new(1., 0., 0.)).abs() > 0.9999 {
        Vec3::new(0., 1., 0.)
    } else {
        Vec3::new(1., 0., 0.)
    }.cross(n).normalized();
    let yy = n.cross(&xx).normalized();
    let zz = n;

    let s = cosine_sample_hemisphere(p);

    xx * s.x + yy * s.y + zz * s.z
}

impl Material for Vec3 {
    fn sample(&self, hit_info: &Hit, _incomming: &Ray, integrator: &Integrator) -> Vec3 {
        let sample_x = 50;
        let sample_y = sample_x;

        let start = hit_info.hitpoint + 0.001 * &hit_info.normal;

        let mut accumulator = Vec3::new(0., 0., 0.);
        for x in 0..=sample_x {
            for y in 0..=sample_y {
                let p = (x as f32 / sample_x as f32, y as f32 / sample_y as f32);
                let dir = hemisphere_sample(&hit_info.normal, p);

                if let Some(hit) = integrator.send_ray(&Ray{start, dir}) {
                    let light = hit.material.total_emission(Vec3::new(1., 1., 1.));
                    let color = light * *self * dir.dot(&hit_info.normal).max(0.);
                    accumulator = accumulator + color;
                }
            }
        }

        accumulator/(sample_x * sample_y) as f32
        //hit_info.normal
    }
    fn total_emission(&self, sampled_color: Vec3) -> Vec3 {
        return sampled_color;
    }
}

fn fill_color(image: &mut [u8], pixel: (u32, u32), res: (u32, u32), color: &Vec3){
    let to_index = |c| (pixel.1*res.0*3 + pixel.0*3 + c) as usize;

    image[to_index(0)] = (color.x*255.) as u8;
    image[to_index(1)] = (color.y*255.) as u8;
    image[to_index(2)] = (color.z*255.) as u8;
}

struct EmissiveMaterial {
    pub emissiveness: f32,
}

impl Material for EmissiveMaterial {
    fn sample(&self, _hit_info: &Hit, _incomming: &Ray, _integrator: &Integrator) -> Vec3 {
        Vec3::new(1., 1., 1.)
    }

    fn total_emission(&self, sampled_color: Vec3) -> Vec3 {
        sampled_color * self.emissiveness
    }
}

struct Scene {
   primitives: Vec::<Box::<Primitive>>,
}

impl Scene {
    pub fn new() -> Self {
        Self{primitives: vec![]}
    }

    pub fn add_primitive(&mut self, p: Box::<Primitive>) {
       self.primitives.push(p)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Primitive> {
        self.primitives.iter().map(Box::as_ref)
    }
}

fn main() {
    let cam = Camera::new(Vec3::new(0.,0.,-2.0), (256, 256), (1., 1.));

    let materials: Vec::<Rc::<Material>> = vec![
        Rc::new(Vec3::new(1., 0., 0.)),
        Rc::new(Vec3::new(0., 1., 0.)),
        Rc::new(Vec3::new(0., 0., 1.)),
        Rc::new(EmissiveMaterial{emissiveness: 100.}),
    ];

    let mut scene = Scene::new();

    scene.add_primitive(Box::new(
        SpherePrimitive{collider: Sphere::new(Vec3::new(-0.45, 0., 0.5), 0.4), material: materials[0].clone()}
    ));
    scene.add_primitive(Box::new(
        SpherePrimitive{collider: Sphere::new(Vec3::new(0.45, 0., 0.5), 0.4), material: materials[1].clone()}
    ));
    scene.add_primitive(Box::new(
        SpherePrimitive{collider: Sphere::new(Vec3::new(0., 0.45, 0.5), 0.4), material: materials[2].clone()}
    ));
    scene.add_primitive(Box::new(
        SpherePrimitive{collider: Sphere::new(Vec3::new(0., -0.25, 0.1), 0.1), material: materials[3].clone()}
    ));

    let integrator = Integrator::new(&scene);

    let mut data = Vec::new();
    data.resize((cam.res.0 * cam.res.1 * 3) as usize, 0);

    for x in 0..cam.res.0 {
        for y in 0..cam.res.1 {
            let ray = cam.generate_ray((x,y));

            let hit = integrator.send_ray(&ray);

            if let Some(info) = hit {
                if info.hit.inside {
                    continue;
                }
                let color = info.material.sample(&info.hit, &ray, &integrator);
                fill_color(&mut data, (x,y), cam.res, &color);
            }
        }
    }

    io::write_ppm("test.ppm", cam.res, &data).unwrap();
}
