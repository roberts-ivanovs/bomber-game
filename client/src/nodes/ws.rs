use bomber_shared::messages::message::{self};
use bomber_shared::messages::PlayerStateBits;
use macroquad::prelude::scene;


use quad_net::quad_socket::client::QuadSocket;
use macroquad::window::next_frame;



pub struct WebSocketClient {
    socket: QuadSocket,
}

impl WebSocketClient {
    pub async fn new() -> Self {
        let socket = QuadSocket::connect("ws://127.0.0.1:9000/game").unwrap();
        #[cfg(target_arch = "wasm32")]
        {
            while socket.is_wasm_websocket_connected() == false {
                next_frame().await;
            }
        }
        Self {
            socket,
        }
    }

    pub fn send_player_state_bits(&mut self, player: PlayerStateBits<[u8; 4]>) {
        self.socket.send_bin(
            &message::tx::MessagesGame::PlayerState(
                message::PlayerState(player.0),
            ),
        );
    }

    pub fn listen<T: nanoserde::DeBin + std::fmt::Debug>(&mut self) -> Option<T> {
        self.socket.try_recv_bin()
    }

}

impl scene::Node for WebSocketClient {
}
