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

// lazy_static! {
//     static ref DIRECTIONS: Vec<Vec2> = vec![
//         vec2(0., TILE_SIZE), // Up
//         vec2(-TILE_SIZE, 0.), // Left
//         vec2(0., -TILE_SIZE), // Down
//         vec2(TILE_SIZE, 0.), // Right
//     ];
// }

const DIRECTIONS: [(u8, f32, f32); 4] = [
    (0b10000000u8, 0., TILE_SIZE),
    (0b01000000u8, -TILE_SIZE, 0.),
    (0b00100000u8, 0., -TILE_SIZE),
    (0b00010000u8, TILE_SIZE, 0.),
];

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

    fn draw_middle(&self, middle_facing: u8) {
        let resources = storage::get::<Resources>();

        let (texture, params) = match middle_facing {
            // Check for all the positions of a threeway explosion
            0b11100000u8 => (resources.fire.threeway, DrawTextureParams::default()),
            0b01110000u8 => (
                resources.fire.threeway,
                DrawTextureParams {
                    rotation: PI / 2.,
                    ..Default::default()
                },
            ),
            0b10110000u8 => (
                resources.fire.threeway,
                DrawTextureParams {
                    rotation: PI,
                    ..Default::default()
                },
            ),
            0b11010000u8 => (
                resources.fire.threeway,
                DrawTextureParams {
                    rotation: -PI / 2.,
                    ..Default::default()
                },
            ),

            // Check for all the positions of a 90deg explosion
            0b11000000u8 => (resources.fire.deg_90, DrawTextureParams::default()),
            0b01100000u8 => (
                resources.fire.deg_90,
                DrawTextureParams {
                    rotation: PI / 2.,
                    ..Default::default()
                },
            ),
            0b00110000u8 => (
                resources.fire.deg_90,
                DrawTextureParams {
                    rotation: -PI,
                    ..Default::default()
                },
            ),
            0b10010000u8 => (
                resources.fire.deg_90,
                DrawTextureParams {
                    rotation: -PI / 2.,
                    ..Default::default()
                },
            ),
            // Check for all the positions of a two way explosion
            0b10100000u8 => (resources.fire.twoway, DrawTextureParams::default()),
            0b01010000u8 => (
                resources.fire.twoway,
                DrawTextureParams {
                    rotation: -PI / 2.,
                    ..Default::default()
                },
            ),

            _ => {
                log::debug!("{}", middle_facing);
                (resources.fire.fourway, DrawTextureParams::default())
            }
        };

        draw_texture_ex(texture, self.pos.x, self.pos.y, color::WHITE, params)
    }
}

impl scene::Node for Fire {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();

        let mut middle_facing = 0b0000000u8;

        for (i, (mask, x, y)) in DIRECTIONS.iter().enumerate() {
            // log::debug!("{} {}", dir, i);

            if (resources
                .collision_world
                .collide_solids(node.pos + vec2(*x, *y), 1, 1))
                == false
            {
                middle_facing = mask | middle_facing;

                match node.bomb_type {
                    BombType::Basic => draw_texture_ex(
                        resources.fire.side,
                        node.pos.x + *x,
                        node.pos.y + *y,
                        color::WHITE,
                        DrawTextureParams::default(),
                    ),
                }
            }
        }

        node.draw_middle(middle_facing);

        let mut middle_texture = resources.fire.fourway;

        fn update(mut node: RefMut<Fire>) {
            node.delete_in_seconds -= get_frame_time();
            if node.delete_in_seconds <= 0. {
                node.delete();
            }
        }
    }
}
