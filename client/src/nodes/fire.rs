use macroquad::{
    color,
    experimental::{
        collections::storage,
        coroutines::{start_coroutine, wait_seconds},
        scene::{self, RefMut},
    },
    prelude::*,
};

use crate::physics::Textures;
use resources::Resources;

pub struct Fire {
    pos: Vec2,
    delete_in_seconds: f32,
}

impl Fire {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            delete_in_seconds: 1.,
        }
    }
}

impl scene::Node for Fire {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();
        let mut texture = resources.get_mut::<Textures>().unwrap();

        draw_texture_ex(
            texture.fire.fourway,
            node.pos.x,
            node.pos.y,
            color::WHITE,
            DrawTextureParams::default(),
        );
    }

    fn update(mut node: RefMut<Self>) {
        node.delete_in_seconds -= get_frame_time();
        if node.delete_in_seconds <= 0. {
            node.delete();
        }
    }
}
