#![allow(dead_code)]
use rand::Rng;
use std::time::{Duration, Instant};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const HALF_WIDTH: i32 = WINDOW_WIDTH / 2;
const HALF_HEIGHT: i32 = WINDOW_HEIGHT / 2;

const LANE_WIDTH: i32 = 16;

const TOP_SPEED: i32 = 16;
const DEFAULT_SPEED: i32 = 8;
const SLOW: i32 = 4;

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
}

struct Direction {
    start: Airt,
    end: Airt,
}

impl Car {
    fn spawn(initial_direction: Airt, index: usize) -> Self {
        let x;
        let y;
        let final_direction;
        let color;
        let mut speed = DEFAULT_SPEED;
        let vertical;

        let r = rand::thread_rng().gen_range(0..3);

        match &initial_direction {
            Airt::Up => {
                vertical = true;
                color = Color::RGB(255, 0, 0);
                match r {
                    0 => {
                        x = HALF_WIDTH;
                        y = WINDOW_HEIGHT - LANE_WIDTH;
                        final_direction = Airt::Left;
                        speed = SLOW;
                    }
                    1 => {
                        x = HALF_WIDTH + LANE_WIDTH;
                        y = WINDOW_HEIGHT - LANE_WIDTH;
                        final_direction = Airt::Up;
                    }
                    2 => {
                        x = HALF_WIDTH + 2 * LANE_WIDTH;
                        y = WINDOW_HEIGHT - LANE_WIDTH;
                        final_direction = Airt::Right;
                        speed = TOP_SPEED;
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
                        x = HALF_WIDTH - 3 * LANE_WIDTH;
                        y = 0;
                        final_direction = Airt::Left;
                        speed = TOP_SPEED;
                    }
                    1 => {
                        x = HALF_WIDTH - 2 * LANE_WIDTH;
                        y = 0;
                        final_direction = Airt::Down;
                    }
                    _ => {
                        x = HALF_WIDTH - LANE_WIDTH;
                        y = 0;
                        final_direction = Airt::Right;
                        speed = SLOW;
                    }
                }
            }

            Airt::Right => {
                vertical = false;
                color = Color::RGB(0, 255, 0);
                match r {
                    0 => {
                        x = 0;
                        y = HALF_HEIGHT;
                        final_direction = Airt::Up;
                        speed = SLOW;
                    }
                    1 => {
                        x = 0;
                        y = HALF_HEIGHT + LANE_WIDTH;
                        final_direction = Airt::Right;
                    }
                    _ => {
                        x = 0;
                        y = HALF_HEIGHT + 2 * LANE_WIDTH;
                        final_direction = Airt::Down;
                        speed = TOP_SPEED;
                    }
                }
            }

            Airt::Left => {
                vertical = false;
                color = Color::RGB(255, 255, 0);
                match r {
                    0 => {
                        x = WINDOW_WIDTH - LANE_WIDTH;
                        y = HALF_HEIGHT - 3 * LANE_WIDTH;
                        final_direction = Airt::Up;
                        speed = TOP_SPEED;
                    }
                    1 => {
                        x = WINDOW_WIDTH - LANE_WIDTH;
                        y = HALF_HEIGHT - 2 * LANE_WIDTH;
                        final_direction = Airt::Left;
                    }
                    2 => {
                        x = WINDOW_WIDTH - LANE_WIDTH;
                        y = HALF_HEIGHT - LANE_WIDTH;
                        final_direction = Airt::Down;
                        speed = SLOW;
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
        }
    }

    fn will_collide(
        &self,
        new_x: i32,
        new_y: i32,
        tentative_positions: &Vec<(i32, i32, usize)>,
    ) -> bool {
        for other in tentative_positions {
            if other.2 == self.index {
                continue;
            }
            if new_x < other.0 + LANE_WIDTH
                && new_x + LANE_WIDTH > other.0
                && new_y < other.1 + LANE_WIDTH
                && new_y + LANE_WIDTH > other.1
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
    ) -> bool {
        if self.x < 0 || self.x > WINDOW_WIDTH || self.y < 0 || self.y > WINDOW_HEIGHT {
            *cars_passed += 1;
            self.gone = true;
            return false;
        }

        let mut new_x = self.x;
        let mut new_y = self.y;

        match self.direction.start {
            Airt::Up => match self.direction.end {
                Airt::Left => {
                    if self.y > HALF_HEIGHT - LANE_WIDTH {
                        new_y = self.y - self.speed;
                    } else {
                        new_y = HALF_HEIGHT - LANE_WIDTH;
                        self.vertical = false;
                        new_x = self.x - self.speed;
                    }
                }
                Airt::Up => {
                    new_y = self.y - self.speed;
                }
                Airt::Right => {
                    if self.y > HALF_HEIGHT + 2 * LANE_WIDTH {
                        new_y = self.y - self.speed;
                    } else {
                        new_y = HALF_HEIGHT + 2 * LANE_WIDTH;
                        self.vertical = false;
                        new_x = self.x + self.speed;
                    }
                }
                _ => panic!("Invalid turn"),
            },
            Airt::Down => match self.direction.end {
                Airt::Left => {
                    if self.y < HALF_HEIGHT - 3 * LANE_WIDTH {
                        new_y = self.y + self.speed;
                    } else {
                        new_y = HALF_HEIGHT - 3 * LANE_WIDTH;
                        new_x = self.x - self.speed;
                        self.vertical = false;
                    }
                }
                Airt::Down => {
                    new_y = self.y + self.speed;
                }
                Airt::Right => {
                    if self.y < HALF_HEIGHT {
                        new_y = self.y + self.speed;
                    } else {
                        new_y = HALF_HEIGHT;
                        new_x = self.x + self.speed;
                        self.vertical = false;
                    }
                }
                _ => panic!("Invalid turn"),
            },
            Airt::Left => match self.direction.end {
                Airt::Up => {
                    if self.x > HALF_WIDTH + 2 * LANE_WIDTH {
                        new_x = self.x - self.speed;
                    } else {
                        new_x = HALF_WIDTH + 2 * LANE_WIDTH;
                        self.vertical = true;
                        new_y = self.y - self.speed;
                    }
                }
                Airt::Left => {
                    new_x = self.x - self.speed;
                }
                Airt::Down => {
                    if self.x > HALF_WIDTH - LANE_WIDTH {
                        new_x = self.x - self.speed;
                    } else {
                        new_x = HALF_WIDTH - LANE_WIDTH;
                        self.vertical = true;
                        new_y = self.y + self.speed;
                    }
                }
                _ => panic!("Invalid turn"),
            },
            Airt::Right => match self.direction.end {
                Airt::Up => {
                    if self.x < HALF_WIDTH {
                        new_x = self.x + self.speed;
                    } else {
                        new_x = HALF_WIDTH;
                        self.vertical = true;
                        new_y = self.y - self.speed;
                    }
                }
                Airt::Right => {
                    new_x = self.x + self.speed;
                }
                Airt::Down => {
                    if self.x < HALF_WIDTH - 3 * LANE_WIDTH {
                        new_x = self.x + self.speed;
                    } else {
                        new_x = HALF_WIDTH - 3 * LANE_WIDTH;
                        self.speed = TOP_SPEED;
                        self.vertical = true;
                        new_y = self.y + self.speed;
                    }
                }
                _ => panic!("Invalid turn"),
            },
        }

        if self.will_collide(new_x, new_y, tentative_positions) {
            return true;
        }

        // Update tentative positions
        tentative_positions[self.index] = (new_x, new_y, self.index);

        // Apply final movement
        self.x = new_x;
        self.y = new_y;

        return false;
    }

    fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        if self.x < 0 || self.x > WINDOW_WIDTH || self.y < 0 || self.y > WINDOW_HEIGHT {
            return;
        }

        let x;
        let y;
        let width;
        let height;

        if self.vertical {
            x = self.x + LANE_WIDTH / 4 as i32;
            y = self.y as i32;
            width = LANE_WIDTH as u32 / 2u32;
            height = LANE_WIDTH as u32;
        } else {
            x = self.x as i32;
            y = self.y + LANE_WIDTH / 4 as i32;
            width = LANE_WIDTH as u32;
            height = LANE_WIDTH as u32 / 2u32;
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

    // let display_mode = video_subsystem.current_display_mode(0).unwrap();
    // let screen_width = display_mode.w;
    // let screen_height = display_mode.h;

    // let width = (screen_width as f32 * 0.8) as u32;
    // let height = (screen_height as f32 * 0.8) as u32;

    let window = video_subsystem
        .window(
            "᛫᛬ᚱᚫᛁᛞᛟ᛬ᛊᛗᚫᚱᛏᛟ᛬᛫",
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    // canvas
    //     .set_logical_size(WINDOW_WIDTH, WINDOW_HEIGHT)
    //     .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut last_keypress_time = Instant::now();
    let keypress_interval = Duration::from_millis(64);

    let mut cars: Vec<Car> = Vec::new();
    let mut near_misses = 0;
    let mut cars_passed = 0;

    'running: loop {
        std::thread::sleep(std::time::Duration::from_millis(16));

        for (i, car) in cars.iter().enumerate() {
            assert!(
                car.index == i,
                "Mismatch: car.index {} does not match position {}",
                car.index,
                i
            );
        }

        render(&mut canvas, &mut cars);

        let mut tentative_positions = cars
            .iter()
            .map(|car| (car.x, car.y, car.index))
            .collect::<Vec<(i32, i32, usize)>>();

        for car in cars.iter_mut() {
            if car.update(&mut tentative_positions, &mut cars_passed) {
                near_misses += 1;
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
                    print!("Crashes: 0");
                    println!("Near misses: {}", near_misses);
                    println!("Cars passed: {}", cars_passed);
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let now = Instant::now();

                    if now.duration_since(last_keypress_time) >= keypress_interval {
                        match keycode {
                            Keycode::Up => cars.push(Car::spawn(Airt::Up, cars.len())),
                            Keycode::Down => cars.push(Car::spawn(Airt::Down, cars.len())),
                            Keycode::Left => cars.push(Car::spawn(Airt::Left, cars.len())),
                            Keycode::Right => cars.push(Car::spawn(Airt::Right, cars.len())),
                            Keycode::R => {
                                let directions = [Airt::Up, Airt::Down, Airt::Left, Airt::Right];
                                let random_direction =
                                    directions[rand::thread_rng().gen_range(0..directions.len())];
                                cars.push(Car::spawn(random_direction, cars.len()));
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
}

fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, cars: &mut Vec<Car>) {
    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas
        .fill_rect(sdl2::rect::Rect::new(
            HALF_WIDTH - 3 * LANE_WIDTH,
            0,
            6 * LANE_WIDTH as u32,
            WINDOW_HEIGHT as u32,
        ))
        .unwrap();

    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas
        .fill_rect(sdl2::rect::Rect::new(
            0,
            HALF_HEIGHT - 3 * LANE_WIDTH,
            WINDOW_WIDTH as u32,
            6 * LANE_WIDTH as u32,
        ))
        .unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Center lines
    canvas
        .draw_line((HALF_WIDTH, 0), (HALF_WIDTH, WINDOW_HEIGHT))
        .unwrap();
    canvas
        .draw_line((0, HALF_HEIGHT), (WINDOW_WIDTH, HALF_HEIGHT))
        .unwrap();

    // Outer lines
    canvas
        .draw_line(
            (HALF_WIDTH - 3 * LANE_WIDTH, 0),
            (HALF_WIDTH - 3 * LANE_WIDTH, WINDOW_HEIGHT),
        )
        .unwrap();
    canvas
        .draw_line(
            (0, HALF_HEIGHT - 3 * LANE_WIDTH),
            (WINDOW_WIDTH, HALF_HEIGHT - 3 * LANE_WIDTH),
        )
        .unwrap();
    canvas
        .draw_line(
            (HALF_WIDTH + 3 * LANE_WIDTH, 0),
            (HALF_WIDTH + 3 * LANE_WIDTH, WINDOW_HEIGHT),
        )
        .unwrap();
    canvas
        .draw_line(
            (0, HALF_HEIGHT + 3 * LANE_WIDTH),
            (WINDOW_WIDTH, HALF_HEIGHT + 3 * LANE_WIDTH),
        )
        .unwrap();

    // Draw lane markers
    for i in 1..3 {
        draw_line(
            canvas,
            (HALF_WIDTH - LANE_WIDTH * i, 0),
            (HALF_WIDTH - LANE_WIDTH * i, WINDOW_HEIGHT),
        );
        draw_line(
            canvas,
            (HALF_WIDTH + LANE_WIDTH * i, 0),
            (HALF_WIDTH + LANE_WIDTH * i, WINDOW_HEIGHT),
        );
        draw_line(
            canvas,
            (0, HALF_HEIGHT - LANE_WIDTH * i),
            (WINDOW_WIDTH, HALF_HEIGHT - LANE_WIDTH * i),
        );
        draw_line(
            canvas,
            (0, HALF_HEIGHT + LANE_WIDTH * i),
            (WINDOW_WIDTH, HALF_HEIGHT + LANE_WIDTH * i),
        );
    }

    for car in cars {
        car.draw(canvas);
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
