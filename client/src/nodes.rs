use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

use resources::Resources;

use super::physics::init_resources;

use macroquad::experimental::{collections::storage, coroutines::start_coroutine};

use crate::gui::Scene;

use self::ws::WebSocketClient;
use super::constants::consts;

mod bomb;
mod camera;
mod destroyable;
mod fire;
mod level_bg;
mod player;
mod remote_player;
mod walls;
pub mod ws;

fn convert_to_absolute(num: f32) -> f32 {
    return num * consts::TILE_SIZE;
}

fn get_nearest_tile(loc: Vec2) -> Vec2 {
    vec2(
        (loc.x / consts::TILE_SIZE).round() * consts::TILE_SIZE,
        (loc.y / consts::TILE_SIZE).round() * consts::TILE_SIZE,
    )
}

pub async fn main_game() -> Scene {
    let resources_loading = start_coroutine(async move {
        storage::store(init_resources().await);
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

    scene::add_node(walls::Walls::new());

    scene::add_node(level_bg::LevelBg::new());

    scene::add_node(destroyable::Destroyable::new());

    scene::add_node(player::Player::new(vec2(32., 32.)));

    scene::add_node(camera::Camera::new(352.0));

    let ws_client = WebSocketClient::new().await;
    scene::add_node(ws_client);

    loop {
        clear_background(WHITE);

        next_frame().await;
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
    Scene::MainMenu
}
