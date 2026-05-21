use rand::RngExt;
use {
    rand::rng,
    sdl3::{
        pixels::Color,
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
    let mut random = rng();
    let (width, height): (u32, u32) = image_dimensions("assets/bg_image.png").unwrap();
    let sdl3_context = sdl3::init().unwrap();
    let video_subsystem = sdl3_context.video().unwrap();
    let ttf3_context = sdl3::ttf::init().unwrap();
    let window = video_subsystem.window("Catch the Apple", width, height)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let main_font = ttf3_context.load_font("assets/ARIAL.TTF", 24f32).unwrap();
    let bg_img = texture_creator.create_texture_from_surface(Surface::from_file("assets/bg_image.png").unwrap()).unwrap();
    let cat_surface = Surface::from_file("assets/spr_cat.png").unwrap();
    let apple_surface = Surface::from_file("assets/spr_apple.png").unwrap();
    let cat_img = texture_creator.create_texture_from_surface(&cat_surface).unwrap();
    let apple_img = texture_creator.create_texture_from_surface(&apple_surface).unwrap();
    let (mut cat_x, cat_y): (i32, i32) = (0, height as i32 - cat_surface.height() as i32);
    let (mut apple_x, mut apple_y): (i32, i32) = (random.random_range(0..=width as i32 - apple_surface.width() as i32), 0);
    let mut cat_speed = 0;
    let apple_speed = 5;
    let mut time = 120 * 60;
    let mut score = 0;
    let mut can_collide = true;
    let mut game_over = false;
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
        let cat_rect = Rect::new(cat_x, cat_y, cat_surface.width(), cat_surface.height());
        let apple_rect = Rect::new(apple_x, apple_y, apple_surface.width(), apple_surface.height());
        if !game_over {
            cat_x += cat_speed;
            apple_y += apple_speed;
            let dx = cat_x as f64 - apple_x as f64;
            let dy = cat_y as f64 - apple_y as f64;
            let distance = (dx * dx + dy * dy).sqrt();
            time -= 1;
            if apple_y > height as i32 {
                apple_y = 0;
                apple_x = random.random_range(0..=width as i32 - apple_surface.width() as i32);
            }
            if can_collide && distance < 60f64 {
                score += 1;
                apple_y = 0;
                apple_x = random.random_range(0..=width as i32 - apple_surface.width() as i32);
                can_collide = false;
            }
            if apple_y > 200 {can_collide = true;}
            if time == 0 {game_over = true;}
            canvas.copy(&bg_img, None, None).unwrap();
            if !game_over {
                canvas.copy(&cat_img, None, cat_rect).unwrap();
                canvas.copy(&apple_img, None, apple_rect).unwrap();
            }
            let score_render = main_font.render(&format!("Score: {}", score)).solid(Color::WHITE).unwrap();
            let time_render = main_font.render(&format!("Time: {:.0}", time as f64 / 60f64)).solid(Color::WHITE).unwrap();
            canvas.copy(&texture_creator.create_texture_from_surface(&score_render).unwrap(), None, Rect::new(0, 0, score_render.width(), score_render.height())).unwrap();
            canvas.copy(&texture_creator.create_texture_from_surface(&time_render).unwrap(), None, Rect::new(0, 24, time_render.width(), time_render.height())).unwrap();
        }
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
