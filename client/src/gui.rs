use macroquad::{
    math::vec2,
    time::get_time,
    ui::{Skin, Ui},
};

mod credits;
mod mainmenu;
mod style;

pub use credits::credits;
pub use mainmenu::main_menu;
pub use style::GuiResources;

const WINDOW_WIDTH: f32 = 700.0;
const WINDOW_HEIGHT: f32 = 300.0;

pub enum Scene {
    MainMenu,
    Credits,
    Game,
}
