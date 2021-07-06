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

use macroquad::experimental::{collections::storage, scene::RefMut};

use super::consts::RUN_SPEED;

use crate::Resources;

#[derive(Default, Debug, Clone)]
pub struct Input {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    place_bomb: bool,
}

pub struct Bomber {
    pub collider: Actor,
    speed: Vec2,
    pos: Vec2,
    input: Input,
    state_machine: StateMachine<RefMut<Player>>,
}

impl Bomber {
    const ST_NORMAL: usize = 0;

    pub fn new(spawner_pos: Vec2) -> Self {
        let mut resources = storage::get_mut::<Resources>();

        let mut state_machine = StateMachine::new();

        state_machine.add_state(Self::ST_NORMAL, State::new().update(Self::update_normal));

        Self {
            collider: resources.collision_world.add_actor(spawner_pos, 30, 30),
            pos: spawner_pos,
            input: Default::default(),
            state_machine,
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
    fn update_normal(node: &mut RefMut<Player>, _dt: f32) {
        let node = &mut **node;

        if node.input.up {
            node.bomber.speed.y = RUN_SPEED;
        } else if node.input.right {
            node.bomber.speed.x = RUN_SPEED;
        } else if node.input.left {
            node.bomber.speed.x = -RUN_SPEED;
        } else if node.input.down {
            node.bomber.speed.y = -RUN_SPEED;
        } else {
            node.bomber.speed.x = 0.;
            node.bomber.speed.y = 0.;
        }
    }
}

impl scene::Node for Bomber {
    fn draw(mut node: RefMut<Self>) {
        node.draw();
    }

    fn update(mut node: RefMut<Self>) {
        let world = &mut storage::get_mut::<Resources>().collision_world;

        node.input.up = is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W);
        node.input.left = is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A);
        node.input.down = is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S);
        node.input.right = is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D);

        // if is_key_down(KeyCode::Right) {
        //     node.bomber.speed.x = RUN_SPEED;
        // } else if is_key_down(KeyCode::Left) {
        //     node.bomber.speed.x = -RUN_SPEED;
        // } else if is_key_down(KeyCode::Up) {
        //     node.bomber.speed.y = RUN_SPEED;
        // } else if is_key_down(KeyCode::Down) {
        //     node.bomber.speed.y = -RUN_SPEED;
        // } else {
        //     node.bomber.speed.x = 0.;
        //     node.bomber.speed.y = 0.;
        // }

        world.move_h(node.collider, node.speed.x * get_frame_time());
        world.move_v(node.collider, node.speed.y * get_frame_time());
    }
}

pub struct Player {
    pub bomber: Bomber,
    pos: Vec2,
    input: Input,
    state_machine: StateMachine<RefMut<Player>>,
}

impl Player {
    const ST_NORMAL: usize = 0;

    pub fn new(spawner_pos: Vec2) -> Self {
        let mut state_machine = StateMachine::new();

        state_machine.add_state(Self::ST_NORMAL, State::new().update(Self::update_normal));

        Player {
            bomber: Bomber::new(spawner_pos),
            pos: spawner_pos,
            state_machine,
            input: Default::default(),
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos
    }

    fn update_normal(node: &mut RefMut<Player>, _dt: f32) {
        let node = &mut **node;

        if node.input.up {
            node.bomber.speed.y = RUN_SPEED;
        } else if node.input.right {
            node.bomber.speed.x = RUN_SPEED;
        } else if node.input.left {
            node.bomber.speed.x = -RUN_SPEED;
        } else if node.input.down {
            node.bomber.speed.y = -RUN_SPEED;
        } else {
            node.bomber.speed.x = 0.;
            node.bomber.speed.y = 0.;
        }
    }
}
