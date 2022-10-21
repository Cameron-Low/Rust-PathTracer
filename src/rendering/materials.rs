use fastrand::Rng;

use crate::math::{Vec3, vec3};

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian { albedo: Texture },
    Metal { albedo: Texture, fuzz: f64 },
    Dielectric { ir: f64 },
}

#[derive(Debug, Clone)]
pub enum Texture {
    Solid { colour: Vec3 },
    Checker { odd: Box<Texture>, even: Box<Texture> },
    Perlin { noise: PerlinNoise, scale: f64 },
    Image { img: image::RgbImage },
}

impl Texture {
    pub fn colour(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        match self {
            Self::Solid { colour } => *colour,
            Self::Checker { odd, even } => {
                let sines = (10.0 * p[0]).sin() * (10.0 * p[1]).sin() * (10.0 * p[2]).sin();
                if sines < 0.0 {
                    odd.colour(u, v, p)
                } else {
                    even.colour(u, v, p)
                }
            },
            Texture::Perlin { noise, scale } => {
                vec3!(1.0, 1.0, 1.0) * 0.5 * (1.0 + (p[2] * *scale + 10.0 * noise.turb(p, 7)).sin())
            },
            Texture::Image { img } => {
                let uu = u.clamp(0.0, 1.0);
                let vv = 1.0 - v.clamp(0.0, 1.0);

                let mut i = (uu * img.width() as f64) as u32;
                let mut j = (vv * img.height() as f64) as u32;

                if i >= img.width() {
                    i = img.width() - 1;
                }

                if j >= img.height() {
                    j = img.height() - 1;
                }

                let pix = img.get_pixel(i, j);

                vec3!(pix[0] as f64, pix[1] as f64, pix[2] as f64) / 255.0
            }, 
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerlinNoise {
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
    ranvec: Vec<Vec3>,
}

impl PerlinNoise {
    pub fn default() -> PerlinNoise {
        let point_count = 256;
        let mut rng = fastrand::Rng::new();
        let perm_x = Self::gen_perm(point_count, &mut rng);
        let perm_y = Self::gen_perm(point_count, &mut rng);
        let perm_z = Self::gen_perm(point_count, &mut rng);
        let ranvec: Vec<Vec3> = (0..point_count).map(|_| Vec3::random_vec(-1.0, 1.0, &mut rng).unit()).collect();

        PerlinNoise {
            perm_x,
            perm_y,
            perm_z,
            ranvec,
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p[0] - p[0].floor();
        let v = p[1] - p[1].floor();
        let w = p[2] - p[2].floor();
        let i: i32 = p[0].floor() as i32;
        let j: i32 = p[1].floor() as i32;
        let k: i32 = p[2].floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[vec3!(0.0, 0.0, 0.0); 2]; 2]; 2];
        for (di, a) in c.iter_mut().enumerate() {
            for (dj, b) in a.iter_mut().enumerate() {
                for (dk, val) in b.iter_mut().enumerate() {
                     *val = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize] ^ 
                                       self.perm_y[((j + dj as i32) & 255) as usize] ^ 
                                       self.perm_z[((k + dk as i32) & 255) as usize]) as usize]; 
                }
            }
        }
        Self::tri_inter(&c, u, v, w)
    }

    pub fn turb(&self, p: &Vec3, depth: u8) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn tri_inter(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        let uu = u*u*(3.0 - 2.0 *u);
        let vv = v*v*(3.0-2.0*v);
        let ww = w*w*(3.0-2.0*w);

        for (i, a) in c.iter().enumerate() {
            let fi = i as f64;
            for (j, b) in a.iter().enumerate() {
                let fj = j as f64;
                for (k, val) in b.iter().enumerate() {
                    let fk = k as f64;
                    let weight = vec3!(u - fi, v - fj, w - fk);
                    accum += (fi * uu + (1.0 - fi) * (1.0 - uu)) *
                             (fj * vv + (1.0 - fj) * (1.0 - vv)) *
                             (fk * ww + (1.0 - fk) * (1.0 - ww)) * Vec3::dot(val, &weight);
                }
            }
        }
        accum
    }

    fn gen_perm(c: i32, rng: &mut Rng) -> Vec<i32> {
        let mut ps: Vec<i32> = (0..c).collect();
        rng.shuffle(&mut ps);
        ps
    }
}
