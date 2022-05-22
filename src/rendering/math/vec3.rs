use std::ops::*;

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

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Vec3 { elems: [self[0] / other,
                               self[1] / other,
                               self[2] / other] }
        }
}
