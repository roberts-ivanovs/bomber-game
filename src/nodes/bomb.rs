use macroquad::{
    experimental::{
        collections::storage,
        coroutines::{start_coroutine, wait_seconds},
        scene::{self, RefMut},
    },
    prelude::*,
};

use crate::Resources;

use super::player::Player;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BombType {
    Basic,
}

struct Bomb {
    player: scene::Handle<Player>,
    pos: Vec2,
    detonation_in_milliseconds: f32,
    bomb_type: BombType,
}

impl Bomb {
    fn new(pos: Vec2, player: scene::Handle<Player>) -> Self {
        Self {
            pos,
            player,
            detonation_in_milliseconds: 3000., // explore after three seconds
            bomb_type: BombType::Basic,
        }
    }
}

impl scene::Node for Bomb {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get_mut::<Resources>();

        // TODO The params may be off. not tested.
        resources.tiled_map.spr_ex(
            "tileset",
            Rect::new(0.0 * 32.0, 6.0 * 32.0, 32.0, 32.0),
            Rect::new(
                node.pos.x - (32.0 - 32.) / 2.,
                node.pos.y - (32.0 - 32.) / 2.,
                32.0,
                32.0,
            ),
        );

        match node.bomb_type {
            BombType::Basic => draw_texture_ex(
                resources.bomb,
                node.pos.x,
                node.pos.y,
                WHITE,
                // TODO The params may be off. not tested.
                DrawTextureParams {
                    source: Some(Rect::new(0.0, 0.0, 64., 32.)),
                    dest_size: Some(vec2(32., 16.)),
                    ..Default::default()
                },
            ),
        }
    }

    fn update(mut node: RefMut<Self>) {
        let mut resources = storage::get_mut::<Resources>();
        let mut player = scene::get_node(node.player);
        let mut others = scene::find_nodes_by_type::<Player>();

        node.detonation_in_milliseconds -= get_frame_time() * 1000.;
        if node.detonation_in_milliseconds <= 0. {
            // TODO kill all nearby players && host player
        }
    }
}
