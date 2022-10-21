use crate::math::{Ray, Vec3, vec3};
use super::{materials::Material, aabb::{Aabb, surrounding_box}};

pub trait Hittable {
    fn intersect<'a>(&'a self, r: &mut Ray, i: &mut Intersection<'a>);
    fn bounding_box(&self, time0: f64, time1: f64) -> Aabb;
}

pub struct Intersection<'a> {
    pub min: f64,
    pub max: f64,
    pub obj: Option<&'a Elem>,
}

#[derive(Debug)]
pub enum Elem {
    Sphere { 
        origin: Vec3, 
        radius: f64, 
        mat: Material
    },
    MovingSphere {
        origin0: Vec3,
        origin1: Vec3,
        radius: f64,
        time0: f64,
        time1: f64,
        mat: Material,
    },
}

impl Elem {
    pub fn get_mat(&self) -> &Material {
        match self {
            Self::Sphere { mat, .. } => mat,
            Self::MovingSphere { mat, .. } => mat,
        }
    }

    pub fn compute_normal(&self, r: &Ray) -> Vec3 {
        match *self {
            Self::Sphere { origin, radius, mat: _ } => {
                (r.origin - origin) / radius
            },
            Self::MovingSphere { origin0, origin1, radius, time0, time1, mat: _} => {
                let origin = origin0 + ((r.time - time0) / (time1 - time0)) * (origin1 - origin0);
                (r.origin - origin) / radius
            },
        }
    }

    pub fn compute_uv(&self, n: &Vec3) -> (f64, f64) {
        //let u = n[0].atan2(n[2]) / (2.0 * std::f64::consts::PI) + 0.5;
        //let v = n[1] * 0.5 + 0.5;
        let theta = (-n[1]).acos();
        let phi = (-n[2]).atan2(n[0]) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;

        (u, v)      
    }
}

impl Hittable for Elem {
    fn intersect<'a>(&'a self, r: &mut Ray, i: &mut Intersection<'a>) {
        match self {
            Self::Sphere { origin, radius, mat: _ } => {
                let diff = r.origin - origin;
                let a = Vec3::dot(&r.dir, &r.dir);
                let b_half = Vec3::dot(&r.dir, &diff);
                let c = Vec3::dot(&diff,&diff) - radius * radius;

                let discriminant = b_half * b_half - a * c;
                if discriminant < 0.0 {
                    return;
                }

                let sqrtd = discriminant.sqrt();

                // Find nearest root in our range
                let mut root = (-b_half - sqrtd) / a;
                if root < i.min || root > i.max {
                    root = (-b_half + sqrtd) / a;
                    if root < i.min || root > i.max {
                        return;
                    }
                }
                i.max = root;
                i.obj = Some(self);
            },
            Self::MovingSphere { origin0, origin1, radius, time0, time1, mat: _ } => {
                let origin = origin0 + ((r.time - time0) / (time1 - time0)) * (origin1 - origin0);
                let diff = r.origin - origin;
                let a = Vec3::dot(&r.dir, &r.dir);
                let b_half = Vec3::dot(&r.dir, &diff);
                let c = Vec3::dot(&diff,&diff) - radius * radius;

                let discriminant = b_half * b_half - a * c;
                if discriminant < 0.0 {
                    return;
                }

                let sqrtd = discriminant.sqrt();

                // Find nearest root in our range
                let mut root = (-b_half - sqrtd) / a;
                if root < i.min || root > i.max {
                    root = (-b_half + sqrtd) / a;
                    if root < i.min || root > i.max {
                        return;
                    }
                }
                i.max = root;
                i.obj = Some(self);
            },
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Aabb {
        match *self {
            Self::Sphere { origin, radius, mat:_ } => {
                Aabb {
                    min: origin - vec3!(radius, radius, radius),
                    max: origin + vec3!(radius, radius, radius),
                }
            },
            Self::MovingSphere { origin0, origin1, radius, time0, time1, mat:_ } => {
                let center0 = origin0 + ((t0 - time0) / (time1 - time0)) * (origin1 - origin0);
                let box1 = Aabb {
                    min: center0 - vec3!(radius, radius, radius),
                    max: center0 + vec3!(radius, radius, radius),
                };

                let center1 = origin0 + ((t1 - time0) / (time1 - time0)) * (origin1 - origin0);
                let box2 = Aabb {
                    min: center1 - vec3!(radius, radius, radius),
                    max: center1 + vec3!(radius, radius, radius),
                };

                surrounding_box(&box1, &box2)
            },
        }
    }

}

