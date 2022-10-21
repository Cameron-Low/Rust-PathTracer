pub mod math;
mod aabb;
mod bvh;
mod materials;
mod objects;
mod camera;
pub mod scene;
mod renderer;

use fastrand::Rng;
pub use renderer::Renderer;

pub struct Options {
   pub pixel_samples: u16,
   pub ray_bounces: u8,
}

#[inline(always)]
pub fn random_f64(rng: &mut Rng, min: f64, max: f64) -> f64 {
    min + rng.f64() * (max - min)
}
