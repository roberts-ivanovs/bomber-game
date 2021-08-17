use macroquad::prelude::collections::storage;
use macroquad::prelude::*;
use macroquad_platformer::World as CollisionWorld;
use macroquad_tiled as tiled;
use rapier2d::prelude::*;

use log::Level;
use sapp_console_log::init_with_level;

mod constants;
mod gui;
mod js_interop;
mod nodes;
mod physics;

use gui::Scene;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ether Bomber".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    init_with_level(Level::Debug).unwrap();

    // load textures
    let gui_resources = gui::GuiResources::new();
    storage::store(gui_resources);

    //let mut next_scene = gui::matchmaking_lobby().await;
    let mut next_scene = Scene::MainMenu;
    loop {
        match next_scene {
            Scene::MainMenu => {
                next_scene = gui::main_menu().await;
            }
            Scene::Credits => {
                next_scene = gui::credits().await;
            }
            Scene::Game => {
                next_scene = nodes::main_game().await;
            }
            Scene::Lobby => todo!(),
        }
    }
}
