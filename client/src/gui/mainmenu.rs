use std::process;

use macroquad::{
    color::BLACK,
    experimental::collections::storage,
    math::vec2,
    ui::{root_ui, widgets},
    window::{clear_background, next_frame, screen_width},
};
use macroquad::{prelude::*, ui};

use super::{GuiResources, Scene};

pub async fn main_menu() -> Scene {
    loop {

        clear_background(BLACK);

        let resources = storage::get::<GuiResources>();
        root_ui().push_skin(&resources.title_skin);

        let title = "ETHER BOMBER";
        let label_size = root_ui().calc_size(title);
        let label_pos = vec2(screen_width() / 2. - label_size.x / 2., 100.);
        root_ui().label(Some(label_pos), title);

        let button_width = 300.0;

        if widgets::Button::new("Quick game")
            .size(vec2(button_width, 300.))
            .position(vec2(
                screen_width() / 2. - ((button_width + 10.) * 3.) / 2.,
                label_pos.y + label_size.y + 50.,
            ))
            .ui(&mut *root_ui())
        {
            root_ui().pop_skin();
            return Scene::Game;
        }

        if widgets::Button::new("    Credits")
            .size(vec2(button_width, 300.))
            .position(vec2(
                screen_width() / 2. - button_width / 2.,
                label_pos.y + label_size.y + 50.,
            ))
            .ui(&mut *root_ui())
        {
            root_ui().pop_skin();
            return Scene::Credits;
        }
        if widgets::Button::new("       Exit")
            .size(vec2(button_width, 300.))
            .position(vec2(
                screen_width() / 2. + button_width / 2. + 10.,
                label_pos.y + label_size.y + 50.,
            ))
            .ui(&mut *root_ui())
        {
            process::exit(0);
        }

        if is_key_pressed(KeyCode::Escape) {
            // Exit the game if we press ESCAPE while in mainmenu
            process::exit(0);
        }

        root_ui().pop_skin();

        next_frame().await;
    }
}
