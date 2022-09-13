use crate::rendering::math::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub min: f32,
    pub max: f32,
}

impl Ray {
    pub fn reflect(&mut self, n: Vec3) {
        self.dir -= n * Vec3::dot(&self.dir, &n) * 2.0;             
    }

    pub fn refract(&mut self, n: Vec3, ir: f32, cos_thetai: f32, cos_thetat: f32) {
        self.dir = self.dir * ir + n * (ir * cos_thetai - cos_thetat);
    }

    pub fn schlick(ir: f32, cos_theta: f32) -> f32 {
        let mut r0 = (1.0 - ir) / (1.0 + ir);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}
