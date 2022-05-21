use crate::rendering::{
    ray::Ray,
    math::vec3::Vec3
};


pub trait Material : Copy {
    fn compute_shading(self, r: &Ray, normal: &Vec3) -> Vec3;
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn compute_shading(self, r: &Ray, normal: &Vec3) -> Vec3 {
        self.albedo * Vec3::dot(&normal, &r.dir).max(0.0)
    }
}
