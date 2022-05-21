mod math;
mod materials;
mod objects;
mod camera;
mod ray;
mod hit_record;

use math::vec3::Vec3;
use camera::Camera;
use objects::{Object, Sphere}; //, Triangle};
use materials::Lambertian;

pub fn cast_rays(fb: &mut Vec<u8>, width: u32, height: u32) {
    let cam = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), width, height, 90.0);
    let sph = Sphere { origin: Vec3::new(0.0, 0.0, -1.0), radius: 0.5, mat: Lambertian { albedo: Vec3::new(1.0, 0.0, 0.0) }};
    let sph2 = Sphere { origin: Vec3::new(1.0, 0.0, -1.0), radius: 0.2, mat: Lambertian { albedo: Vec3::new(0.0, 0.0, 1.0)}};
    //let tr = Triangle { v0: Vec3::new(0.0, 0.2, -0.5), v1: Vec3::new(0.0, 0.1, -0.5), v2: Vec3::new(0.1, 0.3, -0.5), mat: Lambertian {albedo: Vec3::new(1.0, 1.0, 0.0) }};

    let mut objs: Vec<Box<dyn Object>> = Vec::new();
    objs.push(Box::new(sph));
    objs.push(Box::new(sph2));
    //objs.push(Box::new(tr));

    let light = Vec3::new(1.0, 0.0, 0.0).unit();

    let sky = Vec3::new(0.6, 0.6, 0.8);
   
    for x in 0..width {
        for y in 0..height {
            let mut ray = cam.get_ray_to_pixel(x, y);

            let val = y as f32  / height as f32;
            let mut colour = sky * (1.0 - val) + sky * 0.5 * val;

            // Closest intersection
            let mut hit_index = -1;
            for (i, obj) in objs.iter().enumerate() {
                if obj.intersect(&mut ray) {
                    hit_index = i as i32;
                }
            }

            if hit_index >= 0 {
                let hit_obj = objs.get(hit_index as usize).unwrap();
                ray.origin = ray.origin + ray.dir * ray.max;
                colour = hit_obj.scatter(&mut ray, &light);
            }

                //.compute_shading(&mut ray, &mut rec);
            
            
            // Shading
            //objs.iter().for_each(|x| x.intersect(&mut ray, &mut rec));

            fb[(3 * (x + y * width)) as usize] = (colour.elems[0].clamp(0.0, 1.0) * 255.0) as u8;
            fb[(3 * (x + y * width) + 1) as usize] = (colour.elems[1].clamp(0.0, 1.0) * 255.0) as u8;
            fb[(3 * (x + y * width) + 2) as usize] = (colour.elems[2].clamp(0.0, 1.0) * 255.0) as u8;
        }
    }
}

