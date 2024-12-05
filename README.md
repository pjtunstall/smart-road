# smart-road

## Eh?

This is my solution to [smart-road](https://github.com/01-edu/public/tree/master/subjects/smart-road), the first project of the Rust arc of the 01Edu coding bootcamp. It consists of a traffic simulation. The idea is to show cars passing an intersection without colliding.

## Setup

[Install Rust](https://www.rust-lang.org/learn/get-started) if you haven't already. Clone this repo by entering the command `git clone https://github.com/pjtunstall/smart-road` in a terminal. Navigate into the root directory of the project with `cd smart-road` and type `cargo run` to compile and run the program in one step. Alternatively, you can build an executable file with `cargo build`, which will be saved in `smart-road/target/debug`.

## Usage

Arrow keys to spawn cars traveling in a chosen direction, `R` to spawn a car traveling in a random direction, `ESC` or close the window to end the simulation and see the stats.

## Todo

- Scale sdl2 window to screen size and DPI.
- Refactor: modularize.
