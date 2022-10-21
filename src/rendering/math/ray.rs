use crate::math::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub inv_dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn move_along(&mut self, t: f64) {
        self.origin += self.dir * t;
    }

    pub fn reflect(&mut self, n: Vec3) {
        self.dir -= n * Vec3::dot(&self.dir, &n) * 2.0;             
    }

    pub fn refract(&mut self, n: Vec3, ir: f64, cos_thetai: f64, cos_thetat: f64) {
        self.dir = self.dir * ir + n * (ir * cos_thetai - cos_thetat);
    }

    pub fn schlick(ir: f64, cos_theta: f64) -> f64 {
        let mut r0 = (1.0 - ir) / (1.0 + ir);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}
