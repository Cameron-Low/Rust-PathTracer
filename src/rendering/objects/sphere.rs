use crate::rendering::{
    materials::Material,
    math::{Ray, Vec3},
    objects::Object,
};

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
    pub mat: Material, 
}

impl Object for Sphere {
    fn intersect(&self, r: &mut Ray) -> bool {
        let diff = r.origin - self.origin;
        let a = Vec3::dot(&r.dir, &r.dir);
        let b_half = Vec3::dot(&r.dir, &diff);
        let c = Vec3::dot(&diff,&diff) - self.radius * self.radius;

        let discriminant = b_half * b_half - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find nearest root in our range
        let mut root = (-b_half - sqrtd) / a;
        if root < r.min || root > r.max {
            root = (-b_half + sqrtd) / a;
            if root < r.min || root > r.max {
                return false;
            }
        }
        r.max = root;
        true
    }

    fn compute_normal(&self, r: &Ray) -> Vec3 {
        (r.origin - self.origin) / self.radius
    }

    fn scatter(&self, r: &mut Ray) -> (bool, Vec3) {
        let normal = self.compute_normal(r);
        self.mat.scatter(r, normal)
    }
}
