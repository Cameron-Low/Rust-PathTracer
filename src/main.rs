use fltk::{prelude::*, *};
mod rendering;

fn main() {
    let im_width: u32 = 600;
    let im_height: u32 = 400;

    // Set up the frame buffer which we use for rendering
    let mut fb: Vec<u8> = vec![0u8; im_width as usize * im_height as usize * 3];

    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size((im_width + 60) as i32, im_height as i32)
        .with_label("Raytracer");

    let mut flex = group::Flex::default().size_of(&wind);
    flex.set_type(group::FlexType::Row);
    let mut frame = frame::Frame::default();
    let mut but = button::Button::default().with_label("@>");
    flex.set_size(&mut but, 60);
    flex.end();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    but.set_callback(move |_| {
        rendering::cast_rays(&mut fb, im_width, im_height);
        let img = image::RgbImage::new(&fb, im_width as i32, im_height as i32, enums::ColorDepth::Rgb8).unwrap();
        frame.set_image(Some(img));
        frame.redraw();
    });

    app.run().unwrap();
}
