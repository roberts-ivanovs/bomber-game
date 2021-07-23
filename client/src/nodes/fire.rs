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

            _ => (resources.fire.fourway, DrawTextureParams::default()),
        };

        draw_texture_ex(texture, self.pos.x, self.pos.y, color::WHITE, params)
    }

    fn draw_side(&self, x: &f32, y: &f32, length: i32) {
        let resources = storage::get::<Resources>();

        let params = if *x == DIRECTIONS[0].1 && *y == DIRECTIONS[0].2
            || *x == DIRECTIONS[2].1 && *y == DIRECTIONS[2].2 && length != 0
        {
            DrawTextureParams {
                rotation: PI / 2.,
                ..Default::default()
            }
        } else {
            DrawTextureParams::default()
        };

        draw_texture_ex(
            resources.fire.side,
            self.pos.x + x * length as f32,
            self.pos.y + y * length as f32,
            color::WHITE,
            params,
        )
    }

    fn draw_tail(&self, x: &f32, y: &f32, max_length: i32) {
        let resources = storage::get::<Resources>();

        let params = if *x == DIRECTIONS[0].1 && *y == DIRECTIONS[0].2 {
            DrawTextureParams {
                rotation: PI / 2.,
                ..Default::default()
            }
        } else if *x == DIRECTIONS[2].1 && *y == DIRECTIONS[2].2 {
            DrawTextureParams {
                rotation: -PI / 2.,
                ..Default::default()
            }
        } else if *x == DIRECTIONS[1].1 && *y == DIRECTIONS[1].2 {
            DrawTextureParams {
                rotation: PI,
                ..Default::default()
            }
        } else {
            DrawTextureParams::default()
        };

        draw_texture_ex(
            resources.fire.tail,
            self.pos.x + x * max_length as f32,
            self.pos.y + y * max_length as f32,
            color::WHITE,
            params,
        )
    }

    fn is_colliding_in_dir(&self, x: f32, y: f32) -> bool {
        let resources = storage::get::<Resources>();

        (resources
            .collision_world
            .collide_solids(vec2(self.pos.x + x, self.pos.y + y), 1, 1))
            == true
    }
}

impl scene::Node for Fire {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();

        let mut middle_facing = 0b0000000u8;

        for (i, (mask, x, y)) in DIRECTIONS.iter().enumerate() {
            if !node.is_colliding_in_dir(*x, *y) {
                middle_facing = mask | middle_facing;

                let max_length = match node.bomb_type {
                    BombType::Basic => 1,
                };

                for i in 1..max_length {
                    if !node.is_colliding_in_dir(x * i as f32, y * i as f32) {
                        node.draw_side(x, y, i);
                    }
                }

                if !node.is_colliding_in_dir(*x * max_length as f32, *y * max_length as f32) {
                    node.draw_tail(x, y, max_length);
                }
            }
        }

        node.draw_middle(middle_facing);
    }

    fn update(mut node: RefMut<Fire>) {
        node.delete_in_seconds -= get_frame_time();
        if node.delete_in_seconds <= 0. {
            node.delete();
        }
    }
}
