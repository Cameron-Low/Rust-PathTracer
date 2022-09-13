use rand::random;

use crate::rendering::math::{Vec3, vec3, Ray};

pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ir: f32 },
}

impl Material {
    pub fn scatter(&self, r: &mut Ray, normal: Vec3) -> (bool, Vec3) {
        match *self {
            Material::Lambertian { albedo } => {
                r.dir = Vec3::random_in_hemi(&normal);
                if Vec3::close_to_zero(r.dir) {
                    r.dir = normal;
                }
                r.dir = r.dir.unit();
                (false, albedo)
            },
            Material::Metal { albedo, fuzz } => {
                r.reflect(normal);
                r.dir += Vec3::random_in_unit_sphere() * fuzz;
                r.dir = r.dir.unit();
                (Vec3::dot(&r.dir, &normal) < 0.0, albedo)
            },
            Material::Dielectric { mut ir } => {
                let albedo = vec3!(1.0, 1.0, 1.0);

                let mut cos_thetai = Vec3::dot(&r.dir, &normal).clamp(-1.0, 1.0);
                let mut n = normal;

                // Are we entering the medium
                if cos_thetai < 0.0 {
                    cos_thetai = -cos_thetai;
                    ir = 1.0 / ir;
                } else {
                    n = -normal;
                }
                let sin_theta_sq = ir * ir * (1.0 - cos_thetai * cos_thetai);
                let cos_thetat = (1.0 - sin_theta_sq).sqrt();

                //Check for total internal reflection and viewing angles
                if sin_theta_sq > 1.0 || Ray::schlick(ir, cos_thetai) > random::<f32>() {
                    r.reflect(n);
                } else {
                    r.refract(n, ir, cos_thetai, cos_thetat);
                }

                (false, albedo)
            }
        }
    }
}
