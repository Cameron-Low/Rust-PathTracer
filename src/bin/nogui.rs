use rendering::{Renderer, Options, scene::earth_scene};
#[allow(unused_imports)]
use rendering::scene::{Scene, weekend_scene, weekend_scene_bouncing, simple_scene, perlin_scene};

fn main() {
    const ASPECT: f64 = 3.0 / 2.0;
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT) as u32;
    const OPTS: Options = Options { pixel_samples: 100, ray_bounces: 20 };
    let scene = earth_scene(WIDTH, HEIGHT);
    let renderer = Renderer::new(WIDTH, HEIGHT, OPTS, scene);

    let mut fb: Vec<u8> = vec![0; WIDTH as usize * HEIGHT as usize * 3];
    let elapsed = renderer.cast_rays(&mut fb);
    println!("This scene took {}ms to render.", elapsed.as_millis());
    
    image::save_buffer("scene.png", &fb, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap()
}
