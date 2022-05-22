mod math;
mod materials;
mod objects;
mod camera;
mod ray;

use math::vec3::{Vec3, vec3};
use camera::Camera;
use objects::{Object, Sphere, Triangle};
use materials::Lambertian;

pub fn cast_rays(fb: &mut Vec<u8>, width: u32, height: u32) {
    // Setup the scene
    let cam = Camera::new(vec3!(0.0, 0.0, 0.0), vec3!(0.0, 0.0, -1.0), width, height, 90.0);
    let sph = Sphere { origin: vec3!(0.0, 0.0, -1.2), radius: 0.5, mat: Lambertian {albedo: vec3!(1.0, 0.0, 0.0) }};
    let sph2 = Sphere { origin: vec3!(1.0, 0.0, -1.0), radius: 0.2, mat: Lambertian {albedo: vec3!(0.0, 0.0, 1.0)}};
    let tr = Triangle { v0: vec3!(0.3, 0.2, -0.5), v1: vec3!(0.3, 0.3, -0.5), v2: vec3!(0.4, 0.4, -0.5), mat: Lambertian {albedo: vec3!(0.0, 1.0, 0.0)}};

    let objs: Vec<Box<dyn Object>> = vec![Box::new(sph), Box::new(sph2), Box::new(tr)];
    let light = vec3!(1.0, 0.0, 0.0).unit();
    let sky = vec3!(0.6, 0.6, 0.8);
   
    for x in 0..width {
        for y in 0..height {

            // Skybox setup
            let val = y as f32  / height as f32;
            let skybox = sky * ((1.0 - val) + 0.5 * val);
            let mut missed = true;
            let mut colour = vec3!(0.0, 0.0, 0.0);

            // Start supersampling
            for (offx, offy) in [(0.05, 0.05), (-0.05, 0.05), (0.05, -0.05), (-0.05, -0.05)]{
                missed = true;
                
                let mut ray = cam.get_ray_to_pixel(x, y, offx, offy);

                // Closest intersection
                let mut hit_index = -1;
                for (i, obj) in objs.iter().enumerate() {
                    if obj.intersect(&mut ray) {
                        hit_index = i as i32;
                        missed = false;
                    }
                }

                if hit_index >= 0 {
                    let hit_obj = objs.get(hit_index as usize).unwrap();
                    ray.origin += ray.dir * ray.max;
                    colour += hit_obj.scatter(&mut ray, &light);
                }
            }

            // Calculate new sampled colour
            if missed { 
                colour = skybox;
            } else {
                colour /= 4.0;
            }

            // Set the pixel colour
            fb[(3 * (x + y * width)) as usize] = (colour[0].clamp(0.0, 1.0) * 255.0) as u8;
            fb[(3 * (x + y * width) + 1) as usize] = (colour[1].clamp(0.0, 1.0) * 255.0) as u8;
            fb[(3 * (x + y * width) + 2) as usize] = (colour[2].clamp(0.0, 1.0) * 255.0) as u8;
        }
    }
}

