//mod input;
mod core_rust;
mod game;
mod network;
mod ui;
mod utils_debug;
mod world;

//use crate::game::GamePlugin;
use crate::ui::UiPlugin;
use ::bevy::prelude::*;

fn main() {
    println!("Start");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin)
        //.add_plugins(GamePlugin)
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(LogDiagnosticsPlugin::default())
        .run();
    println!("End");
}
