use na::{Vec3, Norm, Dot};
use raytracer::Ray;
use num::traits::Zero;
use std::f64::INFINITY;

#[derive(Copy, Clone)]
pub enum Shape {
    Sphere {
        radius: f64,
        center: Vec3<f64>,
    },

    Box {
        vmin: Vec3<f64>,
        vmax: Vec3<f64>,
    }
}

impl Shape {
    pub fn get_normal(&self, ray: &Ray, tnear: f64) -> Vec3<f64> {
        match *self {
            Shape::Box { vmin, vmax } => {
                let phit = ray.origin + ray.dir * tnear;

                let phit_min = phit - vmin;
                let phit_max = phit - vmax;
                let mut nhit = Vec3::zero();
                let eps = 1e-6;
                if phit_min.x.abs() < eps { nhit.x = -1. }
                if phit_min.y.abs() < eps { nhit.y = -1. }
                if phit_min.z.abs() < eps { nhit.z = -1. }
                if phit_max.x.abs() < eps { nhit.x = 1. }
                if phit_max.y.abs() < eps { nhit.y = 1. }
                if phit_max.z.abs() < eps { nhit.z = 1. }
                nhit
            }

            Shape::Sphere { center, .. } => {
                let phit = ray.origin + ray.dir * tnear;
                let mut nhit = (phit - center).normalize();
                if ray.dir.dot(&nhit) > 0. {
                    nhit = -nhit;
                }
                nhit
            }
        }
    }

    pub fn intersect(&self, ray: &Ray) -> (f64, f64) {
        let (mut t0, mut t1) = (INFINITY, INFINITY);
        match *self {
            Shape::Box { vmin, vmax } => {
                let o = ray.origin;
                let mut d = ray.dir;

                d.x = 1. / d.x;
                d.y = 1. / d.y;
                d.z = 1. / d.z;

                let mut sign: Vec3<bool> = Vec3{ x: true, y: true, z: true };
                sign.x = d.x > 0.;
                sign.y = d.y > 0.;
                sign.z = d.z > 0.;

                let b0 = vmin;
                let b1 = vmax;

                let mut tmin = (if sign.x { b0.x } else { b1.x } - o.x) * d.x;
                let mut tmax = (if sign.x { b1.x } else { b0.x } - o.x) * d.x;

                let tymin = (if sign.y { b0.y } else { b1.y } - o.y) * d.y;
                let tymax = (if sign.y { b1.y } else { b0.y } - o.y) * d.y;

                if tmin > tymax || tymin > tmax { return (t0, t1) };
                if tymin > tmin { tmin = tymin };
                if tymax < tmax { tmax = tymax };

                let tzmin = (if sign.z { b0.z } else { b1.z } - o.z) * d.z;
                let tzmax = (if sign.z { b1.z } else { b0.z } - o.z) * d.z;

                if tmin > tzmax || tzmin > tmax { return (t0, t1) };
                if tzmin > tmin { tmin = tzmin };
                if tzmax < tmax { tmax = tzmax };

                if tmin < 0. {
                    if tmax < 0. { return (t0, t1) };
                    t0 = tmax;
                } else {
                    t0 = tmin;
                }
                (t0, t1)
            }

            Shape::Sphere { radius, center } => {
                let l = center - ray.origin;
                let tca = l.dot(&ray.dir);
                if tca < 0. { return (t0, t1); }
                let d2 = l.dot(&l) - tca * tca;
                let r2 = radius * radius;
                if d2 > r2 { return (t0, t1); }
                let thc: f64 = (r2 - d2).sqrt();
                t0 = tca - thc;
                t1 = tca + thc;
                (t0, t1)
            }
        }
    }
}

pub struct BoxBuilder {
    boxes: Vec<Box<Shape>>
}

impl BoxBuilder {
    pub fn new() -> BoxBuilder {
        BoxBuilder{ boxes: Vec::new() }
    }

    /// Adds a square box to vector
    pub fn add(mut self, x: i32, y: i32, z: i32, size: i32)
               -> BoxBuilder{
        assert!(size > 0);
        let new_box = Box::new(Shape::Box
                               { vmin: Vec3 { x: x as f64,
                                              y: y as f64,
                                              z: z as f64 },
                                 vmax: Vec3 { x: (x + size) as f64,
                                              y: (y + size) as f64,
                                              z: (z + size) as f64 } });
        self.boxes.push(new_box);
        self
    }

    pub fn build(self) -> Vec<Box<Shape>> {
        self.boxes
    }
}
