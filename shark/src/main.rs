use {
    image::image_dimensions,
    sdl3::{
        event::Event,
        keyboard::Keycode,
        surface::Surface,
        image::LoadSurface,
        rect::Rect
    },
    std::{thread::sleep, time::Duration}
};
fn main() {
    let (width, height): (u32, u32) = image_dimensions("assets/bg_image_2.png").unwrap();
    let sdl3_context = sdl3::init().unwrap();
    let video_subsystem = sdl3_context.video().unwrap();
    let window = video_subsystem.window("Shark game", width, height)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let bg_img = texture_creator.create_texture_from_surface(Surface::from_file("assets/bg_image_2.png").unwrap()).unwrap();
    canvas.present();
    let mut events = sdl3_context.event_pump().unwrap();
    'game_loop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'game_loop;
                },
                _ => {}
            }
        }
        canvas.copy(&bg_img, None, None).unwrap();
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
