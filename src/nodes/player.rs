use macroquad::prelude::*;

use macroquad_platformer::Actor;

use macroquad::experimental::{collections::storage, scene::RefMut};

use super::consts::RUN_SPEED;

use crate::Resources;

pub struct Bomber {
    pub collider: Actor,
    speed: Vec2,
}

impl Bomber {
    pub fn new(spawner_pos: Vec2) -> Self {
        let mut resources = storage::get_mut::<Resources>();

        Self {
            collider: resources.collision_world.add_actor(spawner_pos, 30, 30),
            speed: vec2(0., 0.),
        }
    }

    pub fn draw(&mut self) {
        let resources = storage::get::<Resources>();

        let pos = resources.collision_world.actor_pos(self.collider);

        draw_texture_ex(
            resources.player,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0.0, 0.0, 32., 32.)),
                ..Default::default()
            },
        );
    }
}

pub struct Player {
    pub bomber: Bomber,
}

impl Player {
    pub fn new(spawner_pos: Vec2) -> Self {
        Player {
            bomber: Bomber::new(spawner_pos),
        }
    }
}

impl scene::Node for Player {
    fn draw(mut node: RefMut<Self>) {
        node.bomber.draw();
    }

    fn update(mut node: RefMut<Self>) {
        let world = &mut storage::get_mut::<Resources>().collision_world;

        if is_key_down(KeyCode::Right) {
            node.bomber.speed.x = RUN_SPEED;
        } else if is_key_down(KeyCode::Left) {
            node.bomber.speed.x = -RUN_SPEED;
        } else if is_key_down(KeyCode::Up) {
            node.bomber.speed.y = RUN_SPEED;
        } else if is_key_down(KeyCode::Down) {
            node.bomber.speed.y = -RUN_SPEED;
        } else {
            node.bomber.speed.x = 0.;
            node.bomber.speed.y = 0.;
        }

        world.move_h(node.bomber.collider, node.bomber.speed.x * get_frame_time());
        world.move_v(node.bomber.collider, node.bomber.speed.y * get_frame_time());
    }
}
