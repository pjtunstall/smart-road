#![windows_subsystem = "windows"] // Don't show console in Windows when druid app starts

mod cars;
mod lanes;
mod sim;
mod stats;
mod types;

use crate::{
    cars::Traffic,
    types::{Dimensions, Speed},
};

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

    let mut traffic = Traffic::new();

    sim::run(&sdl_context, &mut canvas, &dimensions, &mut traffic);

    let s = traffic.format();
    stats::show(&s);
}
