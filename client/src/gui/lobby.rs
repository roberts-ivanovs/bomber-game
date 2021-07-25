use macroquad::{experimental::collections::storage, math::{vec2, Vec2}, prelude::scene, ui::{hash, root_ui}, window::{next_frame, screen_height, screen_width}};

use crate::nodes::api::ApiController;

use super::{GuiResources, LobbyType, Scene, WINDOW_HEIGHT, WINDOW_WIDTH};

pub async fn lobby(lobby_type: LobbyType) -> Scene {
    let mut api = scene::find_node_by_type::<ApiController>().unwrap();
    match lobby_type {
        LobbyType::Owner => {
            api.create_a_new_lobby()
            // 1. Ask for a new lobby
            // 2. Create a new lobby
        },
        LobbyType::Guest => {
            // 1. prompt - enter ID
            // 2. join lobby --> success: continue, not success: back to main menu ?
        },
    }


    loop {
        log::debug!("{:#?}",api.communication_mode());
        let resources = storage::get::<GuiResources>();
        root_ui().push_skin(&resources.login_skin);

        let mut next_scene = None;
        root_ui().window(
            hash!(),
            Vec2::new(
                screen_width() / 2. - WINDOW_WIDTH / 2.,
                screen_height() / 2. - WINDOW_HEIGHT / 2.,
            ),
            Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            |ui| {
                ui.separator();
                ui.label(None, "Game Lobby");
                if ui.button(vec2(560.0, 200.0), "Back") {
                    next_scene = Some(Scene::MainMenu);
                }
            },
        );

        root_ui().pop_skin();

        if let Some(next_scene) = next_scene {
            return next_scene;
        }

        next_frame().await;
    }
}
