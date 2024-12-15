use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

fn plant_left_trees(canvas: &mut Canvas<Window>, left_trees_texture: &Texture) {
    canvas
        .copy(
            left_trees_texture,
            None,
            Some(Rect::new(
                0,
                290,
                (221.0 * 0.7) as u32,
                (192.0 * 0.7) as u32,
            )),
        )
        .unwrap();
}

fn plant_right_trees(canvas: &mut Canvas<Window>, right_tree_texture: &Texture) {
    let width = 178.0;
    let height = 222.0;

    let right_trees = [
        (200, 555, 0.5),
        (330, 390, 0.5),
        (0, 380, 0.5),
        (488, 316, 0.5),
        (208, 100, 0.4),
    ];

    for tree in right_trees {
        canvas
            .copy(
                right_tree_texture,
                None,
                Some(Rect::new(
                    tree.0,
                    tree.1,
                    (width * tree.2) as u32,
                    (height * tree.2) as u32,
                )),
            )
            .unwrap();
    }
}

fn plant_little_trees(canvas: &mut Canvas<Window>, little_tree_texture: &Texture) {
    let width = 85.0;
    let height = 89.0;

    let little_trees = [
        (336, 64, 0.7),
        (342, 210, 0.6),
        (393, 330, 0.5),
        (236, 55, 0.5),
        (236, 0, 0.4),
        (342, 468, 0.5),
    ];

    for tree in little_trees {
        canvas
            .copy(
                little_tree_texture,
                None,
                Some(Rect::new(
                    tree.0,
                    tree.1,
                    (width * tree.2) as u32,
                    (height * tree.2) as u32,
                )),
            )
            .unwrap();
    }
}

fn plant_tree_top(canvas: &mut Canvas<Window>, tree_top_texture: &Texture) {
    canvas
        .copy(
            tree_top_texture,
            None,
            Some(Rect::new(
                520,
                214,
                (178.0 * 0.4) as u32,
                (222.0 * 0.4) as u32,
            )),
        )
        .unwrap();
}

pub fn plant(
    canvas: &mut Canvas<Window>,
    left_trees_texture: &Texture,
    right_tree_texture: &Texture,
    little_tree_texture: &Texture,
    tree_top_texture: &Texture,
) {
    plant_left_trees(canvas, left_trees_texture);
    plant_right_trees(canvas, right_tree_texture);
    plant_little_trees(canvas, little_tree_texture);
    plant_tree_top(canvas, tree_top_texture);
}
