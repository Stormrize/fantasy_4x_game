use bevy::app::{App, Plugin};

pub mod in_game_loading;
pub mod startup_loading;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(startup_loading::InitialLoader)
            .add_plugins(in_game_loading::InGameAssetLoader);
    }
}
