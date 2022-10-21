use fltk::{prelude::*, *};
use rendering::{Renderer, Options, scene::weekend_scene_bouncing};



fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let im_width: u32 = 1200; //1280;
    let im_height: u32 = (im_width as f64 / aspect_ratio) as u32; //780;
    let options = Options { pixel_samples: 100, ray_bounces: 50 };

    // Set up the frame buffer which we use for rendering
    let mut fb: Vec<u8> = vec![0u8; im_width as usize * im_height as usize * 3];
    let scene: rendering::scene::Scene = weekend_scene_bouncing(im_width, im_height);
    let renderer = Renderer::new(im_width, im_height, options, scene);

    // Set up the application window
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size((im_width + 60) as i32, im_height as i32)
        .with_label("Raytracer");

    // Create the frame for the rendered image and the button for rendering
    let mut flex = group::Flex::default().size_of(&wind);
    flex.set_type(group::FlexType::Row);
    let mut frame = frame::Frame::default();
    let mut but = button::Button::default().with_label("@>");
    flex.set_size(&but, 60);
    flex.end();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    // Defining button press behaviour
    but.set_callback(move |_| {
        let elapsed = renderer.cast_rays(&mut fb);
        println!("The scene took {}ms to render", elapsed.as_millis());

        // Construct an image from the framebuffer and display it
        let img = image::RgbImage::new(&fb, im_width as i32, im_height as i32, enums::ColorDepth::Rgb8).unwrap();
        frame.set_image(Some(img));
        frame.redraw();
    });

    app.run().unwrap();
}
