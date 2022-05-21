use crate::rendering::{
    materials::{Material, Lambertian},
    math::vec3::Vec3,
};

pub struct Hit<T: Material> {
    pub pos: Vec3,
    pub normal: Vec3,
    pub mat: T,
    pub front: bool
}

impl<T: Material> Hit<T> {
    pub fn no_hit() -> Hit<Lambertian> {
        Hit {
            pos: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat: Lambertian {albedo: Vec3::new(0.0, 0.0, 0.0) },
            front: false
        }
    }
}
