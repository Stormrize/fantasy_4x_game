use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    GameSetupMenu,
    MultiplayerSetupMenu,
    InGameMenu,
    SettingsMenu,
    AboutUsMenu,
}