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
        Walls {}
    }
}

impl scene::Node for Walls {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();

        resources
            .tiled_map
            .draw_tiles("walls", Rect::new(0., 0., resources.map_size.w as f32, resources.map_size.h as f32), None);
    }
}
