use {
    sdl3::{
        pixels::Color,
        event::Event,
        keyboard::Keycode
    },
    image::image_dimensions,
    std::{time::Duration, thread::sleep}
};
fn main() {
    let (width, height): (u32, u32) = image_dimensions("assets/bg_image.png").unwrap();
    let sdl3_context = sdl3::init().unwrap();
    let video_subsystem = sdl3_context.video().unwrap();
    let window = video_subsystem.window("Catch the Apple", width, height)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();
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
        // Phần còn lại của game loop sẽ ở đây...

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
