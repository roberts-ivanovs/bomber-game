use bytes::Bytes;
use nanoserde::{DeBin, SerBin};

const NET_PROTOCOL_VERSION: u8 = 0x01;


bitfield::bitfield! {
    pub struct PlayerStateBits([u8]);
    impl Debug;
    u32;
    pub x, set_x: 9, 0;
    pub y, set_y: 19, 10;
    pub dead, set_dead: 20;
}
/// ^ 21-31 leftover bits


pub mod message {
    use nanoserde::{DeBin, SerBin};

    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub struct PlayerState(pub [u8; 4]);
    impl PlayerState {
        pub const OPCODE: u32 = 0;
    }


    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub struct SpawnItem {
        pub id: u32,
        pub x: u16,
        pub y: u16,
        pub item_type: u8,
    }
    impl SpawnItem {
        pub const OPCODE: u32 = 1;
    }

    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub struct DeleteItem {
        pub id: u32,
    }
    impl DeleteItem {
        pub const OPCODE: u32 = 2;
    }
    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub struct Ready;
    impl Ready {
        pub const OPCODE: u32 = 3;
    }

    #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
    pub struct StartGame;
    impl StartGame {
        pub const OPCODE: u32 = 4;
    }

}
