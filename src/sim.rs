use std::{
    thread,
    time::{Duration, Instant},
};

use rand::Rng;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::TextureCreator,
    video::WindowContext, Sdl,
};

use crate::{
    cars::Traffic,
    lanes,
    types::{Airt, Dimensions, Speed},
};

pub fn simulate(traffic: &mut Traffic) {
    let (sdl_context, mut canvas, dimensions) = setup();
    let texture_creator = canvas.texture_creator();
    let texture = create_speckled_texture(
        &texture_creator,
        dimensions.window_width as u32,
        dimensions.window_height as u32,
        &mut canvas,
    );
    run(&sdl_context, &mut canvas, &dimensions, traffic, &texture);
}

fn setup() -> (
    sdl2::Sdl,
    sdl2::render::Canvas<sdl2::video::Window>,
    Dimensions,
) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (ddpi, hdpi, vdpi) = video_subsystem.display_dpi(0).unwrap();

    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    let screen_width = display_mode.w;
    let screen_height = display_mode.h;

    let window_width = screen_width as f32 * 0.6 * hdpi / 133.0;
    let window_height = screen_height as f32 * 0.8 * vdpi / 139.0;

    let inches = f32::sqrt(window_width * window_width + window_height * window_height);

    let lane_width = (16.0 * inches * ddpi / (1024.3201 * 134.4)) as i32;
    let speed = Speed {
        fast: lane_width / 2 as i32,
        default: lane_width * 3 / 8 as i32,
        slow: lane_width / 4 as i32,
    };

    let window_width = window_width as i32;
    let window_height = window_height as i32;

    let dimensions = Dimensions {
        window_width,
        window_height,
        half_width: window_width / 2,
        half_height: window_height / 2,
        lane_width: 16,
        speed,
    };

    let window = video_subsystem
        .window(
            "Smart Road",
            dimensions.window_width as u32,
            dimensions.window_height as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas
        .set_logical_size(
            dimensions.window_width as u32,
            dimensions.window_height as u32,
        )
        .unwrap();

    (sdl_context, canvas, dimensions)
}

fn run(
    sdl_context: &Sdl,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    dimensions: &Dimensions,
    traffic: &mut Traffic,
    texture: &sdl2::render::Texture,
) {
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_keypress_time = Instant::now();
    let keypress_interval = Duration::from_millis(128);
    let mut start_time = Instant::now();

    'running: loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        if elapsed < Duration::from_millis(16) {
            continue;
        }
        start_time = now;

        traffic.update(&dimensions);
        render(canvas, &dimensions, &traffic, &texture);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let now = Instant::now();
                    if now.duration_since(last_keypress_time) <= keypress_interval {
                        continue;
                    }
                    match keycode {
                        Keycode::Up => {
                            traffic.push(Airt::Up, &dimensions);
                        }
                        Keycode::Down => {
                            traffic.push(Airt::Down, &dimensions);
                        }
                        Keycode::Left => {
                            traffic.push(Airt::Left, &dimensions);
                        }
                        Keycode::Right => {
                            traffic.push(Airt::Right, &dimensions);
                        }
                        Keycode::R => {
                            traffic.push_random(&dimensions);
                        }
                        _ => {}
                    }
                    last_keypress_time = now;
                }
                _ => {}
            }
        }
    }

    // To ward against closing the stats window if you press escape for too long.
    thread::sleep(Duration::from_millis(128));
}

fn render(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    dimensions: &Dimensions,
    traffic: &Traffic,
    texture: &sdl2::render::Texture,
) {
    canvas.copy(&texture, None, None).unwrap();
    lanes::draw(canvas, &dimensions);
    traffic.draw(canvas, &dimensions);
    canvas.present();
}

fn create_speckled_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    width: u32,
    height: u32,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> sdl2::render::Texture<'a> {
    // Create a texture target
    let mut texture = texture_creator
        .create_texture_target(None, width, height)
        .expect("Failed to create texture target");

    // Render directly to the texture
    canvas
        .with_texture_canvas(&mut texture, |texture_canvas| {
            // Set base color
            texture_canvas.set_draw_color(Color::RGB(128, 128, 128));
            texture_canvas.clear();

            let mut rng = rand::thread_rng();
            for _ in 0..50000 {
                // Speckle density
                let x = rng.gen_range(0..width as i32);
                let y = rng.gen_range(0..height as i32);
                let size = rng.gen_range(1..4);
                let gray = rng.gen_range(144..255);
                texture_canvas.set_draw_color(Color::RGB(gray, gray, gray));
                texture_canvas
                    .fill_rect(Rect::new(x, y, size, size))
                    .expect("Failed to fill rect");
            }
        })
        .expect("Failed to render speckled texture");

    texture
}
