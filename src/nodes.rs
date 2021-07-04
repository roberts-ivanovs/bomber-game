use macroquad::prelude::collections::storage;
use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

use crate::gui::Scene;

mod bomb;
mod player;

pub mod consts {
    pub const RUN_SPEED: f32 = 300.0;
    pub const TILE_SIZE: f32 = 32.;
}

fn convert_to_absolute(num: f32) -> f32 {
    return num * consts::TILE_SIZE;
}
pub async fn main_game() -> Scene {
    let tilemap = load_texture("assets/tilemap.png").await.unwrap();

    // initialize tilemap
    let tiled_map_json = load_string("assets/Tiled_BaseMap.json").await.unwrap();
    let tileset_json = load_string("assets/Tiled_Tiles.json").await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, &[("tilemap.png", tilemap)], &[]).unwrap();

    let w = tiled_map.raw_tiled_map.tilewidth * tiled_map.raw_tiled_map.width;
    let h = tiled_map.raw_tiled_map.tileheight * tiled_map.raw_tiled_map.height;

    loop {
        clear_background(WHITE);

        tiled_map.draw_tiles("main layer", Rect::new(0.0, 0.0, w as f32, h as f32), None);

        next_frame().await;
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
    Scene::MainMenu
}
