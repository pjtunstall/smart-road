use rand::Rng;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

use crate::types::{Airt, Dimensions};

pub struct Traffic {
    pub cars: Vec<Car>,
    pub cars_passed: i32,
    pub give_ways: i32,
    pub max_time: Duration,
    pub min_time: Duration,
}

impl Traffic {
    pub fn new() -> Self {
        Traffic {
            cars: Vec::new(),
            cars_passed: 0,
            give_ways: 0,
            max_time: Duration::from_millis(0),
            min_time: Duration::MAX,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        dimensions: &Dimensions,
        car_textures: &[sdl2::render::Texture; 4],
    ) {
        for car in &self.cars {
            car.draw(canvas, &dimensions, &car_textures);
        }
    }

    pub fn format(&self) -> String {
        if self.cars_passed == 0 {
            "Crashes: 0\nNear misses: 0\nGive ways: 0\nCars passed: 0\nSlowest speed: N/A\nFastest speed: N/A\nMax time: N/A\nMin time: N/A".to_string()
        } else {
            let slowest_speed = if self.give_ways == 0 { 250 } else { 0 };
            format!(
                "Crashes: 0\nNear misses: 0\nGive ways: {}\nCars passed: {}\nSlowest speed: {}px/s\nFastest speed: 1000px/s\nMax time: {:.2}s\nMin time: {:.2}s",
                self.give_ways, self.cars_passed, slowest_speed, self.max_time.as_secs_f64(), self.min_time.as_secs_f64())
        }
    }

    pub fn push(&mut self, initial_direction: Airt, dimensions: &Dimensions) {
        self.cars
            .push(Car::spawn(initial_direction, self.cars.len(), dimensions));
    }

    pub fn push_random(&mut self, dimensions: &Dimensions) {
        let directions = [Airt::Up, Airt::Down, Airt::Left, Airt::Right];
        let random_direction = directions[rand::thread_rng().gen_range(0..directions.len())];
        self.cars
            .push(Car::spawn(random_direction, self.cars.len(), &dimensions));
    }

    pub fn update(&mut self, dimensions: &Dimensions) {
        for (i, car) in self.cars.iter().enumerate() {
            debug_assert!(
                car.index == i,
                "Mismatch: car.index {} does not match position {}",
                car.index,
                i
            );
        }

        let mut prospective_positions = self
            .cars
            .iter()
            .map(|car| (car.x, car.y, car.index)) // The `index` is used to ignore "collisions" of a car with itself.
            .collect::<Vec<(i32, i32, usize)>>();

        for car in self.cars.iter_mut() {
            if !car.update(
                &mut prospective_positions,
                &mut self.cars_passed,
                &mut self.max_time,
                &mut self.min_time,
                &dimensions,
            ) {
                self.give_ways += 1;
            }
        }

        self.cars.retain(|car| !car.gone);

        for (index, car) in self.cars.iter_mut().enumerate() {
            car.index = index;
        }
    }
}

pub struct Car {
    x: i32,
    y: i32,
    color_code: usize,
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
    pub fn spawn(initial_direction: Airt, index: usize, dimensions: &Dimensions) -> Self {
        let x;
        let y;
        let final_direction;
        let color_code;
        let mut speed = dimensions.speed.default;
        let vertical;

        let r = rand::thread_rng().gen_range(0..3);

        match &initial_direction {
            Airt::Up => {
                vertical = true;
                color_code = 0; // red
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
                color_code = 1; // green
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
                color_code = 2; // blue
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
                color_code = 3; // yellow
                match r {
                    0 => {
                        x = dimensions.window_width - dimensions.lane_width;
                        y = dimensions.half_height - 3 * dimensions.lane_width;
                        final_direction = Airt::Up;
                        speed = dimensions.speed.slow;
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
            color_code,
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
        prospective_positions: &Vec<(i32, i32, usize)>,
        dimensions: &Dimensions,
    ) -> bool {
        for other in prospective_positions {
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
        prospective_positions: &mut Vec<(i32, i32, usize)>,
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
            return true;
        }

        let (new_x, new_y) = self.calculate_new_position(dimensions);

        if self.will_collide(new_x, new_y, prospective_positions, dimensions) {
            return false;
        }

        prospective_positions[self.index] = (new_x, new_y, self.index);

        self.x = new_x;
        self.y = new_y;

        return true;
    }

    fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        dimensions: &Dimensions,
        car_textures: &[sdl2::render::Texture; 4],
    ) {
        if self.x < 0
            || self.x + dimensions.lane_width > dimensions.window_width
            || self.y < 0
            || self.y + dimensions.lane_width > dimensions.window_height
        {
            return;
        }

        let x = self.x as i32;
        let y = self.y as i32;
        let lane_width = dimensions.lane_width as u32;

        let angle = if self.vertical {
            match self.direction.start {
                Airt::Up => 0.0,
                Airt::Down => 180.0,
                Airt::Left | Airt::Right => {
                    if self.direction.end == Airt::Up {
                        0.0
                    } else {
                        180.0
                    }
                }
            }
        } else {
            match self.direction.start {
                Airt::Up | Airt::Down => {
                    if self.direction.end == Airt::Left {
                        -90.0
                    } else {
                        90.0
                    }
                }
                Airt::Left => -90.0,
                Airt::Right => 90.0,
            }
        };

        let center = sdl2::rect::Point::new(lane_width as i32 / 2, lane_width as i32 / 2);

        let car_texture = &car_textures[self.color_code];

        canvas
            .copy_ex(
                car_texture,
                None, // No cropping (draw the whole texture).
                Some(Rect::new(x, y, lane_width, lane_width)),
                angle,
                Some(center),
                false,
                false,
            )
            .expect("Failed to draw car with rotation");
    }

    fn calculate_new_position(&mut self, dimensions: &Dimensions) -> (i32, i32) {
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
                        self.vertical = true;
                        new_y = self.y + self.speed;
                    }
                }
                _ => panic!("Invalid turn"),
            },
        }

        (new_x, new_y)
    }
}
