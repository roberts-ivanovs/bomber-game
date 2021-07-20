use macroquad::{
    experimental::{
        collections::storage,
        scene::{self, RefMut},
    },
    prelude::*,
};

use crate::physics::MapSize;
use crate::physics::Textures;
use resources::Resources;

pub struct LevelBg {
    // pub camera: Handle<Camera>,
}

impl LevelBg {
    pub fn new() -> LevelBg {
        LevelBg {
            // camera: Handle::null(),
        }
    }
}

fn parallax(texture: Texture2D, depth: f32, camera_pos: Vec2) -> Rect {
    let w = texture.width();
    let h = texture.height();

    let dest_rect = Rect::new(0., 0., w, h);
    let parallax_w = w as f32 * 0.1;

    let mut dest_rect2 = Rect::new(
        -parallax_w,
        -parallax_w,
        w + parallax_w * 2.,
        h + parallax_w * 2.,
    );

    let parallax_x = camera_pos.x / dest_rect.w;
    let parallax_y = camera_pos.y / dest_rect.h;

    dest_rect2.x += parallax_w * parallax_x * depth;
    dest_rect2.y += parallax_w * parallax_y * depth;

    dest_rect2
}

impl scene::Node for LevelBg {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get::<Resources>();
        let textures = resources.get_mut::<Textures>().unwrap();
        let map_size = resources.get_mut::<MapSize>().unwrap();

        textures.tiled_map.draw_tiles(
            "walls",
            Rect::new(0.0, 0.0, map_size.w as f32, map_size.h as f32),
            None,
        );
    }
}
