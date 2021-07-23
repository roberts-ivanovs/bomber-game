
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
    use uuid::{Bytes, Uuid};
    use std::fmt::Debug;

    pub type PlayerID = Bytes;

    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub enum MessagesTx {
        PlayerState(PlayerState),
        JoinLobby { username: Username, lobby_id: Bytes },
        CreateLobby,
        Disconnect,
    }


    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub enum MessagesRx {
        // Sent by other players
        PlayerState {
            client: PlayerState,
            player_id: PlayerID,
        },
        SomeElseJoinedLobby {
            username: Username,
            player_id: PlayerID,
        },
        Disconnect { player_id: PlayerID },

        // Sent by the server
        NewLobbyId { lobby_id: Bytes },
        Success,

        // Nothing to do
        Noop,
    }

    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub struct PlayerState(pub [u8; 4]);
    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub struct Username(pub String);

    impl Into<Vec<u8>> for MessagesRx {
        fn into(self) -> Vec<u8> {
            self.serialize_bin()
        }
    }
}

pub fn append_user_id(
    player_id: message::PlayerID,
    client_message: message::MessagesTx,
) -> impl Into<Vec<u8>> {
    match client_message {
        message::MessagesTx::PlayerState(client) => {
            message::MessagesRx::PlayerState {
                client,
                player_id,
            }
        },
        message::MessagesTx::Disconnect => {
            message::MessagesRx::Disconnect { player_id }
        },
        message::MessagesTx::CreateLobby => message::MessagesRx::Noop,
        message::MessagesTx::JoinLobby { username: _, lobby_id: _ } => message::MessagesRx::Noop,
    }
}
