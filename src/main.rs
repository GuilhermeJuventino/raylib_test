use raylib::prelude::*;

// function for drawing things to the screen
fn draw(handle: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = handle.begin_drawing(&thread);

    d.clear_background(Color::WHITE);
    d.draw_text("Hello world", 12, 12, 20, Color::BLACK);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Raylib Test")
        .build();

    while !rl.window_should_close() {
        draw(&mut rl, &thread);
    }
}
