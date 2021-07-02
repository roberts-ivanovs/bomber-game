use macroquad::prelude::*;
use macroquad_platformer::World as CollisionWorld;
use macroquad_tiled as tiled;

mod nodes;

use macroquad_platformer::*;

pub mod consts {
    pub const RUN_SPEED: f32 = 300.0;
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

#[macroquad::main("Bomber")]
async fn main() {
    fn convert_to_absolute(num: f32) -> f32 {
        return num * 32.;
    }

    // set height and width in tiles by 32x32
    let width = 500.;
    let height = 500.;

    // load textures
    let tilemap = load_texture("assets/tilemap.png").await.unwrap();

    // initialize tilemap
    let tiled_map_json = load_string("assets/Tiled_BaseMap.json").await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, &[("tilemap.png", tilemap)], &[]).unwrap();

    loop {
        clear_background(WHITE);

        tiled_map.draw_tiles(
            "main layer",
            Rect::new(
                0.0,
                0.0,
                width,
                height,
            ),
            None,
        );

        next_frame().await;
    }
}
