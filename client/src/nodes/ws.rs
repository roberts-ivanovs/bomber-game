use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
use std::sync::Arc;

use bomber_shared::messages::message;
use bomber_shared::messages::PlayerStateBits;
use macroquad::prelude::coroutines::{start_coroutine, wait_seconds};
use macroquad::prelude::scene::{self, Handle, RefMut};

use macroquad::prelude::*;

use quad_net::quad_socket::client::QuadSocket;
use sapp_console_log::console;

use super::player::Player;
use super::remote_player::RemotePlayer;

const NETWORK_FPS: f32 = 15.;

struct NetworkCache {
    sent_position: [u8; 4],
    last_send_time: f64,
}

impl NetworkCache {
    fn flush(&mut self) {
        self.sent_position = [0; 4];
        self.last_send_time = 0.0;
    }
}
pub struct WebSocketClient {
    network_cache: NetworkCache,
    socket: QuadSocket,
    remote_players: BTreeMap<String, Handle<RemotePlayer>>,
    network_ids: BTreeSet<String>,
}

impl WebSocketClient {
    pub async fn new(network_id: String) -> Self {
        debug!("\n\n\ntest");
        let socket = QuadSocket::connect("ws://127.0.0.1:3030").unwrap();
        #[cfg(target_arch = "wasm32")]
        {
            while socket.is_wasm_websocket_connected() == false {
                next_frame().await;
            }
        }

        Self {
            socket,
            network_cache: NetworkCache {
                sent_position: [0; 4],
                last_send_time: 0.0,
            },
            remote_players: BTreeMap::new(),
            network_ids: {
                let mut network_ids = BTreeSet::new();
                network_ids.insert(network_id.clone());
                network_ids
            },
        }
    }
}

impl scene::Node for WebSocketClient {
    fn ready(mut node: RefMut<Self>) {
        node.socket.send_bin(&(01)); // TODO Send info about game hash (?)
    }

    fn update(mut node: RefMut<Self>)
    where
        Self: Sized,
    {
        // Step 1: TX new state
        {
            if let Some(player) = scene::find_node_by_type::<Player>() {
                // let network_frame =
                //     get_time() - node.network_cache.last_send_time > (1. / NETWORK_FPS) as f64;

                node.network_cache.last_send_time = get_time();

                let mut state = PlayerStateBits([0; 4]);
                state.set_x(player.pos().x as u32);
                state.set_y(player.pos().y as u32);
                state.set_dead(false); // TODO nobody can die yet

                if node.network_cache.sent_position != state.0 {
                    node.network_cache.sent_position = state.0;
                    node.socket.send_bin(&message::PlayerState(state.0));
                }
            }
        }
        // Step 2: RX receive state

        // while let Some((optcode)) = node.socket.try_recv_bin::<(u8)>() {
        // pos.x = x;
        // pos.y = y;
        // last_edit_id = id;
        // }
    }

    fn draw(_node: RefMut<Self>)
    where
        Self: Sized,
    {
        // TODO: Draw the read game state
    }
}
