use crate::rendering::ray::Ray;
use crate::rendering::math::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    width: u32,
    height: u32,
    aspect_ratio: f32,
    fov_scale: f32
}

impl Camera {
    pub fn new(origin: Vec3, lookat: Vec3, width: u32, height: u32, fov: f32) -> Self {
        let forward = (&lookat - &origin).unit();
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
            fov_scale
        }
    }

    pub fn get_ray_to_pixel(&self, pix_x: u32, pix_y: u32) -> Ray {
        let ndc_x = (pix_x as f32 + 0.5) / (self.width as f32);
        let ndc_y = (pix_y as f32 + 0.5) / (self.height as f32);

        let cam_x = (2.0 * ndc_x - 1.0) * self.aspect_ratio * self.fov_scale;
        let cam_y = (1.0 - 2.0 * ndc_y) * self.fov_scale;

        let dir = (self.forward + self.right * cam_x + self.up * cam_y).unit(); 

        Ray {
            origin: self.origin,
            dir,
            min: 0.0001,
            max: std::f32::INFINITY
        }
    }
}

// 
