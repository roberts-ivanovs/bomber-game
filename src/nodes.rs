use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

use macroquad::experimental::{collections::storage, coroutines::start_coroutine};

use crate::gui::Scene;

use crate::Resources;

mod bomb;
mod camera;
mod level_bg;
mod player;
mod fire;

pub mod consts {
    pub const RUN_SPEED: f32 = 300.0;
    pub const TILE_SIZE: f32 = 32.;
}

fn convert_to_absolute(num: f32) -> f32 {
    return num * consts::TILE_SIZE;
}

pub async fn main_game() -> Scene {
    let resources_loading = start_coroutine(async move {
        let resources = Resources::new().await.unwrap();
        storage::store(resources);
    });

    while resources_loading.is_done() == false {
        clear_background(BLACK);
        draw_text(
            &format!(
                "Loading resources {}",
                ".".repeat(((get_time() * 2.0) as usize) % 4)
            ),
            screen_width() / 2.0 - 160.0,
            screen_height() / 2.0,
            40.,
            WHITE,
        );

        next_frame().await;
    }

    // let resources = storage::get::<Resources>();

    scene::add_node(level_bg::LevelBg::new());

    let player = scene::add_node(player::Player::new(vec2(32., 32.)));
    scene::add_node(bomb::Bomb::new(vec2(32., 32.), player));

    let resources = storage::get::<Resources>();
    let w = resources.tiled_map.raw_tiled_map.tilewidth * resources.tiled_map.raw_tiled_map.width;
    let h = resources.tiled_map.raw_tiled_map.tileheight * resources.tiled_map.raw_tiled_map.height;

    let camera = scene::add_node(camera::Camera::new(
        Rect::new(0.0, 0.0, w as f32, h as f32),
        400.0,
    ));

    loop {
        clear_background(WHITE);

        next_frame().await;
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
    Scene::MainMenu
}
