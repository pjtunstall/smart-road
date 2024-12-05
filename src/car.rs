use rand::Rng;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

pub struct Dimensions {
    pub window_width: i32,
    pub window_height: i32,
    pub half_width: i32,
    pub half_height: i32,
    pub lane_width: i32,
    pub speed: Speed,
}

pub struct Speed {
    pub fast: i32,
    pub default: i32,
    pub slow: i32,
}

// These directions are all from our point of view as we look at the screen. They describe a car's initial direction and its direction after it's turned, both from our perspective.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Airt {
    Up,
    Down,
    Left,
    Right,
}

pub struct Car {
    pub x: i32,
    pub y: i32,
    color: Color,
    direction: Direction,
    speed: i32,
    vertical: bool,
    pub gone: bool,
    pub index: usize,
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

    pub fn update(
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

    pub fn draw(
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
