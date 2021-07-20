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

pub struct Destroyable {}

impl Destroyable {
    pub fn new() -> Destroyable {
        Destroyable {}
    }
}

impl scene::Node for Destroyable {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();

        let textures = resources.get_mut::<Textures>().unwrap();
        let map_size = resources.get_mut::<MapSize>().unwrap();

        textures.tiled_map.draw_tiles(
            "destroyable_walls",
            Rect::new(0., 0., map_size.w as f32, map_size.h as f32),
            None,
        );

        // resources.tiled_map.
    }
}
