use macroquad::prelude::collections::storage;
use macroquad::prelude::*;
use macroquad_platformer::World as CollisionWorld;
use macroquad_tiled as tiled;

mod gui;
mod nodes;

use gui::Scene;

struct ExplosionTextures {
    deg_90: Texture2D,
    fourway: Texture2D,
    side: Texture2D,
    tail: Texture2D,
    threeway: Texture2D,
    twoway: Texture2D,
}
struct Resources {
    player: Texture2D,
    tiled_map: tiled::Map,
    bg_1: Texture2D,
    collision_world: CollisionWorld,
    bomb: Texture2D,
    fire: ExplosionTextures,
}

impl Resources {
    async fn new() -> Result<Resources, macroquad::prelude::FileError> {
        let bomb = load_texture("assets/tiles/bomb.png").await?;
        bomb.set_filter(FilterMode::Nearest);

        let expl_90deg = load_texture("assets/tiles/explosion/explosion-90deg.png").await?;
        let expl_fourway = load_texture("assets/tiles/explosion/explosion-fourway.png").await?;
        let expl_side = load_texture("assets/tiles/explosion/explosion-side.png").await?;
        let expl_tail = load_texture("assets/tiles/explosion/explosion-tail.png").await?;
        let expl_threeway = load_texture("assets/tiles/explosion/explosion-threeway.png").await?;
        let expl_twoway = load_texture("assets/tiles/explosion/explosion-twoway.png").await?;
        let player = load_texture("assets/tiles/ronalds(32x32).png").await?;
        player.set_filter(FilterMode::Nearest);

        let bg_1 = load_texture("assets/tileset.png").await?;
        bg_1.set_filter(FilterMode::Nearest);

        let tiled_map_json = load_string("assets/Tiled_BaseMap.json").await.unwrap();
        let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", bg_1)], &[]).unwrap();

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
            bg_1,
            bomb,
            player,
            fire: ExplosionTextures {
                deg_90: expl_90deg,
                fourway: expl_fourway,
                side: expl_side,
                tail: expl_tail,
                threeway: expl_threeway,
                twoway: expl_twoway,
            },
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
