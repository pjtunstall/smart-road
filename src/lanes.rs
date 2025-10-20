use sdl2::{
    pixels::{Color, PixelFormatEnum},
    render::{BlendMode, Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use crate::types::Dimensions;

pub fn draw<'a>(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    dimensions: &Dimensions,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Texture<'a> {
    let mut texture = texture_creator
        .create_texture_target(
            Some(PixelFormatEnum::RGBA8888),
            dimensions.window_width as u32,
            dimensions.window_height as u32,
        )
        .expect("Failed to create texture target");

    texture.set_blend_mode(BlendMode::Blend);

    canvas
        .with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(64, 64, 64));
            texture_canvas
                .fill_rect(sdl2::rect::Rect::new(
                    dimensions.half_width - 3 * dimensions.lane_width,
                    0,
                    6 * dimensions.lane_width as u32,
                    dimensions.window_height as u32,
                ))
                .unwrap();

            texture_canvas
                .fill_rect(sdl2::rect::Rect::new(
                    0,
                    dimensions.half_height - 3 * dimensions.lane_width,
                    dimensions.window_width as u32,
                    6 * dimensions.lane_width as u32,
                ))
                .unwrap();

            texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
            draw_center_lines_to_texture(texture_canvas, dimensions);
            draw_edge_lines_to_texture(texture_canvas, dimensions);
            draw_lane_lines_to_texture(texture_canvas, dimensions);
            draw_give_way_lines_to_texture(texture_canvas, dimensions);
        })
        .expect("Failed to render everything on texture");

    texture
}

fn draw_edge_lines_to_texture(texture_canvas: &mut Canvas<Window>, dimensions: &Dimensions) {
    texture_canvas
        .draw_line(
            (dimensions.half_width - 3 * dimensions.lane_width, 0),
            (
                dimensions.half_width - 3 * dimensions.lane_width,
                dimensions.half_height - 3 * dimensions.lane_width,
            ),
        )
        .unwrap();
    texture_canvas
        .draw_line(
            (
                dimensions.half_width - 3 * dimensions.lane_width,
                dimensions.half_height + 3 * dimensions.lane_width,
            ),
            (
                dimensions.half_width - 3 * dimensions.lane_width,
                dimensions.window_height,
            ),
        )
        .unwrap();

    texture_canvas
        .draw_line(
            (0, dimensions.half_height - 3 * dimensions.lane_width),
            (
                dimensions.half_width - 3 * dimensions.lane_width,
                dimensions.half_height - 3 * dimensions.lane_width,
            ),
        )
        .unwrap();
    texture_canvas
        .draw_line(
            (
                dimensions.half_width + 3 * dimensions.lane_width,
                dimensions.half_height - 3 * dimensions.lane_width,
            ),
            (
                dimensions.window_width,
                dimensions.half_height - 3 * dimensions.lane_width,
            ),
        )
        .unwrap();

    texture_canvas
        .draw_line(
            (dimensions.half_width + 3 * dimensions.lane_width, 0),
            (
                dimensions.half_width + 3 * dimensions.lane_width,
                dimensions.half_height - 3 * dimensions.lane_width,
            ),
        )
        .unwrap();
    texture_canvas
        .draw_line(
            (
                dimensions.half_width + 3 * dimensions.lane_width,
                dimensions.half_height + 3 * dimensions.lane_width,
            ),
            (
                dimensions.half_width + 3 * dimensions.lane_width,
                dimensions.window_height,
            ),
        )
        .unwrap();

    texture_canvas
        .draw_line(
            (0, dimensions.half_height + 3 * dimensions.lane_width),
            (
                dimensions.half_width - 3 * dimensions.lane_width,
                dimensions.half_height + 3 * dimensions.lane_width,
            ),
        )
        .unwrap();
    texture_canvas
        .draw_line(
            (
                dimensions.half_width + 3 * dimensions.lane_width,
                dimensions.half_height + 3 * dimensions.lane_width,
            ),
            (
                dimensions.window_width,
                dimensions.half_height + 3 * dimensions.lane_width,
            ),
        )
        .unwrap();
}

fn draw_give_way_lines_to_texture(texture_canvas: &mut Canvas<Window>, dimensions: &Dimensions) {
    let dash_len = (dimensions.lane_width as f32 * 0.25).round() as i32;
    let gap_len = dash_len;
    let offset = dimensions.lane_width / 2;

    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width,
            dimensions.half_height + 3 * dimensions.lane_width,
        ),
        (
            dimensions.half_width + 3 * dimensions.lane_width,
            dimensions.half_height + 3 * dimensions.lane_width,
        ),
        dash_len,
        gap_len,
    );
    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width,
            dimensions.half_height + 3 * dimensions.lane_width + offset,
        ),
        (
            dimensions.half_width + 3 * dimensions.lane_width,
            dimensions.half_height + 3 * dimensions.lane_width + offset,
        ),
        dash_len,
        gap_len,
    );

    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width,
            dimensions.half_height - 3 * dimensions.lane_width,
        ),
        (
            dimensions.half_width - 3 * dimensions.lane_width,
            dimensions.half_height - 3 * dimensions.lane_width,
        ),
        dash_len,
        gap_len,
    );
    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width,
            dimensions.half_height - 3 * dimensions.lane_width - offset,
        ),
        (
            dimensions.half_width - 3 * dimensions.lane_width,
            dimensions.half_height - 3 * dimensions.lane_width - offset,
        ),
        dash_len,
        gap_len,
    );

    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width + 3 * dimensions.lane_width,
            dimensions.half_height,
        ),
        (
            dimensions.half_width + 3 * dimensions.lane_width,
            dimensions.half_height - 3 * dimensions.lane_width,
        ),
        dash_len,
        gap_len,
    );
    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width + 3 * dimensions.lane_width + offset,
            dimensions.half_height,
        ),
        (
            dimensions.half_width + 3 * dimensions.lane_width + offset,
            dimensions.half_height - 3 * dimensions.lane_width,
        ),
        dash_len,
        gap_len,
    );

    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width - 3 * dimensions.lane_width,
            dimensions.half_height,
        ),
        (
            dimensions.half_width - 3 * dimensions.lane_width,
            dimensions.half_height + 3 * dimensions.lane_width,
        ),
        dash_len,
        gap_len,
    );
    draw_dashed_line_to_texture(
        texture_canvas,
        (
            dimensions.half_width - 3 * dimensions.lane_width - offset,
            dimensions.half_height,
        ),
        (
            dimensions.half_width - 3 * dimensions.lane_width - offset,
            dimensions.half_height + 3 * dimensions.lane_width,
        ),
        dash_len,
        gap_len,
    );
}

fn draw_lane_lines_to_texture(
    texture_canvas: &mut sdl2::render::Canvas<Window>,
    dimensions: &Dimensions,
) {
    let dash_len = dimensions.lane_width / 4;
    let gap_len = dimensions.lane_width / 4;

    for i in 1..3 {
        draw_dashed_line_to_texture(
            texture_canvas,
            (dimensions.half_width - dimensions.lane_width * i, 0),
            (
                dimensions.half_width - dimensions.lane_width * i,
                dimensions.window_height,
            ),
            dash_len,
            gap_len,
        );
        draw_dashed_line_to_texture(
            texture_canvas,
            (dimensions.half_width + dimensions.lane_width * i, 0),
            (
                dimensions.half_width + dimensions.lane_width * i,
                dimensions.window_height,
            ),
            dash_len,
            gap_len,
        );
        draw_dashed_line_to_texture(
            texture_canvas,
            (0, dimensions.half_height - dimensions.lane_width * i),
            (
                dimensions.window_width,
                dimensions.half_height - dimensions.lane_width * i,
            ),
            dash_len,
            gap_len,
        );
        draw_dashed_line_to_texture(
            texture_canvas,
            (0, dimensions.half_height + dimensions.lane_width * i),
            (
                dimensions.window_width,
                dimensions.half_height + dimensions.lane_width * i,
            ),
            dash_len,
            gap_len,
        );
    }
}

fn draw_center_lines_to_texture(
    texture_canvas: &mut sdl2::render::Canvas<Window>,
    dimensions: &Dimensions,
) {
    texture_canvas
        .draw_line(
            (dimensions.half_width, 0),
            (dimensions.half_width, dimensions.window_height),
        )
        .unwrap();
    texture_canvas
        .draw_line(
            (0, dimensions.half_height),
            (dimensions.window_width, dimensions.half_height),
        )
        .unwrap();
}

fn draw_dashed_line_to_texture(
    texture_canvas: &mut sdl2::render::Canvas<Window>,
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

            texture_canvas
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
