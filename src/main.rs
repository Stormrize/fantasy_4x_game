extern crate core;

mod core_rust;
mod game;
mod io;
mod network;
mod ui;
mod utils_debug;
mod world;

use bevy::prelude::*;

use crate::game::GamePlugin;
use crate::ui::UiPlugin;

fn main() {
    println!("Start");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin)
        .add_plugins(GamePlugin)
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(LogDiagnosticsPlugin::default())
        .run();
    println!("End");
}
