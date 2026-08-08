//mod input;
mod network;
mod ui;
mod utils_debug;
mod world;


use::bevy::prelude::*;
use serde::Deserialize;
//use crate::camera::camera::CameraPlugin;
//use crate::game_logic::GameLogic;
//use crate::game_logic::selection::SelectionPlugin;
//use crate::generation::insert_map_resources;
//use crate::game_logic::GameLogic;
//use crate::renderer::MapPlugin;
use crate::ui::menus::AppState;
use crate::ui::UiPlugin;

fn main() {
    println!("Start");
    App::new()
        //.add_systems(OnEnter(AppState::InGameMenu), insert_map_resources)
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin)
        //.add_plugins(MapPlugin)
        //.add_plugins(CameraPlugin)
        //.add_plugins(GameLogic)
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(LogDiagnosticsPlugin::default())
        .run();
    println!("End");
}