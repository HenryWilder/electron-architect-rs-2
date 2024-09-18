#![allow(unused)]

use std::{cell::RefCell, rc::{Rc, Weak}};

use graph::{node::{gate::Gate, Node}, Graph};
use raylib::prelude::*;
use vector2i::Vector2i;

mod vector2i;
mod graph;

fn main() {
    let window_width = 1280.0;
    let window_height = 720.0;
    let (mut rl, thread) = init()
        .size(window_width as i32, window_height as i32)
        .title("Electron Architect")
        .build();

    const GRID_COLOR: Color = Color::new(16, 16, 16, 255);

    let mut graph = Graph::new();
    let grid_bottom_right = graph.world_to_grid(Vector2::new(
        window_width  + Graph::GRID_SIZE,
        window_height + Graph::GRID_SIZE,
    ));

    let mut current_node: Option<Weak<RefCell<Node>>> = None;

    while !rl.window_should_close() {
        let mouse_cell = graph.world_to_grid(rl.get_mouse_position());
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let new_node = graph.put_node(Gate::Always, mouse_cell);
            if let Some(current_node) = current_node {
                if let (Some(input), Some(output)) = (current_node.upgrade(), new_node.upgrade()) {
                    graph.wire(input, output);
                }
            }
            current_node = Some(new_node);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Draw grid
        for n in 0..grid_bottom_right.x.max(grid_bottom_right.y) {
            let Vector2 { x, y } = graph.grid_to_world(Vector2i::new(n, n));
            if x <= window_width  { d.draw_rectangle_rec(Rectangle::new(x - 1.0, 0.0, 2.0, window_height), GRID_COLOR); }
            if y <= window_height { d.draw_rectangle_rec(Rectangle::new(0.0, y - 1.0, window_width,  2.0), GRID_COLOR); }
        }

        graph.draw_wires(&mut d);
        graph.draw_proxy_nodes(&mut d);
    }
}
