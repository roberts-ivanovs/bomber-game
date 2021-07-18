use bomber_shared::messages::message::PlayerID;
use macroquad::{
    experimental::{
        coroutines::{start_coroutine, wait_seconds},
        scene::{self, Handle, RefMut},
    },
    prelude::*,
};

use super::player::Bomber;

pub struct RemotePlayer {
    pub username: String,
    pub id: PlayerID,
    bomber: Bomber,

    pub dead: bool,
    pub ready: bool,
    last_move_time: f64,
}

impl RemotePlayer {
    pub fn new(username: String, id: PlayerID) -> RemotePlayer {
        let empty_vec = vec2(0., 0.);
        RemotePlayer {
            id,
            bomber: Bomber::new(empty_vec),
            username,
            last_move_time: 0.0,
            ready: false,
            dead: false,
        }
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.last_move_time = get_time();
        self.bomber.set_pos(pos);
    }

    pub fn set_dead(&mut self, dead: bool) {
        self.dead = dead;
    }

}
impl scene::Node for RemotePlayer {
    fn draw(mut node: RefMut<Self>) {

        // Username
        draw_text_ex(
            &node.username,
            node.bomber.pos().x - 1.,
            node.bomber.pos().y - 1.,
            TextParams {
                font_size: 50,
                font_scale: 0.25,
                ..Default::default()
            },
        );

        node.bomber.draw();
    }
}
