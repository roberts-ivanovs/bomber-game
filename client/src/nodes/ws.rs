use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
use std::sync::Arc;

use bomber_shared::messages::message::{self, PlayerID, Username};
use bomber_shared::messages::PlayerStateBits;
use macroquad::prelude::coroutines::{start_coroutine, wait_seconds};
use macroquad::prelude::scene::{self, Handle, RefMut};

use macroquad::prelude::*;

use quad_net::quad_socket::client::QuadSocket;
use sapp_console_log::console::{self, debug};

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
    remote_players: BTreeMap<PlayerID, Handle<RemotePlayer>>,
}

impl WebSocketClient {
    pub async fn new() -> Self {
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
        }
    }
}

impl scene::Node for WebSocketClient {
    fn ready(mut node: RefMut<Self>) {
        // node.socket.send_bin(&(01)); // TODO Send info about game hash (?)
        node.socket
            .send_bin(&message::MessagesClientTx::JoinLobbyClient {
                username: Username("Test".to_owned()),
            });
    }

    fn update(mut node: RefMut<Self>)
    where
        Self: Sized,
    {
        // Step 1: TX new state
        {
            let network_frame =
                get_time() - node.network_cache.last_send_time > (1. / NETWORK_FPS) as f64;

            match scene::find_node_by_type::<Player>() {
                Some(player) if network_frame => {
                    node.network_cache.last_send_time = get_time();

                    let mut state = PlayerStateBits([0; 4]);
                    state.set_x(player.pos().x as u32);
                    state.set_y(player.pos().y as u32);
                    state.set_dead(false); // TODO nobody can die yet

                    if node.network_cache.sent_position != state.0 {
                        node.network_cache.sent_position = state.0;
                        node.socket
                            .send_bin(&message::MessagesClientTx::PlayerStateClient(
                                message::PlayerState(state.0),
                            ));
                    }
                }
                _ => {}
            }
        }
        // Step 2: RX receive state
        while let Some(msg) = node.socket.try_recv_bin::<message::MessagesServerTx>() {
            match msg {
                message::MessagesServerTx::PlayerStateServer { client, player_id } => {
                    log::debug!("msg client {:?} player_id {:?}", &client, &player_id);
                    node.remote_players.get_mut(&player_id).and_then(|h| {
                        let mut other = scene::get_node(*h);
                        let state = PlayerStateBits(client.0);
                        other.set_pos(vec2(state.x() as f32, state.y() as f32));
                        other.set_dead(state.dead());
                        Some(h)
                    });
                }
                message::MessagesServerTx::JoinLobbyServer {
                    username,
                    player_id,
                } => {
                    log::debug!("Before adding");
                    let player = RemotePlayer::new(username.0, player_id);
                    log::debug!("Got player");
                    let remote_player = scene::add_node(player);
                    log::debug!("After adding");
                    node.remote_players.insert(player_id, remote_player);
                }
            }
        }
    }

    fn draw(_node: RefMut<Self>)
    where
        Self: Sized,
    {
        // TODO: Draw the read game state
    }
}
