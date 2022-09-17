use crate::rendering::math::{Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    width: u32,
    height: u32,
    aspect_ratio: f32,
    fov_scale: f32,
    focus_dist: f32,
    aperture: f32,
}

impl Camera {
    pub fn new(origin: Vec3, lookat: Vec3, width: u32, height: u32, fov: f32, focus_dist: f32, aperture: f32) -> Self {
        let forward = (lookat - origin).unit();
        let right = Vec3::cross(&forward, &Vec3::new(0.0, 1.0, 0.0)).unit();
        let up = Vec3::cross(&right, &forward).unit();

        let aspect_ratio = width as f32 / height as f32;
        let fov_scale = (fov * std::f32::consts::FRAC_PI_2 / 180.0).tan();

        Camera {
            origin,
            forward,
            right,
            up,
            width,
            height,
            aspect_ratio,
            fov_scale,
            focus_dist,
            aperture,
        }
    }

    pub fn get_ray_to_pixel(&self, pix_x: u32, pix_y: u32, offset_x: f32, offset_y: f32) -> Ray {
        let ndc_x = (pix_x as f32 + 0.5 + offset_x) / (self.width as f32);
        let ndc_y = (pix_y as f32 + 0.5 + offset_y) / (self.height as f32);

        let cam_x = (2.0 * ndc_x - 1.0) * self.aspect_ratio * self.fov_scale;
        let cam_y = (1.0 - 2.0 * ndc_y) * self.fov_scale;

        let rd = Vec3::random_in_unit_disk() * (self.aperture / 2.0);
        let offset = self.right * rd[0] + self.up * rd[1];

        let dir = (self.forward + self.right * cam_x + self.up * cam_y) * self.focus_dist - offset; 

        Ray {
            origin: self.origin + offset,
            dir,
        }
    }
}
