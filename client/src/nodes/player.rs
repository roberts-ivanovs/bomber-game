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

use super::{
    bomb::{self, BombType},
    consts::RUN_SPEED,
    get_nearest_tile,
};

use crate::{js_interop::FromJS, Resources};

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
    input: Input,
    current_bomb_type: BombType,
}

impl Bomber {
    pub fn new(spawner_pos: Vec2) -> Self {
        let mut resources = storage::get_mut::<Resources>();
        Self {
            collider: resources.collision_world.add_actor(spawner_pos, 32, 32),
            input: Default::default(),
            speed: vec2(0., 0.),
            current_bomb_type: BombType::Basic,
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

    /// Get a reference to the bomber's pos.
    pub fn pos(&self) -> Vec2 {
        let resources = storage::get::<Resources>();
        resources.collision_world.actor_pos(self.collider)
    }

    /// Set the bomber's pos.
    pub fn set_pos(&mut self, pos: Vec2) {
        let mut resources = storage::get_mut::<Resources>();
        resources.collision_world.set_actor_position(self.collider, pos);
    }
}

pub struct Player {
    pub bomber: Bomber,
    bomb_place_time: f64,
    input: Input,
    state_machine: StateMachine<RefMut<Player>>,
}

impl Player {
    const ST_NORMAL: usize = 0;
    const ST_DEATH: usize = 1;
    const ST_PUTTING_BOMB: usize = 2;

    pub fn new(spawner_pos: Vec2) -> Self {
        let mut state_machine = StateMachine::new();
        state_machine.add_state(Self::ST_NORMAL, State::new().update(Self::update_normal));
        state_machine.add_state(Self::ST_DEATH, State::new());

        Player {
            bomber: Bomber::new(spawner_pos),
            bomb_place_time: 0.,
            state_machine,
            input: Default::default(),
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.bomber.pos()
    }


    fn update_normal(node: &mut RefMut<Player>, _dt: f32) {
        let node = &mut **node;
        let bomber = &mut node.bomber;

        if node.input.up {
            bomber.speed.y = -RUN_SPEED;
        } else if node.input.down {
            bomber.speed.y = RUN_SPEED;
        } else {
            bomber.speed.y = 0.;
        }

        if node.input.right {
            bomber.speed.x = RUN_SPEED;
        } else if node.input.left {
            bomber.speed.x = -RUN_SPEED;
        } else {
            bomber.speed.x = 0.;
        }

        if node.input.place_bomb {
            if get_time() - node.bomb_place_time > 3. {
                match bomber.current_bomb_type {
                    BombType::Basic => node.place_bomb(),
                }
            }
        }
    }

    fn place_bomb(&mut self) {
        let resources = storage::get::<Resources>();
        let pos = resources.collision_world.actor_pos(self.bomber.collider);
        scene::add_node(bomb::Bomb::new(get_nearest_tile(pos)));

        self.bomb_place_time = get_time();
    }
}

impl scene::Node for Player {
    fn ready(node: RefMut<Self>) {
        let handle = node.handle();
        // user meta data logging
        start_coroutine(async move {
            loop {
                wait_seconds(1.).await;
                if let Some(this) = scene::try_get_node(handle) {
                    log::trace!("player.pos\t: {:?}", this.bomber.pos());
                }
            }
        });
    }
    fn draw(mut node: RefMut<Self>) {
        node.bomber.draw();
    }

    fn update(mut node: RefMut<Self>) {
        node.input.up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
        node.input.left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        node.input.down = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);
        node.input.right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        node.input.place_bomb = is_key_down(KeyCode::Space);

        {
            let node = &mut *node;
            let bomber = &mut node.bomber;

            let mut resources = storage::get_mut::<Resources>();
            resources
                .collision_world
                .move_h(bomber.collider, bomber.speed.x * get_frame_time());
            resources
                .collision_world
                .move_v(bomber.collider, bomber.speed.y * get_frame_time());
        }
        StateMachine::update_detached(&mut node, |node| &mut node.state_machine);
    }
}
