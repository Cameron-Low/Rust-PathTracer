use rand::random;

use crate::rendering::{
    camera::Camera,
    materials::Material,
    math::{Vec3, vec3},
};

use super::objects::Elem;


pub struct Scene {
    pub cam: Camera,
    pub objs: Vec<Elem>,
    pub skybox_colour: Vec3,
}

pub fn simple_scene(width: u32, height: u32) -> Scene {
    let mut objs: Vec<Elem> = vec![];
    let ground_material = Material::Lambertian { albedo: vec3!(0.5, 0.5, 0.5) };
    objs.push(Elem::Sphere { 
        origin: vec3!(0.0, -100.5, -1.0), 
        radius: 100.0,
        mat: ground_material,
    });
    let material2 = Material::Lambertian{ albedo: vec3!(0.5, 0.5, 0.5) };
    objs.push(Elem::Sphere {
        origin: vec3!(0.0, 0.0, -1.0),
        radius: 0.5,
        mat: material2,
    });

    let from = vec3!(0.0, 0.0, 0.0);
    let at = vec3!(0.0, 0.0, -1.0);

    let cam = Camera::new(from, at, width, height, 90.0, 1.0, 0.0);
    Scene {
        cam, 
        objs, 
        skybox_colour: vec3!(0.5, 0.7, 1.0),
    }
}

pub fn three_spheres_scene(width: u32, height: u32) -> Scene {
    let mut objs: Vec<Elem> = vec![];
    let ground_mat = Material::Lambertian { albedo: vec3!(0.8, 0.8, 0.0) };
    let mat1 = Material::Lambertian { albedo: vec3!(0.1, 0.2, 0.5) };
    let mat2 = Material::Dielectric { ir: 1.5 };
    let mat3 = Material::Metal { albedo: vec3!(0.8, 0.6, 0.2), fuzz: 0.0 };
    objs.push(Elem::Sphere { 
        origin: vec3!(0.0, -100.5, -1.0), 
        radius: 100.0,
        mat: ground_mat,
    });
    objs.push(Elem::Sphere {
        origin: vec3!(0.0, 0.0, -1.0),
        radius: 0.5,
        mat: mat1,
    });
    objs.push(Elem::Sphere {
        origin: vec3!(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat: mat2,
    });
    objs.push(Elem::Sphere {
        origin: vec3!(-1.0, 0.0, -1.0),
        radius: -0.45,
        mat: mat2,
    });
    objs.push(Elem::Sphere {
        origin: vec3!(1.0, 0.0, -1.0),
        radius: 0.5,
        mat: mat3,
    });

    let from = vec3!(-2.0, 2.0, 1.0);
    let at = vec3!(0.0, 0.0, -1.0);

    let cam = Camera::new(from, at, width, height, 20.0, 1.0, 0.0);
    Scene {
        cam, 
        objs, 
        skybox_colour: vec3!(0.5, 0.7, 1.0),
    }
}

pub fn weekend_scene(width: u32, height: u32) -> Scene {
    let mut objs: Vec<Elem> = vec![];
    let ground_material = Material::Lambertian { albedo: vec3!(0.5, 0.5, 0.5) };
    objs.push(Elem::Sphere { 
        origin: vec3!(0.0, -1000.0, 0.0), 
        radius: 1000.0,
        mat: ground_material,
    });

    for a in -11..11 {
        for b in -11..11 {  
            let choose_mat = random::<f32>();
            let center = vec3!(a as f32 + 0.9*random::<f32>(), 0.2, b as f32 + 0.9*random::<f32>());

            if (center - vec3!(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random_vec(0.0, 1.0) * Vec3::random_vec(0.0, 1.0); 
                    sphere_material = Material::Lambertian { albedo };
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_vec(0.5, 1.0).unit();
                    let fuzz = random::<f32>() / 2.0;
                    sphere_material = Material::Metal { albedo, fuzz };
                } else {
                    // glass
                    sphere_material = Material::Dielectric { ir: 1.5 };
                }
                objs.push(Elem::Sphere { 
                    origin: center, 
                    radius: 0.2, 
                    mat: sphere_material,
                });
            }
        }
    }

    let material1 = Material::Dielectric { ir: 1.5 };
    objs.push(Elem::Sphere {
        origin: vec3!(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: material1,
    });

    let material2 = Material::Lambertian{ albedo: vec3!(0.4, 0.2, 0.1) };
    objs.push(Elem::Sphere {
        origin: vec3!(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material2,
    });

    let material3 = Material::Metal{ albedo: vec3!(0.7, 0.6, 0.5), fuzz: 0.0 };
    objs.push(Elem::Sphere {
        origin: vec3!(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material3,
    });

    let from = vec3!(13.0, 2.0, 3.0);
    let at = vec3!(0.0, 0.0, 0.0);

    let cam = Camera::new(from, at, width, height, 20.0, 10.0, 0.1);
    Scene {
        cam, 
        objs, 
        skybox_colour: vec3!(0.5, 0.7, 1.0),
    }
}
