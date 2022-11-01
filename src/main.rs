use raylib::prelude::*;

// constant variables
const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 480;
const MAX_FPS: u32 = 60;

const GRASS_TEXTURE: &str = "assets/Tilesets/ground_tiles/new_tiles/Grass_tiles_v2.png";

// game loop functions
fn input() {}

fn update() {}

fn render(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
) {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(
            147,
            211,
            196,
            255,
        ));
        // drawing scene

        d.draw_text("Hello World", 190, 200, 20, Color::LIGHTGRAY);
}

// fn init() {}

// fn quit() {}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raylib Test")
        .build();
    
    rl.set_exit_key(Some(KeyboardKey::KEY_ZERO));
    rl.set_target_fps(MAX_FPS);
    
    while !rl.window_should_close() {
        input();
        update();
        render(&mut rl, &thread);
    }
}