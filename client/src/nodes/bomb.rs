use std::vec;

use macroquad::prelude::*;

use macroquad_platformer::Actor;

use macroquad::{
    audio::{self, play_sound_once},
    color,
    experimental::{
        animation::{AnimatedSprite, Animation},
        coroutines::{start_coroutine, wait_seconds, Coroutine},
        state_machine::{State, StateMachine},
    },
    prelude::*,
    ui::{self, hash},
};

use crate::Resources;
use macroquad::experimental::{collections::storage, scene::RefMut};

use super::player::{Bomber, Player};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BombType {
    Basic,
}

pub struct Bomb {
    pub pos: Vec2,
    visual_scale: f32,
    detonation_in_milliseconds: f32,
    bomb_type: BombType,
}

impl Bomb {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            visual_scale: 1.,
            detonation_in_milliseconds: 3000., // explode after three seconds
            bomb_type: BombType::Basic,
        }
    }
}

impl scene::Node for Bomb {
    fn ready(node: RefMut<Self>) {
        // node.detonation_in_milliseconds -= get_frame_time() * 1000.;
        // if node.detonation_in_milliseconds <= 0. {
        //     // TODO kill all nearby players && host player
        //     let x = node.pos.x;
        //     let y = node.pos.y;
        //     node.delete();
        //     scene::add_node(super::fire::Fire::new(vec2(x, y)));
        // }
        let handle = node.handle();

        let detonation = node.detonation_in_milliseconds;

        let bomb_pos = vec2(node.pos.x, node.pos.y);

        let bomb_type = node.bomb_type;

        start_coroutine(async move {
            let n = 25;
            for i in 0..n {
                // if player pick up the item real quick - the node may be already removed here
                if let Some(mut this) = scene::try_get_node(handle) {
                    this.visual_scale =
                        1. - (i as f32 / n as f32 * std::f32::consts::PI).sin() * 0.2;
                }

                next_frame().await;
            }
        });

        start_coroutine(async move {
            wait_seconds(detonation / 1000.).await;
            // TODO kill all nearby players && host player
            scene::add_node(super::fire::Fire::new(bomb_pos, bomb_type));

            if let Some(this) = scene::try_get_node(handle) {
                this.delete();
            }
        });
    }
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();

        match node.bomb_type {
            BombType::Basic => {
                draw_texture_ex(
                    resources.bomb,
                    node.pos.x - (32.0 * node.visual_scale - 32.) / 2.,
                    node.pos.y - (32.0 * node.visual_scale - 32.) / 2.,
                    color::WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(32.0 * node.visual_scale, 32.0 * node.visual_scale)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}
