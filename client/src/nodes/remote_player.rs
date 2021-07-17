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
    pub id: String,
    bomber: Bomber,

    pub dead: bool,
    pub ready: bool,
    pos_delta: Vec2,
    last_move_time: f64,
}

impl RemotePlayer {
    pub fn new(username: &str, id: &str) -> RemotePlayer {
        let pos = vec2(100., 105.);

        RemotePlayer {
            bomber: Bomber::new(pos),
            username: username.to_string(),
            id: id.to_string(),
            pos_delta: vec2(0.0, 0.0),
            last_move_time: 0.0,
            ready: false,
            dead: false,
        }
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.last_move_time = get_time();
        self.pos_delta = pos - *self.bomber.pos();
        self.bomber.set_pos(pos);
    }

    pub fn set_dead(&mut self, dead: bool) {
        self.dead = dead;
    }

}
impl scene::Node for RemotePlayer {
    fn draw(mut node: RefMut<Self>) {
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
