use std::process;

use macroquad::{
    color::BLACK,
    experimental::collections::storage,
    math::vec2,
    ui::{root_ui, widgets},
    window::{clear_background, next_frame, screen_width},
};
use macroquad::{prelude::*, ui};

use super::{GuiResources, LobbyType, Scene};

const BUTTON_WIDTH: f32 = 250.0;
const MAIN_MENU_BUTTON_COUNT: u32 = 16;
const BUTTON_HEIGHT: f32 = 60.0;
const BUTTON_Y_GAP: f32 = 15.0;

fn draw_core_button(title: &str, offset: u32) -> widgets::Button {
    // let label_size = root_ui().calc_size(title);
    let label_pos = vec2(
        screen_width() / MAIN_MENU_BUTTON_COUNT as f32,
        (screen_height() / MAIN_MENU_BUTTON_COUNT as f32)
            + ((BUTTON_HEIGHT + BUTTON_Y_GAP) * offset as f32)
            + 200., // Extra padding from the top
    );
    widgets::Button::new(title)
        .size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
        .position(vec2(label_pos.x as f32, label_pos.y as f32))
}

pub async fn main_menu() -> Scene {
    loop {
        clear_background(BLACK);

        let resources = storage::get::<GuiResources>();
        root_ui().push_skin(&resources.title_skin);

        let title = "ETHER BOMBER";
        let label_size = root_ui().calc_size(title);
        let label_pos = vec2(screen_width() / 2. - label_size.x / 2., 100.);
        root_ui().label(Some(label_pos), title);

        let title = "QUICK GAME";
        if draw_core_button(title, 0).ui(&mut *root_ui()) {
            root_ui().pop_skin();
            return Scene::Game;
        }

        let title = "CREATE A LOBBY";
        if draw_core_button(title, 1).ui(&mut *root_ui()) {
            root_ui().pop_skin();
            return Scene::Lobby(LobbyType::Owner);
        }

        let title = "JOIN A LOBBY";
        if draw_core_button(title, 2).ui(&mut *root_ui()) {
            root_ui().pop_skin();
            return Scene::Lobby(LobbyType::Guest);
        }

        let title = "CREDITS";
        if draw_core_button(title, 3).ui(&mut *root_ui()) {
            root_ui().pop_skin();
            return Scene::Credits;
        }

        root_ui().pop_skin();

        next_frame().await;
    }
}
