use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::mem;
use std::rc::Rc;
use std::sync::Arc;

use bomber_shared::messages::message::{self, PlayerID, Username};
use bomber_shared::messages::PlayerStateBits;
use macroquad::prelude::coroutines::{start_coroutine, wait_seconds};
use macroquad::prelude::scene::{self, Handle, RefMut};

use macroquad::prelude::*;

use macroquad_platformer::Actor;
use quad_net::quad_socket::client::QuadSocket;
use sapp_console_log::console::{self, debug};

use super::player::Player;
use super::remote_player::RemotePlayer;
use super::ws::WebSocketClient;

const NETWORK_FPS: f32 = 15.;

#[derive(Debug)]
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

#[derive(Debug)]
enum SocketCommMode {
    Lobby {
        username: Username,
        remote_players: BTreeMap<PlayerID, Username>,
    },
    RealGame {
        remote_players: BTreeMap<PlayerID, Handle<RemotePlayer>>,
        network_cache: NetworkCache,
    },
    None,
}

impl Default for SocketCommMode {
    fn default() -> Self {
        SocketCommMode::None
    }
}

pub struct ApiController {
    communication_mode: SocketCommMode,
    websocket: Handle<WebSocketClient>,
}

impl ApiController {
    pub async fn new() -> Self {
        let ws_client = WebSocketClient::new().await;
        let websocket = scene::add_node(ws_client);
        Self { communication_mode: SocketCommMode::None, websocket }
    }
}

impl scene::Node for ApiController {
    fn update(mut node: RefMut<Self>)
    where
        Self: Sized,
    {
        let mut comm = mem::take(&mut node.communication_mode);
        match &mut comm {
            SocketCommMode::Lobby {
                username,
                remote_players,
            } => {

            },
            SocketCommMode::RealGame {
                remote_players,
                network_cache,
            } => {
                let mut socket = scene::get_node(node.websocket);
                // Step 1: TX new state
                {
                    let network_frame =
                        get_time() - network_cache.last_send_time > (1. / NETWORK_FPS) as f64;

                    match scene::find_node_by_type::<Player>() {
                        Some(player) if network_frame => {
                            network_cache.last_send_time = get_time();

                            let mut state = PlayerStateBits([0; 4]);
                            state.set_x(player.pos().x as u32);
                            state.set_y(player.pos().y as u32);
                            state.set_dead(false); // TODO nobody can die yet

                            if network_cache.sent_position != state.0 {
                                network_cache.sent_position = state.0;
                                socket.send_player_state_bits(state);
                            }
                        }
                        _ => {}
                    }
                }
                // Step 2: RX receive state
                while let Some(msg) = socket.listen::<message::MessagesRx>() {
                    match msg {
                        message::MessagesRx::PlayerState { client, player_id } => {
                            remote_players.get_mut(&player_id).and_then(|h| {
                                let mut other = scene::get_node(*h);
                                let state = PlayerStateBits(client.0);
                                other.set_pos(vec2(state.x() as f32, state.y() as f32));
                                other.set_dead(state.dead());
                                Some(h)
                            });
                        }
                        message::MessagesRx::JoinLobby {
                            username,
                            player_id,
                        } => {
                            let player = RemotePlayer::new(username.0, player_id);
                            let remote_player = scene::add_node(player);
                            remote_players.insert(player_id, remote_player);
                        }
                        message::MessagesRx::Disconnect { player_id } => todo!(),
                    }
                }
            }
            SocketCommMode::None => todo!(),
        }
        node.communication_mode = comm;
    }
}
