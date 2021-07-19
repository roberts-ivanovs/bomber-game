use std::{collections::HashMap, vec};

use lazy_static::lazy_static;
use macroquad::{
    color,
    experimental::{
        collections::storage,
        coroutines::{start_coroutine, wait_seconds},
        scene::{self, RefMut},
    },
    prelude::*,
};
use std::f32::consts::PI;

use crate::Resources;

use super::{bomb::BombType, consts::TILE_SIZE};

lazy_static! {
    static ref DIRECTIONS: Vec<Vec2> = vec![
        vec2(0., TILE_SIZE), // Up
        vec2(-TILE_SIZE, 0.), // Left
        vec2(0., -TILE_SIZE), // Down
        vec2(TILE_SIZE, 0.), // Right
    ];
}

// pub struct Direction {
//     up: Vec2,
//     left: Vec2,
//     down: Vec2,
//     right: Vec2,
// }

// impl Direction {
//     pub fn new(pos: Vec2) -> Self {
//         Self {
//             up: vec2(0., TILE_SIZE),
//             left: vec2(-TILE_SIZE, 0.),
//             down: vec2(0., -TILE_SIZE),
//             right: vec2(TILE_SIZE, 0.),
//         }
//     }
// }

pub struct Fire {
    pos: Vec2,
    delete_in_seconds: f32,
    bomb_type: BombType,
}

impl Fire {
    pub fn new(pos: Vec2, bomb_type: BombType) -> Self {
        Self {
            pos,
            delete_in_seconds: 1.,
            bomb_type,
        }
    }
    // pub fn generate_in_direction(&self, dir: Vec2) {}

    // pub fn draw_middle(&self) -> Texture2D {
    //     let resources = storage::get::<Resources>();

    //     resources.fire.fourway
    // }
}

impl scene::Node for Fire {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();

        let mut middle_facing: Vec<bool> = vec![false, false, false, false];

        for (i, dir) in DIRECTIONS.iter().enumerate() {
            log::debug!("{} {}", dir, i);

            if (resources
                .collision_world
                .collide_solids(node.pos + *dir, 1, 1))
                == false
            {
                middle_facing[i] = true;

                match node.bomb_type {
                    BombType::Basic => draw_texture_ex(
                        resources.fire.side,
                        node.pos.x + dir.x,
                        node.pos.y + dir.y,
                        color::WHITE,
                        DrawTextureParams::default(),
                    ),
                }
            }
        }

        let mut middle_texture = resources.fire.fourway;

        // Check for all the positions of a three way explosion
        if middle_facing == vec![true, true, true, false] {
            draw_texture_ex(
                resources.fire.threeway,
                node.pos.x,
                node.pos.y,
                color::WHITE,
                DrawTextureParams::default(),
            );
        } else if middle_facing == vec![false, true, true, true] {
            draw_texture_ex(
                resources.fire.threeway,
                node.pos.x,
                node.pos.y,
                color::WHITE,
                DrawTextureParams {
                    rotation: -PI / 2.,
                    ..Default::default()
                },
            );
        } else if middle_facing == vec![true, false, true, true] {
            draw_texture_ex(
                resources.fire.threeway,
                node.pos.x,
                node.pos.y,
                color::WHITE,
                DrawTextureParams {
                    rotation: PI,
                    ..Default::default()
                },
            );
        } else if middle_facing == vec![true, true, false, true] {
            draw_texture_ex(
                resources.fire.threeway,
                node.pos.x,
                node.pos.y,
                color::WHITE,
                DrawTextureParams {
                    rotation: PI / 2,
                    ..Default::default()
                },
            );
        } else {
            draw_texture_ex(
                resources.fire.fourway,
                node.pos.x,
                node.pos.y,
                color::WHITE,
                DrawTextureParams::default(),
            );
        }
    }

    fn update(mut node: RefMut<Self>) {
        node.delete_in_seconds -= get_frame_time();
        if node.delete_in_seconds <= 0. {
            node.delete();
        }
    }
}
