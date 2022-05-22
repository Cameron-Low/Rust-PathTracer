use crate::rendering::{
    math::vec3::Vec3,
    ray::Ray,
    materials::Material
};

pub trait Object {
    fn intersect(&self, r: &mut Ray) -> bool;
    fn scatter(&self, r: &mut Ray, l: &Vec3) -> Vec3;
}

pub struct Sphere<T: Material> {
    pub origin: Vec3,
    pub radius: f32,
    pub mat: T
}

impl<T: Material> Object for Sphere<T> {
    fn intersect(&self, r: &mut Ray) -> bool {
        let diff = r.origin - &self.origin;
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

    fn scatter(&self, r: &mut Ray, l: &Vec3) -> Vec3 {
        let mut normal = ((r.origin - self.origin) / self.radius).unit();
        let front = Vec3::dot(&normal, &r.dir) < 0.0;
        if front {
            normal = normal * -1.0;
        }
        r.dir = *l;
        self.mat.compute_shading(r, &normal)
    }
}

pub struct Triangle<T: Material> {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub mat: T
}

impl<T: Material> Object for Triangle<T> {
    fn intersect(&self, r: &mut Ray) -> bool {
        let v01 = &self.v1 - &self.v0;
        let v02 = &self.v2 - &self. v0;
        let pvec = Vec3::cross(&r.dir, &v02);
        
        let det = Vec3::dot(&v01, &pvec);
        if det.abs() < 0.0001 {
            return false 
        }

        let inv_det = 1.0 / det;
        let tvec = r.origin - self.v0;
        
        let u = Vec3::dot(&tvec, &pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
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
    
    fn scatter(&self, r: &mut Ray, l: &Vec3) -> Vec3 {
        &r.origin - l
    }
}

