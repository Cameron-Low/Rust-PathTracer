use std::ops::*;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub elems: [f32; 3]
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { elems: [x, y, z] }
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
        u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Self {
        Vec3 {
            elems: [u[1] * v[2] - u[2] * v[1], 
                    u[2] * v[0] - u[0] * v[2], 
                    u[0] * v[1] - u[1] * v[0]]
        }
    }

    pub fn len_sq(&self) -> f32 {
        Self::dot(self, self)
    }

    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    pub fn unit(&self) -> Self {
        self / self.len()
    }

    pub fn close_to_zero(v: Vec3) -> bool {
        let err = 1e-8;
        v[0].abs() < err && v[1].abs() < err && v[2].abs() < err 
    }

    pub fn random_vec(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        vec3!(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }

    pub fn random_unit_vec() -> Vec3 {
       Self::random_in_unit_sphere().unit() 
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::random_vec(-1.0, 1.0);
            if p.len_sq() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_hemi(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();

        if Vec3::dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = vec3!(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.len_sq() >= 1.0 {
                continue;
            }
            return p;
        }
    }

}

#[macro_export]
macro_rules! vec3 {
    ($x: expr, $y: expr, $z: expr) => {Vec3::new($x, $y, $z)};
}
pub use vec3;


// Operator overloading

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &f32 {
        &self.elems[idx]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elems[index]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {elems: [-self[0], -self[1], -self[2]]}
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {elems: [-self[0], -self[1], -self[2]]}
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 { elems: [self[0] + other[0],
                       self[1] + other[1],
                       self[2] + other[2]] }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 { elems: [self[0] + other[0],
                       self[1] + other[1],
                       self[2] + other[2]] }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { elems: [self[0] + other[0],
                       self[1] + other[1],
                       self[2] + other[2]] }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 { elems: [self[0] + other[0],
                       self[1] + other[1],
                       self[2] + other[2]] }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 { elems: [self[0] + other[0],
                               self[1] + other[1],
                               self[2] + other[2]] }
        }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        *self = Vec3 { elems: [self[0] + other[0],
                               self[1] + other[1],
                               self[2] + other[2]] }
        }
}


impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 { elems: [self[0] - other[0],
                       self[1] - other[1],
                       self[2] - other[2]] }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 { elems: [self[0] - other[0],
                       self[1] - other[1],
                       self[2] - other[2]] }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { elems: [self[0] - other[0],
                       self[1] - other[1],
                       self[2] - other[2]] }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 { elems: [self[0] - other[0],
                       self[1] - other[1],
                       self[2] - other[2]] }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 { elems: [self[0] - other[0],
                               self[1] - other[1],
                               self[2] - other[2]] }
        }
}

impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, other: &Vec3) {
        *self = Vec3 { elems: [self[0] - other[0],
                               self[1] - other[1],
                               self[2] - other[2]] }
        }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { elems: [self[0] * other[0],
                       self[1] * other[1],
                       self[2] * other[2]] }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Self {
        Vec3::new(self[0] * other,
                  self[1] * other,
                  self[2] * other)
    }

}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(self[0] * other,
                  self[1] * other,
                  self[2] * other)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        other * self
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Vec3 { elems: [self[0] * other,
                               self[1] * other,
                               self[2] * other] }
        }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Vec3 { elems: [self[0] / other,
                       self[1] / other,
                       self[2] / other] }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3{
        Vec3 { elems: [self[0] / other,
                       self[1] / other,
                       self[2] / other] }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        other / self
    }
}

impl Div<&Vec3> for f32 {
    type Output = Vec3;

    fn div(self, other: &Vec3) -> Vec3 {
        other / self
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Vec3 { elems: [self[0] / other,
                               self[1] / other,
                               self[2] / other] }
        }
}
