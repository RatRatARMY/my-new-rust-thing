use {
    sdl3::{
        event::Event,
        keyboard::Keycode,
        image::LoadSurface,
        surface::Surface,
        rect::Rect
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
    let texture_creator = canvas.texture_creator();
    let bg_img = texture_creator.create_texture_from_surface(Surface::from_file("assets/bg_image.png").unwrap()).unwrap();
    let cat_surface = Surface::from_file("assets/spr_cat.png").unwrap();
    let cat_img = texture_creator.create_texture_from_surface(&cat_surface).unwrap();
    let (mut cat_x, cat_y): (i32, i32) = (0, height as i32 - cat_surface.height() as i32);
    let mut cat_speed = 0;
    canvas.present();
    let mut events = sdl3_context.event_pump().unwrap();
    'game_loop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'game_loop;
                },
                Event::KeyDown {keycode: Some(Keycode::Left), ..} |
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    cat_speed = -5;
                },
                Event::KeyDown {keycode: Some(Keycode::Right), ..} |
                Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    cat_speed = 5;
                },
                Event::KeyUp {..} => {
                    cat_speed = 0;
                }
                _ => {}
            }
        }
        cat_x += cat_speed;
        let cat_rect = Rect::new(cat_x, cat_y, cat_surface.width(), cat_surface.height());
        canvas.copy(&bg_img, None, None).unwrap();
        canvas.copy(&cat_img, None, cat_rect).unwrap();
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
