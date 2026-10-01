mod camera_plugin;
mod game_logic_plugin;
mod renderer;

use bevy::app::App;
use bevy::prelude::Plugin;

use crate::game::{
    camera_plugin::camera::CameraPlugin, game_logic_plugin::GameLogic, renderer::MapRenderPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapRenderPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(GameLogic);
    }
}
