use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    InitialAssetLoading,
    MainMenu,
    GameSetupMenu,
    MultiplayerSetupMenu,
    InGameAssetLoading,
    InGameRenderer,
    SettingsMenu,
    AboutUsMenu,
}
