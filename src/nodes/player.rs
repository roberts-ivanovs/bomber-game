use macroquad::prelude::*;

use macroquad_platformer::Actor;

use macroquad::experimental::{
    collections::storage,
};

use crate::{
    consts,
    Resources,
};

pub struct Player {
    pub colider: Actor,
    pos: Vec2,
    speed: Vec2,
}

impl Player {
    pub fn new(spawner_pos: Vec2) -> Self {
        let mut resources = storage::get_mut::<Resources>();

        Self {
            colider: resources.physics.add_actor(spawner_pos, 30, 30),
            pos: spawner_pos,
            speed: vec2(0., 0.),
        }
    }
}
