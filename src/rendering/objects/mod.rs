use crate::rendering::math::{Ray, Vec3};

mod sphere;
mod triangle;

pub use sphere::Sphere;
pub use triangle::Triangle;

pub trait Object {
    fn intersect(&self, r: &mut Ray) -> bool;
    fn compute_normal(&self, r: &Ray) -> Vec3;
    fn scatter(&self, r: &mut Ray) -> (bool, Vec3);
}

