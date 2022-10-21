use crate::{
    camera::Camera,
    materials::Material,
    math::{Vec3, vec3}, bvh::{BvhTree, LinearBvh},
};

use super::{objects::Elem, camera::CamSettings, materials::{Texture, PerlinNoise}};

pub struct Scene {
    pub cam: Camera,
    //pub bvh: BvhTree,
    pub bvh: LinearBvh,
    pub skybox_colour: Vec3,
}

#[allow(unused)]
pub fn simple_scene(width: u32, height: u32) -> Scene {
    let mut objs: Vec<Elem> = vec![];
    let ground_material = Material::Lambertian {
        albedo: Texture::Solid { colour: vec3!(0.5, 0.5, 0.5) },
    };
    objs.push(Elem::Sphere { 
        origin: vec3!(0.0, -100.5, -1.0), 
        radius: 100.0,
        mat: ground_material,
    });
    let material2 = Material::Lambertian {
        albedo: Texture::Solid { colour: vec3!(0.5, 0.5, 0.5) },
    };
    objs.push(Elem::Sphere {
        origin: vec3!(0.0, 0.0, -1.0),
        radius: 0.5,
        mat: material2,
    });

    let from = vec3!(0.0, 0.0, 0.0);
    let at = vec3!(0.0, -0.5, -1.0);

    let cs = CamSettings {
        view_width: width,
        view_height: height,
        vfov: 40.0,
        focus_dist: 1.0,
        aperture: 0.0,
    };
    let cam = Camera::new(from, at, cs, 0.0, 0.0);
    Scene {
        cam, 
        skybox_colour: vec3!(0.5, 0.7, 1.0),
        bvh: LinearBvh::new(BvhTree::new(objs, 0.0, 0.0)),
        //bvh: BvhTree::new(objs, 0.0, 0.0),
    }
}

#[allow(unused)]
pub fn three_spheres_scene(width: u32, height: u32) -> Scene {
    let mut objs: Vec<Elem> = vec![];
    let ground_mat = Material::Lambertian { albedo: Texture::Solid { colour: vec3!(0.8, 0.8, 0.0) } };
    let mat1 = Material::Lambertian { albedo: Texture::Solid { colour: vec3!(0.1, 0.2, 0.5) } };
    let mat2 = Material::Dielectric { ir: 1.5 };
    let mat3 = Material::Metal { albedo: Texture::Solid { colour: vec3!(0.8, 0.6, 0.2) }, fuzz: 0.0 };
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
        mat: mat2.clone(),
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

    let cs = CamSettings {
        view_width: width,
        view_height: height,
        vfov: 20.0,
        focus_dist: 1.0,
        aperture: 0.0,
    };
    let cam = Camera::new(from, at, cs, 0.0, 0.0);
    Scene {
        cam, 
        skybox_colour: vec3!(0.5, 0.7, 1.0),
        bvh: LinearBvh::new(BvhTree::new(objs, 0.0, 0.0)),
        //bvh: BvhTree::new(objs, 0.0, 0.0),
    }
}

#[allow(unused)]
pub fn weekend_scene(width: u32, height: u32) -> Scene {
    let mut rng = fastrand::Rng::new();
    rng.seed(10);
    let mut objs: Vec<Elem> = vec![];
    let ground_material = Material::Lambertian { albedo: Texture::Solid { colour: vec3!(0.5, 0.5, 0.5) } };
    objs.push(Elem::Sphere { 
        origin: vec3!(0.0, -1000.0, 0.0), 
        radius: 1000.0,
        mat: ground_material,
    });

    for a in -11..11 {
        for b in -11..11 {  
            let choose_mat = rng.f64();
            let center = vec3!(a as f64 + 0.9*rng.f64(), 0.2, b as f64 + 0.9*rng.f64());

            if (center - vec3!(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Texture::Solid { colour: Vec3::random_vec(0.0, 1.0, &mut rng) * Vec3::random_vec(0.0, 1.0, &mut rng) }; 
                    sphere_material = Material::Lambertian { albedo };
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Texture::Solid { colour: Vec3::random_vec(0.5, 1.0, &mut rng).unit() };
                    let fuzz = rng.f64() / 2.0;
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

    let material2 = Material::Lambertian{ albedo: Texture::Solid { colour: vec3!(0.4, 0.2, 0.1) } };
    objs.push(Elem::Sphere {
        origin: vec3!(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material2,
    });

    let material3 = Material::Metal{ albedo: Texture::Solid { colour: vec3!(0.7, 0.6, 0.5) }, fuzz: 0.0 };
    objs.push(Elem::Sphere {
        origin: vec3!(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material3,
    });

    let from = vec3!(13.0, 2.0, 3.0);
    let at = vec3!(0.0, 0.0, 0.0);

    let cs = CamSettings {
        view_width: width,
        view_height: height,
        vfov: 20.0,
        focus_dist: 10.0,
        aperture: 0.1,
    };
    let cam = Camera::new(from, at, cs, 0.0, 0.0);
    Scene {
        cam, 
        skybox_colour: vec3!(0.5, 0.7, 1.0),
        bvh: LinearBvh::new(BvhTree::new(objs, 0.0, 0.0)),
        //bvh: BvhTree::new(objs, 0.0, 0.0),
    }
}

#[allow(unused)]
pub fn weekend_scene_bouncing(width: u32, height: u32) -> Scene {
    let mut rng = fastrand::Rng::new();
    rng.seed(10);
    let mut objs: Vec<Elem> = vec![];
    let ground_material = Material::Lambertian { 
        albedo: Texture::Checker {
            odd: Box::new(Texture::Solid { colour: vec3!(0.2, 0.3, 0.1) }),
            even: Box::new(Texture::Solid { colour: vec3!(0.9, 0.9, 0.9) }),
        } 
    };
    objs.push(Elem::Sphere { 
        origin: vec3!(0.0, -1000.0, 0.0), 
        radius: 1000.0,
        mat: ground_material,
    });

    for a in -11..11 {
        for b in -11..11 {  
            let choose_mat = rng.f64();
            let center = vec3!(a as f64 + 0.9*rng.f64(), 0.2, b as f64 + 0.9*rng.f64());

            if (center - vec3!(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Texture::Solid { colour: Vec3::random_vec(0.0, 1.0, &mut rng) * Vec3::random_vec(0.0, 1.0, &mut rng) }; 
                    sphere_material = Material::Lambertian { albedo };
                    let center2 = center + vec3!(0.0, rng.f64() / 2.0, 0.0);
                    objs.push(Elem::MovingSphere { 
                        origin0: center, 
                        origin1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2, 
                        mat: sphere_material,
                    });
                } else {
                    if choose_mat < 0.95 {
                        // metal
                        let albedo = Texture::Solid { colour: Vec3::random_vec(0.5, 1.0, &mut rng).unit() };
                        let fuzz = rng.f64() / 2.0;
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
    }

    let material1 = Material::Dielectric { ir: 1.5 };
    objs.push(Elem::Sphere {
        origin: vec3!(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: material1,
    });

    let material2 = Material::Lambertian{ albedo: Texture::Solid { colour: vec3!(0.4, 0.2, 0.1) } };
    objs.push(Elem::Sphere {
        origin: vec3!(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material2,
    });

    let material3 = Material::Metal{ albedo: Texture::Solid { colour: vec3!(0.7, 0.6, 0.5) }, fuzz: 0.0 };
    objs.push(Elem::Sphere {
        origin: vec3!(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material3,
    });

    let from = vec3!(13.0, 2.0, 3.0);
    let at = vec3!(0.0, 0.0, 0.0);

    let cs = CamSettings {
        view_width: width,
        view_height: height,
        vfov: 20.0,
        focus_dist: 10.0,
        aperture: 0.1,
    };
    let cam = Camera::new(from, at, cs, 0.0, 1.0);
    Scene {
        cam, 
        skybox_colour: vec3!(0.5, 0.7, 1.0),
        bvh: LinearBvh::new(BvhTree::new(objs, 0.0, 1.0)),
        //bvh: BvhTree::new(objs, 0.0, 1.0),
    }
}

#[allow(unused)]
pub fn perlin_scene(width: u32, height: u32) -> Scene {
    let mut objs: Vec<Elem> = vec![];
    let perlin_mat = Material::Lambertian { 
        albedo: Texture::Perlin { noise: PerlinNoise::default(), scale: 4.0 },
    };
    objs.push(Elem::Sphere { 
        origin: vec3!(0.0, -1000.0, 0.0), 
        radius: 1000.0,
        mat: perlin_mat.clone(),
    });
    objs.push(Elem::Sphere { 
        origin: vec3!(0.0, 2.0, 0.0), 
        radius: 2.0,
        mat: perlin_mat,
    });
    let from = vec3!(13.0, 2.0, 3.0);
    let at = vec3!(0.0, 0.0, 0.0);

    let cs = CamSettings {
        view_width: width,
        view_height: height,
        vfov: 20.0,
        focus_dist: 10.0,
        aperture: 0.1,
    };
    let cam = Camera::new(from, at, cs, 0.0, 1.0);
    Scene {
        cam, 
        skybox_colour: vec3!(0.5, 0.7, 1.0),
        bvh: LinearBvh::new(BvhTree::new(objs, 0.0, 0.0)),
        //bvh: BvhTree::new(objs, 0.0, 0.0),
    }
}

#[allow(unused)]
pub fn earth_scene(width: u32, height: u32) -> Scene {
    let mut objs: Vec<Elem> = vec![Elem::Sphere { 
        origin: vec3!(0.0, 0.0, 0.0), 
        radius: 2.0,
        mat: Material::Lambertian { albedo: Texture::Image { img: image::open("earthmap.jpg").unwrap().to_rgb8() } },
    }];
    let from = vec3!(13.0, 2.0, 3.0);
    let at = vec3!(0.0, 0.0, 0.0);

    let cs = CamSettings {
        view_width: width,
        view_height: height,
        vfov: 20.0,
        focus_dist: 1.0,
        aperture: 0.0,
    };
    let cam = Camera::new(from, at, cs, 0.0, 0.0);
    Scene {
        cam, 
        skybox_colour: vec3!(0.5, 0.7, 1.0),
        bvh: LinearBvh::new(BvhTree::new(objs, 0.0, 0.0)),
        //bvh: BvhTree::new(objs, 0.0, 0.0),
    }
}
