use macroquad::prelude::*;

use macroquad_platformer::Actor;

use macroquad::experimental::{
    collections::storage,
};

use crate::{
    consts,
    Resources,
};

pub struct Bomber {
    pub colider: Actor,
    pos: Vec2,
    speed: Vec2,
}

impl Bomber {
    pub fn new(spawner_pos: Vec2) -> Bomber {
        let mut resources = storage::get_mut::<Resources>();

        Bomber {
            colider: resources.physics.add_actor(spawner_pos, 30, 30),
            pos: spawner_pos,
            speed: vec2(0., 0.),
        }
    }
}
