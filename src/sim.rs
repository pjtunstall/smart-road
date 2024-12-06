use std::{
    thread,
    time::{Duration, Instant},
};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, Sdl};

use crate::{
    cars::Traffic,
    lanes,
    types::{Airt, Dimensions},
};

pub fn run(
    sdl_context: &Sdl,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    dimensions: &Dimensions,
    traffic: &mut Traffic,
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
        render(canvas, &dimensions, &traffic);

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

                    if now.duration_since(last_keypress_time) >= keypress_interval {
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
                }
                _ => {}
            }
        }
    }

    thread::sleep(Duration::from_millis(128));
}

fn render(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    dimensions: &Dimensions,
    traffic: &Traffic,
) {
    canvas.set_draw_color(Color::RGB(127, 127, 127));
    canvas.clear();

    lanes::draw(canvas, &dimensions);
    traffic.draw(canvas, &dimensions);

    canvas.present();
}
