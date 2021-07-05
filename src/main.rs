use macroquad::prelude::collections::storage;
use macroquad::prelude::*;
use macroquad_platformer::World as CollisionWorld;
use macroquad_tiled as tiled;

mod gui;
mod nodes;

use gui::Scene;

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

fn window_conf() -> Conf {
    Conf {
        window_title: "Ether Bomber".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    // load textures
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
            Scene::Game => {
                next_scene = nodes::main_game().await;
            }
        }
    }
}
