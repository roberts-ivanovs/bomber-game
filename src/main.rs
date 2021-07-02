use macroquad::prelude::*;

mod nodes;

use macroquad_platformer::*;

pub mod consts {
    pub const RUN_SPEED: f32 = 300.0;
}

struct Resources {
    bomber: Texture2D,
    physics: World,
}

#[macroquad::main("Bomber")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}
