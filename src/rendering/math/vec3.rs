use std::ops::{Sub, Add, Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub elems: [f32; 3]
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { elems: [x, y, z] }
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
        u.elems[0] * v.elems[0]
        + u.elems[1] * v.elems[1]
        + u.elems[2] * v.elems[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Self {
        Vec3 {
            elems: [u.elems[1] * v.elems[2] - u.elems[2] * v.elems[1], 
                    u.elems[2] * v.elems[0] - u.elems[0] * v.elems[2], 
                    u.elems[0] * v.elems[1] - u.elems[1] * v.elems[0]]
        }
    }

    pub fn unit(&self) -> Self {
        self / Vec3::dot(self, self).sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 { elems: [self.elems[0] + other.elems[0],
                       self.elems[1] + other.elems[1],
                       self.elems[2] + other.elems[2]] }
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 { elems: [self.elems[0] + other.elems[0],
                       self.elems[1] + other.elems[1],
                       self.elems[2] + other.elems[2]] }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Self {
        Vec3::new(self.elems[0] * other,
                  self.elems[1] * other,
                  self.elems[2] * other)
    }

}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(self.elems[0] * other,
                  self.elems[1] * other,
                  self.elems[2] * other)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 { elems: [self.elems[0] - other.elems[0],
                       self.elems[1] - other.elems[1],
                       self.elems[2] - other.elems[2]] }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 { elems: [self.elems[0] - other.elems[0],
                       self.elems[1] - other.elems[1],
                       self.elems[2] - other.elems[2]] }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Vec3 { elems: [self.elems[0] / other,
                       self.elems[1] / other,
                       self.elems[2] / other] }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3{
        Vec3 { elems: [self.elems[0] / other,
                       self.elems[1] / other,
                       self.elems[2] / other] }
    }
}
