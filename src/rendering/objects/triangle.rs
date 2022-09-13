use crate::rendering::{
    materials::Material,
    math::{Ray, Vec3},
    objects::Object,
};

pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub mat: Material,
}

impl Object for Triangle {
    fn intersect(&self, r: &mut Ray) -> bool {
        let v01 = self.v1 - self.v0;
        let v02 = self.v2 - self.v0;
        let pvec = Vec3::cross(&r.dir, &v02);
        
        let det = Vec3::dot(&v01, &pvec);
        if det.abs() < 0.0001 {
            return false 
        }

        let inv_det = 1.0 / det;
        let tvec = r.origin - self.v0;
        
        let u = Vec3::dot(&tvec, &pvec) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return false
        }

        let qvec = Vec3::cross(&tvec, &v01);
        let v = Vec3::dot(&r.dir, &qvec) * inv_det; 
        if v < 0.0 || u + v > 1.0 {
            return false
        }

        r.max = Vec3::dot(&v02, &qvec) * inv_det;
        true
    }
    
    fn compute_normal(&self, r: &Ray) -> Vec3 {
        let v01 = self.v1 - self.v0;
        let v02 = self.v2 - self.v0;
        let mut normal = Vec3::cross(&v01, &v02);

        let front = Vec3::dot(&normal, &r.dir) > 0.0;
        if front {
            normal *= -1.0;
        }

        normal
    }

    fn scatter(&self, r: &mut Ray) -> (bool, Vec3) {
        let normal = self.compute_normal(r);
        self.mat.scatter(r, normal)
    }
}
