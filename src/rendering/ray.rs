use crate::rendering::math::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub min: f32,
    pub max: f32
}
