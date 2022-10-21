use fastrand::Rng;

use crate::{math::{Ray, Vec3}, random_f64, vec3};

pub struct Camera {
    origin: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    inv_width: f64,
    inv_height: f64,
    aspect_ratio: f64,
    fov_scale: f64,
    focus_dist: f64,
    aperture: f64,
    time0: f64,
    time1: f64,
}

pub struct CamSettings {
    pub view_height: u32, 
    pub view_width: u32, 
    pub vfov: f64,
    pub focus_dist: f64,
    pub aperture: f64,
}

impl Camera {
    pub fn new(origin: Vec3, lookat: Vec3, cs: CamSettings, time0: f64, time1: f64) -> Self {
        let forward = (lookat - origin).unit();
        let right = Vec3::cross(&forward, &Vec3::new(0.0, 1.0, 0.0)).unit();
        let up = Vec3::cross(&right, &forward).unit();

        let aspect_ratio = cs.view_width as f64 / cs.view_height as f64;
        let fov_scale = (cs.vfov * std::f64::consts::FRAC_PI_2 / 180.0).tan();

        Camera {
            origin,
            forward,
            right,
            up,
            inv_height: 1.0 / cs.view_height as f64,
            inv_width: 1.0 / cs.view_width as f64,
            aspect_ratio,
            fov_scale,
            focus_dist: cs.focus_dist,
            aperture: cs.aperture,
            time0,
            time1,
        }
    }

    pub fn get_ray_to_pixel(&self, pix_x: u32, pix_y: u32, rng: &mut Rng) -> Ray {
        let offx = rng.f64();
        let offy = rng.f64();
        let ndc_x = (pix_x as f64 + offx) * self.inv_width;
        let ndc_y = (pix_y as f64 + offy) * self.inv_height;

        let cam_x = (2.0 * ndc_x - 1.0) * self.aspect_ratio * self.fov_scale;
        let cam_y = (1.0 - 2.0 * ndc_y) * self.fov_scale;

        let rd = Vec3::random_in_unit_disk(rng) * (self.aperture / 2.0);
        let offset = self.right * rd[0] + self.up * rd[1];

        let dir = (self.forward + self.right * cam_x + self.up * cam_y) * self.focus_dist - offset; 

        let time = random_f64(rng, self.time0, self.time1);
        Ray {
            origin: self.origin + offset,
            dir,
            time, 
            inv_dir: vec3!(1.0 / dir[0], 1.0 / dir[1], 1.0 / dir[2]),
        }
    }
}
