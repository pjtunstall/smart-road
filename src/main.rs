#![windows_subsystem = "windows"] // From the druid docs: "By default, Windows will open a console with your application’s window. If you don’t want the console to be shown, use #![windows_subsystem = "windows"] at the beginning of your crate."

mod cars;
mod lanes;
mod sim;
mod stats;
mod types;

use crate::cars::Traffic;

fn main() {
    let mut traffic = Traffic::new();
    let (sdl_context, mut canvas, dimensions) = sim::setup();
    sim::run(&sdl_context, &mut canvas, &dimensions, &mut traffic);
    let s = traffic.format();
    stats::show(&s);
}
