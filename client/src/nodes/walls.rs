use macroquad::{
    experimental::{
        collections::storage,
        scene::{self, RefMut},
    },
    prelude::*,
};

use crate::Resources;

pub struct Walls {}

impl Walls {
    pub fn new() -> Walls {
        Walls {};
    }
}

impl scene::Node for Walls {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();

        draw_texture_ex(
            resources.tileset,
            100.0,
            100.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(1000.0, 1500.0)),
                ..Default::default()
            },
        );

        let w = resources.tiled_map.raw_tiled_map.tilewidth * resources.tiled_map.raw_tiled_map.width;
        let h = resources.tiled_map.raw_tiled_map.tileheight * resources.tiled_map.raw_tiled_map.height;

        resources
            .tiled_map
            .draw_tiles("walls", Rect::new(0., 0., w as f32, h as f32), None);
    }
}
