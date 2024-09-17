use raylib::prelude::*;

mod graph;

fn main() {
    let (mut rl, thread) = init()
        .size(1280, 720)
        .title("Electron Architect")
        .build();

    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
}
