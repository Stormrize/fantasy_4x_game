//mod input;
mod game;
mod network;
mod ui;
mod utils_debug;
mod world;

use ::bevy::prelude::*;
use serde::Deserialize;
use crate::game::GamePlugin;
//use crate::camera_plugin::camera_plugin::CameraPlugin;
//use crate::game_logic_plugin::GameLogic;
//use crate::game_logic_plugin::selection::SelectionPlugin;
//use crate::generation::insert_map_resources;
//use crate::game_logic_plugin::GameLogic;
//use crate::renderer::MapPlugin;
use crate::ui::UiPlugin;
use crate::ui::menus::AppState;

fn main() {
    println!("Start");
    App::new()
        //.add_systems(OnEnter(AppState::InGameMenu), insert_map_resources)
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin)
        .add_plugins(GamePlugin)
        //.add_plugins(GameLogic)
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(LogDiagnosticsPlugin::default())
        .run();
    println!("End");
}
