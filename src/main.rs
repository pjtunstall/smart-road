#![allow(dead_code)]
use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

const HALF_WIDTH: i32 = WINDOW_WIDTH / 2;
const HALF_HEIGHT: i32 = WINDOW_HEIGHT / 2;

const LANE_WIDTH: i32 = 16;

const CAR_WIDTH: i32 = 16;
const CAR_HEIGHT: i32 = 16;

const TOP_SPEED: i32 = 8;

#[derive(Debug, PartialEq)]
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
}

struct Direction {
    start: Airt,
    end: Airt,
}

impl Car {
    fn spawn(initial_direction: Airt) -> Self {
        let x;
        let y;
        let final_direction;
        let color;
        let mut speed = 1;

        let r = rand::thread_rng().gen_range(0..3);

        match &initial_direction {
            Airt::Up => {
                color = Color::RGB(255, 0, 0);
                match r {
                    0 => {
                        x = HALF_WIDTH;
                        y = WINDOW_HEIGHT - CAR_HEIGHT;
                        final_direction = Airt::Left;
                    }
                    1 => {
                        x = HALF_WIDTH + LANE_WIDTH;
                        y = WINDOW_HEIGHT - CAR_HEIGHT;
                        final_direction = Airt::Up;
                    }
                    2 => {
                        x = HALF_WIDTH + 2 * LANE_WIDTH;
                        y = WINDOW_HEIGHT - CAR_HEIGHT;
                        final_direction = Airt::Right;
                        speed = TOP_SPEED;
                    }
                    _ => {
                        panic!("Invalid turn");
                    }
                }
            }
            Airt::Down => {
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
                    }
                }
            }
            _ => {
                panic!(
                    "Initial direction {:?} not yet implemented",
                    initial_direction
                );
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
        }
    }

    fn update(&mut self) {
        if self.x < 0 || self.x > 800 || self.y < 0 || self.y > 600 {
            return;
        }

        match self.direction.start {
            Airt::Up => match self.direction.end {
                Airt::Left => {
                    if self.y > HALF_HEIGHT - LANE_WIDTH {
                        self.y -= self.speed;
                    } else {
                        self.x -= self.speed;
                    }
                }
                Airt::Up => {
                    self.y -= 1;
                }
                Airt::Right => {
                    if self.y > HALF_HEIGHT + 2 * LANE_WIDTH {
                        self.y -= self.speed;
                    } else {
                        self.x += self.speed;
                    }
                }
                _ => {
                    panic!("Invalid turn");
                }
            },
            Airt::Down => match self.direction.end {
                Airt::Left => {
                    if self.y < HALF_HEIGHT - 3 * LANE_WIDTH {
                        self.y += self.speed;
                    } else {
                        self.x -= self.speed;
                    }
                }
                Airt::Down => {
                    self.y += self.speed;
                }
                Airt::Right => {
                    if self.y < HALF_HEIGHT {
                        self.y += self.speed;
                    } else {
                        self.x += self.speed;
                    }
                }
                _ => {
                    panic!("Invalid turn");
                }
            },
            _ => {
                panic!("Not yet implemented");
            }
        }
    }

    fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        if self.x < 0 || self.x > WINDOW_WIDTH || self.y < 0 || self.y > WINDOW_HEIGHT {
            return;
        }

        canvas.set_draw_color(self.color);

        canvas
            .fill_rect(sdl2::rect::Rect::new(
                self.x,
                self.y,
                CAR_WIDTH as u32,
                CAR_HEIGHT as u32,
            ))
            .unwrap();
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

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
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut cars = Vec::new();

    'running: loop {
        std::thread::sleep(std::time::Duration::from_millis(4));

        render(&mut canvas, &mut cars);

        for car in &mut cars {
            car.update();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Up => cars.push(Car::spawn(Airt::Up)),
                    Keycode::Down => cars.push(Car::spawn(Airt::Down)),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, cars: &mut Vec<Car>) {
    canvas.set_draw_color(Color::RGB(255, 255, 255)); // White background
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black lines

    canvas
        .draw_line((HALF_WIDTH, 0), (HALF_WIDTH, WINDOW_HEIGHT))
        .unwrap();
    canvas
        .draw_line((0, HALF_HEIGHT), (WINDOW_WIDTH, HALF_HEIGHT))
        .unwrap();

    // Draw lane markers
    for i in 0..4 {
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
