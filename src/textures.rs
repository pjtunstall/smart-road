use image::GenericImageView;
use rand::Rng;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{BlendMode, Canvas, Texture, TextureCreator},
    surface::Surface,
    video::WindowContext,
};

use crate::{lanes, types::Dimensions};

pub fn create_textures<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    dimensions: &Dimensions,
    canvas: &mut Canvas<sdl2::video::Window>,
) -> (
    Texture<'a>,
    Texture<'a>,
    [Texture<'a>; 4],
    Vec<(Texture<'a>, [f64; 2])>,
) {
    let background_texture = create_speckled_texture(
        &texture_creator,
        dimensions.window_width as u32,
        dimensions.window_height as u32,
        canvas,
    );
    let lanes_texture = lanes::draw(canvas, &dimensions, &texture_creator);
    let car_textures = create_car_textures(&texture_creator, &dimensions);

    let paths = [
        "images/trees.jpg",
        "images/left_trees.png",
        "images/right_tree.png",
        "images/little_tree.png",
        "images/tree_top.png",
    ];

    let mut tree_textures = Vec::new();
    for path in paths {
        tree_textures.push(create_texture_from_image(&texture_creator, path));
    }

    (
        background_texture,
        lanes_texture,
        car_textures,
        tree_textures,
    )
}

fn create_texture_from_image<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    path: &str,
) -> (Texture<'a>, [f64; 2]) {
    let img = image::open(path).expect("Failed to open image");
    let (width, height) = img.dimensions();
    let mut raw_pixels: Vec<u8> = img.to_rgba8().into_raw();

    let surface = Surface::from_data(
        &mut raw_pixels,
        width as u32,
        height as u32,
        4 * width as u32, // Row length in bytes (4 for RGBA)
        PixelFormatEnum::RGBA32,
    )
    .expect("Failed to create surface");

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .expect("Failed to create texture");

    (texture, [width as f64, height as f64])
}

pub fn create_speckled_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    width: u32,
    height: u32,
    canvas: &mut Canvas<sdl2::video::Window>,
) -> Texture<'a> {
    // Create texture with blending enabled.
    let mut texture = texture_creator
        .create_texture_target(Some(PixelFormatEnum::RGBA8888), width, height)
        .expect("Failed to create texture target");

    texture.set_blend_mode(BlendMode::Blend);

    canvas
        .with_texture_canvas(&mut texture, |texture_canvas| {
            // Clear with fully transparent color
            texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
            texture_canvas.clear();

            let mut rng = rand::rng();
            for _ in 0..255 {
                let x = rng.random_range(0..width as i32);
                let y = rng.random_range(0..height as i32);
                let size = rng.random_range(1..4);
                let gray = rng.random_range(128..255);

                texture_canvas.set_draw_color(Color::RGBA(gray, gray, gray, 255));
                texture_canvas
                    .fill_rect(Rect::new(x, y, size, size))
                    .expect("Failed to fill rect");
            }
        })
        .expect("Failed to render speckled texture");

    texture
}

fn create_car_textures<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    dimensions: &Dimensions,
) -> [sdl2::render::Texture<'a>; 4] {
    [
        create_car_texture(texture_creator, dimensions, Color::RGB(255, 0, 0)),
        create_car_texture(texture_creator, dimensions, Color::RGB(0, 255, 0)),
        create_car_texture(texture_creator, dimensions, Color::RGB(0, 0, 255)),
        create_car_texture(texture_creator, dimensions, Color::RGB(255, 255, 0)),
    ]
}

fn create_car_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    dimensions: &Dimensions,
    body_color: Color,
) -> sdl2::render::Texture<'a> {
    let lane_width = dimensions.lane_width as u32;

    let scale = lane_width as f32 / 16.0;

    let s = |val: u32| (val as f32 * scale).round() as u32;
    let si = |val: i32| (val as f32 * scale).round() as i32;

    let mut car_surface = sdl2::surface::Surface::new(
        lane_width,
        lane_width,
        sdl2::pixels::PixelFormatEnum::RGBA8888,
    )
    .expect("Failed to create car surface");

    car_surface
        .fill_rect(
            Rect::new(0, 0, lane_width, lane_width),
            Color::RGBA(0, 0, 0, 0),
        )
        .unwrap();

    // Draw body.
    car_surface
        .fill_rect(Rect::new(si(4), si(1), s(8), s(14)), body_color)
        .unwrap();

    // Draw windows.
    car_surface
        .fill_rect(Rect::new(si(5), si(6), s(6), s(2)), Color::RGB(0, 0, 0))
        .unwrap();
    car_surface
        .fill_rect(Rect::new(si(5), si(9), s(6), s(2)), Color::RGB(0, 0, 0))
        .unwrap();

    // Draw headlights.
    car_surface
        .fill_rect(
            Rect::new(si(5), si(1), s(2).max(1), s(1).max(1)),
            Color::RGB(255, 255, 255),
        )
        .unwrap();
    car_surface
        .fill_rect(
            Rect::new(si(9), si(1), s(2).max(1), s(1).max(1)),
            Color::RGB(255, 255, 255),
        )
        .unwrap();

    texture_creator
        .create_texture_from_surface(&car_surface)
        .expect("Failed to create car texture")
}
