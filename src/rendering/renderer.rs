use rand::prelude::*;
use super::Options;
use super::materials::Material;
use super::objects::{Intersection, Hittable};
use super::scene::{Scene, weekend_scene};
use super::math::{Ray, Vec3, vec3};

use rayon::prelude::*;
pub struct Renderer {
    scene: Scene,
    width: u32,
    height: u32,
    options: Options,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Renderer {
        let scene = weekend_scene(width, height);
        let options = Options { pixel_samples: 100, ray_bounces: 50 };

        Renderer {
            scene,
            width,
            height,
            options,
        }
    }

    pub fn cast_rays(&self, fb: &mut [u8]) -> std::time::Duration {
        // Very basic timing setup
        use std::time::Instant;
        let now = Instant::now();

        #[cfg(feature="multithreading")]
        {
        let chunks: Vec<(usize, &mut [u8])> = fb.chunks_mut(3).enumerate().collect();
        chunks.into_par_iter().for_each(|(i, chunk)| {
            let x = i as u32 % self.width;
            let y = i as u32 / self.width;

            let colour = self.calc_pixel_colour(x, y);
            
            // Set the pixel colour
            for v in 0..3 {
                chunk[v] = (colour[v].sqrt().clamp(0.0, 1.0) * 255.0) as u8;
            }
        });
        }
        #[cfg(not(feature="multithreading"))]
        {
        for x in 0..self.width {
            for y in 0..self.height {
                let fb_offset: usize = 3 * (x + y * self.width) as usize;
                let colour = self.calc_pixel_colour(x, y);

                // Set the pixel colour
                for i in 0..3 {
                    fb[fb_offset + i] = (colour[i].sqrt().clamp(0.0, 1.0) * 255.0) as u8;
                }
            }
        }
        }

        now.elapsed()
    }

    fn calc_pixel_colour(&self, x: u32, y: u32) -> Vec3 {
        let mut colour = vec3!(0.0, 0.0, 0.0);

        let mut rng = rand::thread_rng();
        for _ in 0..self.options.pixel_samples {
            // Send a ray into the scene
            let offx = rng.gen::<f32>() - 0.5;
            let offy = rng.gen::<f32>() - 0.5;
            let mut ray = self.scene.cam.get_ray_to_pixel(x, y, offx, offy);
            colour += self.calc_ray_colour(&mut ray, self.options.ray_bounces);
        }

        colour /= self.options.pixel_samples as f32;
        colour
    }


    fn calc_ray_colour(&self, ray: &mut Ray, depth: u8) -> Vec3 {
        // Check if the we have reached the maximum depth
        if depth == 0 {
            return vec3!(0.0, 0.0, 0.0);
        }
        
        // Find the closest intersecting object
        let mut intersection = Intersection { min: 0.001, max: std::f32::INFINITY, obj: None };
        self.scene.objs.iter().for_each(|obj| {
            obj.intersect(ray, &mut intersection);
        });

        // If we hit something, compute the next ray and it's colour
        if let Some(hit_obj) = intersection.obj {
            // Move the ray to the intersection point and ready it for scattering
            ray.move_along(intersection.max);
            let normal = hit_obj.compute_normal(ray);
            

            let (absorbed, attenuation) = match hit_obj.get_mat() {
                Material::Lambertian { albedo } => {
                    ray.dir = normal + Vec3::random_unit_vec();
                    if Vec3::close_to_zero(ray.dir) {
                        ray.dir = normal;
                    }
                    (false, albedo)
                },
                Material::Metal { albedo, fuzz } => {
                    ray.dir = ray.dir.unit();
                    ray.reflect(normal);
                    ray.dir += Vec3::random_in_unit_sphere() * fuzz;
                    (Vec3::dot(&ray.dir, &normal) < 0.0, albedo)
                },
                Material::Dielectric { mut ir } => {
                    let albedo = vec3!(1.0, 1.0, 1.0);

                    ray.dir = ray.dir.unit();
                    let mut cos_thetai = Vec3::dot(&ray.dir, &normal).clamp(-1.0, 1.0);
                    let mut n = normal;

                    // Are we entering the medium
                    if cos_thetai < 0.0 {
                        cos_thetai = -cos_thetai;
                        ir = 1.0 / ir;
                    } else {
                        n = -normal;
                    }
                    let sin_theta_sq = ir * ir * (1.0 - cos_thetai * cos_thetai);
                    let cos_thetat = (1.0 - sin_theta_sq).sqrt();

                    //Check for total internal reflection and viewing angles
                    if sin_theta_sq > 1.0 || Ray::schlick(ir, cos_thetai) > random::<f32>() {
                        ray.reflect(n);
                    } else {
                        ray.refract(n, ir, cos_thetai, cos_thetat);
                    }

                    (false, albedo)
                }
            };
            
            if !absorbed {
                return self.calc_ray_colour(ray, depth - 1) * attenuation;
            }
            
            return vec3!(0.0, 0.0, 0.0);
        }

        let t = 0.5 * (ray.dir[1] + 1.0);
        vec3!(1.0, 1.0, 1.0) * (1.0 - t) + self.scene.skybox_colour * t
    }
}

