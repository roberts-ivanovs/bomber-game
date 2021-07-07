use macroquad::{
    color,
    experimental::{
        collections::storage,
        coroutines::{start_coroutine, wait_seconds},
        scene::{self, RefMut},
    },
    prelude::*,
};

use crate::Resources;

use super::player::{Bomber, Player};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BombType {
    Basic,
}

pub struct Bomb {
    pos: Vec2,
    detonation_in_milliseconds: f32,
    bomb_type: BombType,
}

impl Bomb {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            detonation_in_milliseconds: 3000., // explore after three seconds
            bomb_type: BombType::Basic,
        }
    }
}

impl scene::Node for Bomb {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();
        match node.bomb_type {
            BombType::Basic => {
                draw_texture_ex(
                    resources.bomb,
                    node.pos.x,
                    node.pos.y,
                    color::WHITE,
                    DrawTextureParams::default(),
                );
            }
        }
    }

    fn update(mut node: RefMut<Self>) {
        node.detonation_in_milliseconds -= get_frame_time() * 1000.;
        if node.detonation_in_milliseconds <= 0. {
            // TODO kill all nearby players && host player
            let x = node.pos.x;
            let y = node.pos.y;
            node.delete();
            scene::add_node(super::fire::Fire::new(vec2(x, y)));
        }
    }
}
