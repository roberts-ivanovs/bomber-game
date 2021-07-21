use macroquad::{
    experimental::collections::storage,
    math::{vec2, Vec2},
    ui::{hash, root_ui},
    window::{next_frame, screen_height, screen_width},
};

use super::{GuiResources, LobbyType, Scene, WINDOW_HEIGHT, WINDOW_WIDTH};

pub async fn lobby(lobby_type: LobbyType) -> Scene {

    match lobby_type {
        LobbyType::Owner => {
            // 1. Generate a new uuid
            // 2. Create a new lobby
        },
        LobbyType::Guest => {

            // 1. prompt - enter ID
            // 2. join lobby --> success: continue, not success: back to main menu ?
        },
    }


    loop {
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
