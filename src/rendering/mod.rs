mod math;
mod materials;
mod objects;
mod camera;
mod scene;
mod renderer;

pub use renderer::Renderer;

pub struct Options {
   pub pixel_samples: u16,
   pub ray_bounces: u8,
}
