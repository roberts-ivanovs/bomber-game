mod credits;
mod mainmenu;
mod style;
mod lobby;

pub use credits::credits;
pub use lobby::lobby;
pub use mainmenu::main_menu;
pub use style::GuiResources;

const WINDOW_WIDTH: f32 = 700.0;
const WINDOW_HEIGHT: f32 = 300.0;


pub enum LobbyType {
    Owner,
    Guest,
}

pub enum Scene {
    MainMenu,
    Credits,
    Lobby(LobbyType),
    Game,
}
