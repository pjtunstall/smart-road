#![windows_subsystem = "windows"] // Don't show console in Windows when druid app starts
use rand::Rng;
use std::time::{Duration, Instant};

use druid::{widget::Label, AppLauncher, Env, Point, Screen, Widget, WidgetExt, WindowDesc};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

struct Dimensions {
    window_width: i32,
    window_height: i32,
    half_width: i32,
    half_height: i32,
    lane_width: i32,
    speed: Speed,
}

struct Speed {
    fast: i32,
    default: i32,
    slow: i32,
}

// These directions are all from our point of view as we look at the screen. They describe a car's initial direction and its direction after it's turned, both from our perspective.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Airt {
    Up,
    Down,
    Left,
    Right,
}

struct Car {
    x: i32,
    y: i32,
    color: Color,
    direction: Direction,
    speed: i32,
    vertical: bool,
    gone: bool,
    index: usize,
    birthday: Instant,
}

struct Direction {
    start: Airt,
    end: Airt,
}

impl Car {
    fn spawn(initial_direction: Airt, index: usize, dimensions: &Dimensions) -> Self {
        let x;
        let y;
        let final_direction;
        let color;
        let mut speed = dimensions.speed.default;
        let vertical;

        let r = rand::thread_rng().gen_range(0..3);

        match &initial_direction {
            Airt::Up => {
                vertical = true;
                color = Color::RGB(255, 0, 0);
                match r {
                    0 => {
                        x = dimensions.half_width;
                        y = dimensions.window_height - dimensions.lane_width;
                        final_direction = Airt::Left;
                    }
                    1 => {
                        x = dimensions.half_width + dimensions.lane_width;
                        y = dimensions.window_height - dimensions.lane_width;
                        final_direction = Airt::Up;
                        speed = dimensions.speed.fast;
                    }
                    2 => {
                        x = dimensions.half_width + 2 * dimensions.lane_width;
                        y = dimensions.window_height - dimensions.lane_width;
                        final_direction = Airt::Right;
                        speed = dimensions.speed.slow;
                    }
                    _ => {
                        panic!("Invalid turn");
                    }
                }
            }

            Airt::Down => {
                vertical = true;
                color = Color::RGB(0, 0, 255);
                match r {
                    0 => {
                        x = dimensions.half_width - 3 * dimensions.lane_width;
                        y = 0;
                        final_direction = Airt::Left;
                        speed = dimensions.speed.slow;
                    }
                    1 => {
                        x = dimensions.half_width - 2 * dimensions.lane_width;
                        y = 0;
                        final_direction = Airt::Down;
                        speed = dimensions.speed.fast;
                    }
                    _ => {
                        x = dimensions.half_width - dimensions.lane_width;
                        y = 0;
                        final_direction = Airt::Right;
                    }
                }
            }

            Airt::Right => {
                vertical = false;
                color = Color::RGB(0, 255, 0);
                match r {
                    0 => {
                        x = 0;
                        y = dimensions.half_height;
                        final_direction = Airt::Up;
                    }
                    1 => {
                        x = 0;
                        y = dimensions.half_height + dimensions.lane_width;
                        final_direction = Airt::Right;
                        speed = dimensions.speed.fast;
                    }
                    _ => {
                        x = 0;
                        y = dimensions.half_height + 2 * dimensions.lane_width;
                        final_direction = Airt::Down;
                        speed = dimensions.speed.slow;
                    }
                }
            }

            Airt::Left => {
                vertical = false;
                color = Color::RGB(255, 255, 0);
                match r {
                    0 => {
                        x = dimensions.window_width - dimensions.lane_width;
                        y = dimensions.half_height - 3 * dimensions.lane_width;
                        final_direction = Airt::Up;
                    }
                    1 => {
                        x = dimensions.window_width - dimensions.lane_width;
                        y = dimensions.half_height - 2 * dimensions.lane_width;
                        final_direction = Airt::Left;
                        speed = dimensions.speed.fast;
                    }
                    2 => {
                        x = dimensions.window_width - dimensions.lane_width;
                        y = dimensions.half_height - dimensions.lane_width;
                        final_direction = Airt::Down;
                        speed = dimensions.speed.slow;
                    }
                    _ => {
                        panic!("Invalid turn");
                    }
                }
            }
        }

        Car {
            x,
            y,
            color,
            direction: Direction {
                start: initial_direction,
                end: final_direction,
            },
            speed,
            vertical,
            gone: false,
            index,
            birthday: Instant::now(),
        }
    }

    fn will_collide(
        &self,
        new_x: i32,
        new_y: i32,
        tentative_positions: &Vec<(i32, i32, usize)>,
        dimensions: &Dimensions,
    ) -> bool {
        for other in tentative_positions {
            if other.2 == self.index {
                continue;
            }
            if new_x < other.0 + dimensions.lane_width
                && new_x + dimensions.lane_width > other.0
                && new_y < other.1 + dimensions.lane_width
                && new_y + dimensions.lane_width > other.1
            {
                return true;
            }
        }
        false
    }

    fn update(
        &mut self,
        tentative_positions: &mut Vec<(i32, i32, usize)>,
        cars_passed: &mut i32,
        max_time: &mut Duration,
        min_time: &mut Duration,
        dimensions: &Dimensions,
    ) -> bool {
        if self.x < 0
            || self.x + dimensions.lane_width > dimensions.window_width
            || self.y < 0
            || self.y + dimensions.lane_width > dimensions.window_height
        {
            *cars_passed += 1;
            self.gone = true;
            let elapsed = Instant::now().duration_since(self.birthday);
            if *max_time < elapsed {
                *max_time = elapsed;
            }
            if *min_time > elapsed {
                *min_time = elapsed;
            }
            return false;
        }

        let mut new_x = self.x;
        let mut new_y = self.y;

        match self.direction.start {
            Airt::Up => match self.direction.end {
                Airt::Left => {
                    if self.y > dimensions.half_height - dimensions.lane_width {
                        new_y = self.y - self.speed;
                    } else {
                        new_y = dimensions.half_height - dimensions.lane_width;
                        self.vertical = false;
                        new_x = self.x - self.speed;
                    }
                }
                Airt::Up => {
                    new_y = self.y - self.speed;
                }
                Airt::Right => {
                    if self.y > dimensions.half_height + 2 * dimensions.lane_width {
                        new_y = self.y - self.speed;
                    } else {
                        new_y = dimensions.half_height + 2 * dimensions.lane_width;
                        self.vertical = false;
                        new_x = self.x + self.speed;
                    }
                }
                _ => panic!("Invalid turn"),
            },
            Airt::Down => match self.direction.end {
                Airt::Left => {
                    if self.y < dimensions.half_height - 3 * dimensions.lane_width {
                        new_y = self.y + self.speed;
                    } else {
                        new_y = dimensions.half_height - 3 * dimensions.lane_width;
                        new_x = self.x - self.speed;
                        self.vertical = false;
                    }
                }
                Airt::Down => {
                    new_y = self.y + self.speed;
                }
                Airt::Right => {
                    if self.y < dimensions.half_height {
                        new_y = self.y + self.speed;
                    } else {
                        new_y = dimensions.half_height;
                        new_x = self.x + self.speed;
                        self.vertical = false;
                    }
                }
                _ => panic!("Invalid turn"),
            },
            Airt::Left => match self.direction.end {
                Airt::Up => {
                    if self.x > dimensions.half_width + 2 * dimensions.lane_width {
                        new_x = self.x - self.speed;
                    } else {
                        new_x = dimensions.half_width + 2 * dimensions.lane_width;
                        self.vertical = true;
                        new_y = self.y - self.speed;
                    }
                }
                Airt::Left => {
                    new_x = self.x - self.speed;
                }
                Airt::Down => {
                    if self.x > dimensions.half_width - dimensions.lane_width {
                        new_x = self.x - self.speed;
                    } else {
                        new_x = dimensions.half_width - dimensions.lane_width;
                        self.vertical = true;
                        new_y = self.y + self.speed;
                    }
                }
                _ => panic!("Invalid turn"),
            },
            Airt::Right => match self.direction.end {
                Airt::Up => {
                    if self.x < dimensions.half_width {
                        new_x = self.x + self.speed;
                    } else {
                        new_x = dimensions.half_width;
                        self.vertical = true;
                        new_y = self.y - self.speed;
                    }
                }
                Airt::Right => {
                    new_x = self.x + self.speed;
                }
                Airt::Down => {
                    if self.x < dimensions.half_width - 3 * dimensions.lane_width {
                        new_x = self.x + self.speed;
                    } else {
                        new_x = dimensions.half_width - 3 * dimensions.lane_width;
                        self.speed = dimensions.speed.fast;
                        self.vertical = true;
                        new_y = self.y + self.speed;
                    }
                }
                _ => panic!("Invalid turn"),
            },
        }

        if self.will_collide(new_x, new_y, tentative_positions, dimensions) {
            return true;
        }

        // Update tentative positions
        tentative_positions[self.index] = (new_x, new_y, self.index);

        // Apply final movement
        self.x = new_x;
        self.y = new_y;

        return false;
    }

    fn draw(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        dimensions: &Dimensions,
    ) {
        if self.x < 0
            || self.x + dimensions.lane_width > dimensions.window_width
            || self.y < 0
            || self.y + dimensions.lane_width > dimensions.window_height
        {
            return;
        }

        let x;
        let y;
        let width;
        let height;

        if self.vertical {
            x = self.x + dimensions.lane_width / 4 as i32;
            y = self.y as i32 + 1;
            width = dimensions.lane_width as u32 / 2u32;
            height = dimensions.lane_width as u32 - 1u32;
        } else {
            x = self.x as i32 + 1;
            y = self.y + dimensions.lane_width / 4 as i32;
            width = dimensions.lane_width as u32 - 1u32;
            height = dimensions.lane_width as u32 / 2u32;
        }

        canvas.set_draw_color(self.color);

        canvas
            .fill_rect(sdl2::rect::Rect::new(x, y, width, height))
            .unwrap();
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let (ddpi, hdpi, vdpi) = video_subsystem.display_dpi(0).unwrap();

    println!(
        "Diagonal DPI: {}, Horizontal DPI: {}, Vertical DPI: {}",
        ddpi, hdpi, vdpi
    );

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

    let s = if cars_passed == 0 {
        "Crashes: 0\nNear misses: 0\nGive ways: 0\nCars passed: 0\nSlowest speed: N/A\nFastest speed: N/A\nMax time: N/A\nMin time: N/A".to_string()
    } else {
        let slowest_speed = if give_ways == 0 { 250 } else { 0 };
        format!(
            "Crashes: 0\nNear misses: 0\nGive ways: {}\nCars passed: {}\nSlowest speed: {}logical px/s\nFastest speed: 1000 logical px/s\nMax time: {:.2}s\nMin time: {:.2}s",
            give_ways, cars_passed, slowest_speed, max_time.as_secs_f64(), min_time.as_secs_f64())
    };
    show(&s);
}

fn render(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    cars: &mut Vec<Car>,
    dimensions: &Dimensions,
) {
    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas
        .fill_rect(sdl2::rect::Rect::new(
            dimensions.half_width - 3 * dimensions.lane_width,
            0,
            6 * dimensions.lane_width as u32,
            dimensions.window_height as u32,
        ))
        .unwrap();

    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas
        .fill_rect(sdl2::rect::Rect::new(
            0,
            dimensions.half_height - 3 * dimensions.lane_width,
            dimensions.window_width as u32,
            6 * dimensions.lane_width as u32,
        ))
        .unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Center lines
    canvas
        .draw_line(
            (dimensions.half_width, 0),
            (dimensions.half_width, dimensions.window_height),
        )
        .unwrap();
    canvas
        .draw_line(
            (0, dimensions.half_height),
            (dimensions.window_width, dimensions.half_height),
        )
        .unwrap();

    // Outer lines
    canvas
        .draw_line(
            (dimensions.half_width - 3 * dimensions.lane_width, 0),
            (
                dimensions.half_width - 3 * dimensions.lane_width,
                dimensions.window_height,
            ),
        )
        .unwrap();
    canvas
        .draw_line(
            (0, dimensions.half_height - 3 * dimensions.lane_width),
            (
                dimensions.window_width,
                dimensions.half_height - 3 * dimensions.lane_width,
            ),
        )
        .unwrap();
    canvas
        .draw_line(
            (dimensions.half_width + 3 * dimensions.lane_width, 0),
            (
                dimensions.half_width + 3 * dimensions.lane_width,
                dimensions.window_height,
            ),
        )
        .unwrap();
    canvas
        .draw_line(
            (0, dimensions.half_height + 3 * dimensions.lane_width),
            (
                dimensions.window_width,
                dimensions.half_height + 3 * dimensions.lane_width,
            ),
        )
        .unwrap();

    // Draw lane markers
    for i in 1..3 {
        draw_line(
            canvas,
            (dimensions.half_width - dimensions.lane_width * i, 0),
            (
                dimensions.half_width - dimensions.lane_width * i,
                dimensions.window_height,
            ),
        );
        draw_line(
            canvas,
            (dimensions.half_width + dimensions.lane_width * i, 0),
            (
                dimensions.half_width + dimensions.lane_width * i,
                dimensions.window_height,
            ),
        );
        draw_line(
            canvas,
            (0, dimensions.half_height - dimensions.lane_width * i),
            (
                dimensions.window_width,
                dimensions.half_height - dimensions.lane_width * i,
            ),
        );
        draw_line(
            canvas,
            (0, dimensions.half_height + dimensions.lane_width * i),
            (
                dimensions.window_width,
                dimensions.half_height + dimensions.lane_width * i,
            ),
        );
    }

    for car in cars {
        car.draw(canvas, &dimensions);
    }

    canvas.present();
}

fn draw_line(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    start: (i32, i32),
    end: (i32, i32),
) {
    let dash_length = 4;
    let gap_length = 4;
    let (x1, y1) = start;
    let (x2, y2) = end;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let steps = (dx.abs().max(dy.abs())) as f32;

    let step_x = dx as f32 / steps;
    let step_y = dy as f32 / steps;

    let mut current_x = x1 as f32;
    let mut current_y = y1 as f32;

    let mut draw_dash = true;
    let mut step_counter = 0.0;

    while step_counter < steps {
        if draw_dash {
            let next_x = current_x + step_x;
            let next_y = current_y + step_y;

            canvas
                .draw_line(
                    (current_x as i32, current_y as i32),
                    (next_x as i32, next_y as i32),
                )
                .unwrap();
        }

        current_x += step_x;
        current_y += step_y;

        step_counter += 1.0;
        if step_counter as i32 % (dash_length + gap_length) < dash_length {
            draw_dash = true;
        } else {
            draw_dash = false;
        }
    }
}

fn show(s: &str) {
    let screen = Screen::get_monitors()[0].virtual_rect();
    let x = screen.x0 + (screen.x1 - screen.x0) / 2.0;
    let y = screen.y0 + (screen.y1 - screen.y0) / 2.0;

    let main_window = WindowDesc::new(ui_builder(s))
        .window_size((600.0, 400.0))
        .title("Smart Road")
        .set_position(Point::new(x - 300.0, y - 200.0));

    AppLauncher::with_window(main_window)
        .delegate(MyAppDelegate)
        .launch(())
        .expect("Launch failed");
}

struct MyAppDelegate;

impl druid::AppDelegate<()> for MyAppDelegate {
    fn event(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _window_id: druid::WindowId,
        event: druid::Event,
        _data: &mut (),
        _env: &Env,
    ) -> Option<druid::Event> {
        match event {
            druid::Event::WindowCloseRequested => {
                ctx.submit_command(druid::commands::CLOSE_WINDOW);
                std::thread::sleep(Duration::from_millis(10));
                std::process::exit(0);
            }
            druid::Event::KeyDown(ref key_event) => {
                if key_event.key == druid::keyboard_types::Key::Escape {
                    ctx.submit_command(druid::commands::CLOSE_WINDOW);
                    std::thread::sleep(Duration::from_millis(10));
                    std::process::exit(0);
                }
            }
            _ => {}
        }
        Some(event)
    }
}

fn ui_builder(s: &str) -> impl Widget<()> {
    Label::new(s).center()
}
