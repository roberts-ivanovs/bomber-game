use macroquad::{
    experimental::{
        collections::storage,
        scene::{self, RefMut},
    },
    prelude::*,
};

use crate::Resources;

pub struct Destroyable {}

impl Destroyable {
    pub fn new() -> Destroyable {
        Destroyable {}
    }
}

impl scene::Node for Destroyable {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();

        resources
            .tiled_map
            .draw_tiles("destroyable_walls", Rect::new(0., 0., resources.map_size.w as f32, resources.map_size.h as f32), None);

        // resources.tiled_map.
    }
}
