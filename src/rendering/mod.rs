mod math;
mod materials;
mod objects;
mod camera;
mod scene;

use rand::prelude::*;
use scene::{Scene, weekend_scene};
use math::{Vec3, vec3};

use self::math::Ray;

pub fn cast_rays(fb: &mut [u8], width: u32, height: u32) {
    // Setup the scene
    let scene = weekend_scene(width, height);
   
    // Very basic timing setup
    use std::time::Instant;
    let now = Instant::now();

    #[cfg(feature="multithreading")]
    {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(16).build().unwrap();

    pool.scope(|s| {
        for (i, chunk) in fb.chunks_mut(3).enumerate() {
            let x = i as u32 % width;
            let y = i as u32 / width;

            let scene = &scene;
            s.spawn(move |_| {
                let colour = calc_pixel_colour(scene, x, y);
                
                // Set the pixel colour
                for v in 0..3 {
                    chunk[v] = (colour[v].sqrt().clamp(0.0, 1.0) * 256.0) as u8;
                }
            });
        }
    });
    }

    /* Speedy single threaded
     * NEEDED modifying due to slice ownership
     */
    #[cfg(not(feature="multithreading"))]
    {
    for x in 0..width {
        for y in 0..height {
            let fb_offset: usize = 3 * (x + y * width) as usize;
            let colour = calc_pixel_colour(&scene, x, y);

            // Set the pixel colour
            for i in 0..3 {
                fb[fb_offset + i] = (colour[i].sqrt().clamp(0.0, 1.0) * 256.0) as u8;
            }
        }
    }
    }

    let elapsed = now.elapsed();
    println!("The scene took {}ms to render", elapsed.as_millis());
}

fn calc_pixel_colour(scene: &Scene, x: u32, y: u32) -> Vec3 {
    let mut colour = vec3!(0.0, 0.0, 0.0);
    let samples = 25;
    let ray_bounces = 10;

    let mut rng = rand::thread_rng();
    for _ in 0..samples {
        // Send a ray into the scene
        let offx = rng.gen::<f32>() - 0.5;
        let offy = rng.gen::<f32>() - 0.5;
        let mut ray = scene.cam.get_ray_to_pixel(x, y, offx, offy);
        colour += calc_ray_colour(scene, &mut ray, ray_bounces);
    }

    colour /= samples as f32;
    colour
}


fn calc_ray_colour(scene: &Scene, ray: &mut Ray, depth: u32) -> Vec3 {
    // Check if the we have reached the maximum depth
    if depth == 0 {
        return vec3!(0.0, 0.0, 0.0);
    }

    // Find the closest intersecting object
    let mut hit_index = None;
    for (i, obj) in scene.objs.iter().enumerate() {
        if obj.intersect(ray) {
            hit_index = Some(i);
        }
    }

    // If we hit something, compute the next ray and it's colour
    if let Some(i) = hit_index {
        // Get the object
        let hit_obj = scene.objs.get(i).unwrap();

        
        // Move the ray to the intersection point and ready it for scattering
        ray.origin += ray.dir * ray.max;
        ray.min = 0.001;
        ray.max = std::f32::INFINITY;
        
        
        // Compute the attenuation
        let (absorbed, attenuation) = hit_obj.scatter(ray);
        
        if !absorbed {
            return calc_ray_colour(scene, ray, depth - 1) * attenuation;
        }
        
        return vec3!(0.0, 0.0, 0.0);
    }

    let t = 0.5 * (ray.dir[1] + 1.0);
    vec3!(1.0, 1.0, 1.0) * (1.0 - t) + scene.skybox_colour * t
}

