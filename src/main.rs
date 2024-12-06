#![windows_subsystem = "windows"] // Don't show console in Windows when druid app starts

mod cars;
mod lanes;
mod stats;

use std::{
    thread,
    time::{Duration, Instant},
};

use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

use cars::{Airt, Car, Dimensions, Speed};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let (ddpi, _hdpi, _vdpi) = video_subsystem.display_dpi(0).unwrap();

    let lane_width = (16.0 * ddpi / 134.4) as i32;

    let speed = Speed {
        fast: lane_width,
        default: lane_width / 2 as i32,
        slow: lane_width / 4 as i32,
    };

    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    let screen_width = display_mode.w;
    let screen_height = display_mode.h;

    let window_width = (screen_width as f32 * 0.6) as i32;
    let window_height = (screen_height as f32 * 0.8) as i32;

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

    let mut canvas = window.into_canvas().build().unwrap();
    canvas
        .set_logical_size(
            dimensions.window_width as u32,
            dimensions.window_height as u32,
        )
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut last_keypress_time = Instant::now();
    let keypress_interval = Duration::from_millis(128);

    let mut cars: Vec<Car> = Vec::new();
    let mut give_ways = 0;
    let mut cars_passed = 0;
    let mut max_time = Duration::from_millis(0);
    let mut min_time = Duration::MAX;

    let mut start_time = Instant::now();

    'running: loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        if elapsed < Duration::from_millis(16) {
            continue;
        }
        start_time = now;

        for (i, car) in cars.iter().enumerate() {
            assert!(
                car.index == i,
                "Mismatch: car.index {} does not match position {}",
                car.index,
                i
            );
        }

        render(&mut canvas, &mut cars, &dimensions);

        let mut tentative_positions = cars
            .iter()
            .map(|car| (car.x, car.y, car.index))
            .collect::<Vec<(i32, i32, usize)>>();

        for car in cars.iter_mut() {
            if car.update(
                &mut tentative_positions,
                &mut cars_passed,
                &mut max_time,
                &mut min_time,
                &dimensions,
            ) {
                give_ways += 1;
            }
        }

        cars.retain(|car| !car.gone);

        for (index, car) in cars.iter_mut().enumerate() {
            car.index = index;
        }

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
                            Keycode::Up => cars.push(Car::spawn(Airt::Up, cars.len(), &dimensions)),
                            Keycode::Down => {
                                cars.push(Car::spawn(Airt::Down, cars.len(), &dimensions))
                            }
                            Keycode::Left => {
                                cars.push(Car::spawn(Airt::Left, cars.len(), &dimensions))
                            }
                            Keycode::Right => {
                                cars.push(Car::spawn(Airt::Right, cars.len(), &dimensions))
                            }
                            Keycode::R => {
                                let directions = [Airt::Up, Airt::Down, Airt::Left, Airt::Right];
                                let random_direction =
                                    directions[rand::thread_rng().gen_range(0..directions.len())];
                                cars.push(Car::spawn(random_direction, cars.len(), &dimensions));
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

    drop(canvas);

    thread::sleep(Duration::from_millis(256));

    let s = if cars_passed == 0 {
        "Crashes: 0\nNear misses: 0\nGive ways: 0\nCars passed: 0\nSlowest speed: N/A\nFastest speed: N/A\nMax time: N/A\nMin time: N/A".to_string()
    } else {
        let slowest_speed = if give_ways == 0 { 250 } else { 0 };
        format!(
            "Crashes: 0\nNear misses: 0\nGive ways: {}\nCars passed: {}\nSlowest speed: {} logical px/s\nFastest speed: 1000 logical px/s\nMax time: {:.2}s\nMin time: {:.2}s",
            give_ways, cars_passed, slowest_speed, max_time.as_secs_f64(), min_time.as_secs_f64())
    };
    stats::show(&s);
}

fn render(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    cars: &mut Vec<Car>,
    dimensions: &Dimensions,
) {
    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.clear();

    lanes::draw(canvas, &dimensions);

    for car in cars {
        car.draw(canvas, &dimensions);
    }

    canvas.present();
}
