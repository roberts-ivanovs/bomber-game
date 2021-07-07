use macroquad::{
    experimental::{
        collections::storage,
        scene::{self, RefMut},
    },
    prelude::*,
};

use crate::Resources;

pub struct Destroyable {}

impl Destroyable {
    pub fn new() -> Destroyable {
        Destroyable {};
    }
}
