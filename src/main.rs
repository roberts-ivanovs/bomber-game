use macroquad::prelude::*;
use macroquad::prelude::collections::storage;
use macroquad_platformer::World as CollisionWorld;
use macroquad_tiled as tiled;

mod gui;
mod nodes;

use gui::Scene;

use macroquad_platformer::*;

pub mod consts {
    pub const RUN_SPEED: f32 = 300.0;
    pub const TILE_SIZE: f32 = 32.;
}

struct Resources {
    player: Texture2D,
    tiled_map: tiled::Map,
    collision_world: CollisionWorld,
    bomb: Texture2D,
}

impl Resources {
    async fn new() -> Result<Resources, macroquad::prelude::FileError> {
        let bomb = load_texture("assets/items/bomb.png").await?;
        let fire = load_texture("assets/items/fire.png").await?;
        let player = load_texture("assets/items/player.png").await?;

        let tileset = load_texture("assets/tileset.png").await?;
        tileset.set_filter(FilterMode::Nearest);

        let tiled_map_json = load_string("assets/map.json").await.unwrap();
        let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();

        let mut static_colliders = vec![];
        for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
            static_colliders.push(tile.is_some());
        }
        let mut collision_world = CollisionWorld::new();
        collision_world.add_static_tiled_layer(
            static_colliders,
            32.,
            32.,
            tiled_map.raw_tiled_map.width as _,
            1,
        );

        Ok(Resources {
            tiled_map,
            collision_world,
            bomb,
            player,
        })
    }
}

fn convert_to_absolute(num: f32) -> f32 {
    return num * consts::TILE_SIZE;
}

#[macroquad::main("Bomber")]
async fn main() {
    // load textures
    // let tilemap = load_texture("assets/tilemap.png").await.unwrap();

    // // initialize tilemap
    // let tiled_map_json = load_string("assets/Tiled_BaseMap.json").await.unwrap();
    // let tileset_json = load_string("assets/Tiled_Tiles.json").await.unwrap();
    // let tiled_map = tiled::load_map(&tiled_map_json, &[("tilemap.png", tilemap)], &[]).unwrap();

    // let w = tiled_map.raw_tiled_map.tilewidth * tiled_map.raw_tiled_map.width;
    // let h = tiled_map.raw_tiled_map.tileheight * tiled_map.raw_tiled_map.height;

    // loop {
    //     clear_background(WHITE);

    //     tiled_map.draw_tiles("main layer", Rect::new(0.0, 0.0, w as f32, h as f32), None);

    //     next_frame().await;
    // }

    let gui_resources = gui::GuiResources::new();
    storage::store(gui_resources);

    //let mut next_scene = gui::matchmaking_lobby().await;
    let mut next_scene = Scene::MainMenu;
    loop {
        match next_scene {
            Scene::MainMenu => {
                next_scene = gui::main_menu().await;
            }
            Scene::Credits => {
                next_scene = gui::credits().await;
            }
            Scene::Game => todo!(),
        }
    }
}
