use macroquad::{
    experimental::{
        collections::storage,
        scene::{self, RefMut},
    },
    prelude::*,
};

use crate::physics::MapSize;
use crate::physics::Textures;
use resources::Resources;

pub struct Walls {}

impl Walls {
    pub fn new() -> Walls {
        Walls {}
    }
}

impl scene::Node for Walls {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();

        let textures = resources.get_mut::<Textures>().unwrap();
        let map_size = resources.get_mut::<MapSize>().unwrap();

        textures.tiled_map.draw_tiles(
            "walls",
            Rect::new(0., 0., map_size.w as f32, map_size.h as f32),
            None,
        );
    }
}
