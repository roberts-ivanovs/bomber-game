use self::message::tx;

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
    use uuid::{Bytes, Uuid};

    pub type PlayerID = Bytes;

    pub mod tx {
        use super::{PlayerState, Username};
        use nanoserde::{DeBin, SerBin};
        use nserde_into_vec::IntoVecU8;
        use uuid::Bytes;

        #[derive(Debug, Clone, SerBin, DeBin, PartialEq, IntoVecU8)]
        pub enum TxOptions {
            MessagesMainMenu(MessagesMainMenu),
            MessagesLobby(MessagesLobby),
            MessagesGame(MessagesGame),
        }

        #[derive(Debug, Clone, SerBin, DeBin, PartialEq, IntoVecU8)]
        pub enum MessagesMainMenu {
            JoinLobby { username: Username, lobby_id: Bytes },
            CreateLobby,
        }

        #[derive(Debug, Clone, SerBin, DeBin, PartialEq, IntoVecU8)]
        pub enum MessagesLobby {
            Disconnect,
            StartGame,
        }

        #[derive(Debug, Clone, SerBin, DeBin, PartialEq, IntoVecU8)]
        pub enum MessagesGame {
            PlayerState(PlayerState),
        }
    }

    pub mod rx {
        use super::{PlayerID, PlayerState, Username};
        use nanoserde::{DeBin, SerBin};
        use nserde_into_vec::IntoVecU8;
        use uuid::Bytes;
        #[derive(Debug, Clone, SerBin, DeBin, PartialEq, IntoVecU8)]
        pub enum RxOptions {
            MessagesMainMenu(MessagesMainMenu),
            MessagesLobby(MessagesLobby),
            MessagesGame(MessagesGame),
        }

        #[derive(Debug, Clone, SerBin, DeBin, PartialEq, IntoVecU8)]
        pub enum MessagesMainMenu {
            NewLobbyId { lobby_id: Bytes },
            SuccessfulJoin,
        }

        #[derive(Debug, Clone, SerBin, DeBin, PartialEq, IntoVecU8)]
        pub enum MessagesLobby {
            SomeElseJoinedLobby {
                username: Username,
                player_id: PlayerID,
            },
            Disconnect {
                player_id: PlayerID,
            },
        }

        #[derive(Debug, Clone, SerBin, DeBin, PartialEq, IntoVecU8)]
        pub enum MessagesGame {
            // Sent by other players
            PlayerState {
                client: PlayerState,
                player_id: PlayerID,
            },
        }
    }
    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub struct PlayerState(pub [u8; 4]);
    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub struct Username(pub String);
}
