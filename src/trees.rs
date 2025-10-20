use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

use crate::types::Dimensions;

const REFERENCE_WIDTH: f64 = 600.0;
const REFERENCE_HEIGHT: f64 = 600.0;

pub fn plant(
    canvas: &mut Canvas<Window>,
    tree_textures: &Vec<(Texture, [f64; 2])>,
    dimensions: &Dimensions,
) {
    plant_left_trees(canvas, &tree_textures[1].0, tree_textures[1].1, dimensions);
    plant_right_trees(canvas, &tree_textures[2].0, tree_textures[2].1, dimensions);
    plant_little_trees(canvas, &tree_textures[3].0, tree_textures[3].1, dimensions);
    plant_tree_top(canvas, &tree_textures[4].0, tree_textures[4].1, dimensions);
}

fn scale_position(original_x: i32, original_y: i32, dimensions: &Dimensions) -> (i32, i32) {
    let scale_x = dimensions.window_width as f64 / REFERENCE_WIDTH;
    let scale_y = dimensions.window_height as f64 / REFERENCE_HEIGHT;

    (
        (original_x as f64 * scale_x) as i32,
        (original_y as f64 * scale_y) as i32,
    )
}

fn plant_left_trees(
    canvas: &mut Canvas<Window>,
    left_trees_texture: &Texture,
    data: [f64; 2],
    dimensions: &Dimensions,
) {
    let width = data[0];
    let height = data[1];
    let (x, y) = scale_position(0, 290, dimensions);

    canvas
        .copy(
            left_trees_texture,
            None,
            Some(Rect::new(x, y, (width * 0.7) as u32, (height * 0.7) as u32)),
        )
        .unwrap();
}

fn plant_right_trees(
    canvas: &mut Canvas<Window>,
    right_tree_texture: &Texture,
    data: [f64; 2],
    dimensions: &Dimensions,
) {
    let width = data[0];
    let height = data[1];

    let right_trees = [
        (200, 555, 0.5),
        (330, 390, 0.5),
        (0, 380, 0.5),
        (488, 316, 0.5),
        (208, 100, 0.4),
    ];

    for tree in right_trees {
        let (x, y) = scale_position(tree.0, tree.1, dimensions);
        canvas
            .copy(
                right_tree_texture,
                None,
                Some(Rect::new(
                    x,
                    y,
                    (width * tree.2) as u32,
                    (height * tree.2) as u32,
                )),
            )
            .unwrap();
    }
}

fn plant_little_trees(
    canvas: &mut Canvas<Window>,
    little_tree_texture: &Texture,
    data: [f64; 2],
    dimensions: &Dimensions,
) {
    let width = data[0];
    let height = data[1];

    let little_trees = [
        (336, 64, 0.7),
        (342, 210, 0.6),
        (393, 330, 0.5),
        (236, 55, 0.5),
        (236, 0, 0.4),
        (342, 468, 0.5),
    ];

    for tree in little_trees {
        let (x, y) = scale_position(tree.0, tree.1, dimensions);
        canvas
            .copy(
                little_tree_texture,
                None,
                Some(Rect::new(
                    x,
                    y,
                    (width * tree.2) as u32,
                    (height * tree.2) as u32,
                )),
            )
            .unwrap();
    }
}

fn plant_tree_top(
    canvas: &mut Canvas<Window>,
    tree_top_texture: &Texture,
    data: [f64; 2],
    dimensions: &Dimensions,
) {
    let width = data[0];
    let height = data[1];
    let (x, y) = scale_position(520, 214, dimensions);

    canvas
        .copy(
            tree_top_texture,
            None,
            Some(Rect::new(x, y, (width * 0.4) as u32, (height * 0.4) as u32)),
        )
        .unwrap();
}
