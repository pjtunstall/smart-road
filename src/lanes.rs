use sdl2::pixels::Color;

use crate::types::Dimensions;

pub fn draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, dimensions: &Dimensions) {
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

    draw_center_lines(canvas, dimensions);
    draw_edge_lines(canvas, dimensions);
    draw_lane_lines(canvas, dimensions);
}

fn draw_edge_lines(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    dimensions: &Dimensions,
) {
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
}

fn draw_lane_lines(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    dimensions: &Dimensions,
) {
    for i in 1..3 {
        draw_dashed_line(
            canvas,
            (dimensions.half_width - dimensions.lane_width * i, 0),
            (
                dimensions.half_width - dimensions.lane_width * i,
                dimensions.window_height,
            ),
            4,
            4,
        );
        draw_dashed_line(
            canvas,
            (dimensions.half_width + dimensions.lane_width * i, 0),
            (
                dimensions.half_width + dimensions.lane_width * i,
                dimensions.window_height,
            ),
            4,
            4,
        );
        draw_dashed_line(
            canvas,
            (0, dimensions.half_height - dimensions.lane_width * i),
            (
                dimensions.window_width,
                dimensions.half_height - dimensions.lane_width * i,
            ),
            4,
            4,
        );
        draw_dashed_line(
            canvas,
            (0, dimensions.half_height + dimensions.lane_width * i),
            (
                dimensions.window_width,
                dimensions.half_height + dimensions.lane_width * i,
            ),
            4,
            4,
        );
    }
}

fn draw_center_lines(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    dimensions: &Dimensions,
) {
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
}

fn draw_dashed_line(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    start: (i32, i32),
    end: (i32, i32),
    dash_length: i32,
    gap_length: i32,
) {
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
