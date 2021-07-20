// ^ 21-31 leftover bits
bitfield::bitfield! {
    pub struct PlayerStateBits([u8]);
    impl Debug;
    u32;
    pub x, set_x: 9, 0;
    pub y, set_y: 19, 10;
    pub dead, set_dead: 20;
}

pub mod message {
    use nanoserde::{DeBin, SerBin};
    use std::fmt::Debug;

    pub type PlayerID = usize;

    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub enum MessagesClientTx {
        PlayerStateClient(PlayerState),
        JoinLobbyClient { username: Username },
    }

    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub enum MessagesServerTx {
        PlayerStateServer {
            client: PlayerState,
            player_id: PlayerID,
        },
        JoinLobbyServer {
            username: Username,
            player_id: PlayerID,
        },
    }

    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub struct PlayerState(pub [u8; 4]);
    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub struct Username(pub String);

    impl Into<Vec<u8>> for MessagesServerTx {
        fn into(self) -> Vec<u8> {
            self.serialize_bin()
        }
    }
}

pub fn append_user_id(
    player_id: message::PlayerID,
    client_message: message::MessagesClientTx,
) -> message::MessagesServerTx {
    match client_message {
        message::MessagesClientTx::PlayerStateClient(client) => {
            message::MessagesServerTx::PlayerStateServer { client, player_id }
        }
        message::MessagesClientTx::JoinLobbyClient { username } => {
            message::MessagesServerTx::JoinLobbyServer {
                username,
                player_id,
            }
        }
    }
}
