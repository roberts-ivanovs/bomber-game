use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

use macroquad::experimental::{collections::storage, coroutines::start_coroutine};

use crate::gui::Scene;

use crate::Resources;

use self::consts::TILE_SIZE;
use self::ws::WebSocketClient;

mod bomb;
mod camera;
mod fire;
mod level_bg;
mod player;
mod remote_player;
pub mod ws;

pub mod consts {
    pub const RUN_SPEED: f32 = 100.0;
    pub const TILE_SIZE: f32 = 32.;
}

fn convert_to_absolute(num: f32) -> f32 {
    return num * consts::TILE_SIZE;
}

fn get_nearest_tile(loc: Vec2) -> Vec2 {
    vec2(
        (loc.x / TILE_SIZE).round() * TILE_SIZE,
        (loc.y / TILE_SIZE).round() * TILE_SIZE,
    )
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

    let ws_client = WebSocketClient::new().await;
    scene::add_node(ws_client);

    scene::add_node(level_bg::LevelBg::new());

    let player = scene::add_node(player::Player::new(vec2(32., 32.)));
    // scene::add_node(bomb::Bomb::new(vec2(32., 32.), player));

    let resources = storage::get::<Resources>();
    let w = resources.tiled_map.raw_tiled_map.tilewidth * resources.tiled_map.raw_tiled_map.width;
    let h = resources.tiled_map.raw_tiled_map.tileheight * resources.tiled_map.raw_tiled_map.height;
    drop(resources);

    let camera = scene::add_node(camera::Camera::new(352.0));

    loop {
        clear_background(WHITE);

        next_frame().await;
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
    Scene::MainMenu
}

pub async fn lobby() -> Scene {
    Scene::MainMenu
}
