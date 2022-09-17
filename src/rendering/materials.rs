use crate::rendering::math::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ir: f32 },
}

