use crate::rendering::math::{Ray, Vec3};
use super::materials::Material;

pub trait Hittable {
    fn intersect(&self, r: &mut Ray, i: &mut Intersection);
}

pub struct Intersection {
    pub min: f32,
    pub max: f32,
    pub obj: Option<Elem>,
}

#[derive(Debug, Clone, Copy)]
pub enum Elem {
    Sphere { origin: Vec3, radius: f32, mat: Material},
}

impl Elem {
    pub fn get_mat(&self) -> Material {
        match *self {
            Self::Sphere { mat, .. } => mat,
        }
    }

    pub fn compute_normal(&self, r: &Ray) -> Vec3 {
        match *self {
            Self::Sphere { origin, radius, mat: _ } => {
                (r.origin - origin) / radius
            },
        }
    }
}

impl Hittable for Elem {
    fn intersect(&self, r: &mut Ray, i: &mut Intersection) {
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
                i.obj = Some(*self);
            }
        }
    }

}

