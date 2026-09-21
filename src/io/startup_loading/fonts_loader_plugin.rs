use bevy::asset::{AssetServer, Handle, LoadState};
use bevy::platform::collections::HashMap;
use bevy::prelude::{
    App, AppExtStates, Commands, Font, IntoScheduleConfigs, NextState, OnEnter, Plugin, Res,
    ResMut, Resource, States, Update, in_state,
};

use crate::io::startup_loading::localization_loader_plugin::Language;

#[derive(Resource, Default)]
pub struct Fonts(HashMap<Language, Handle<Font>>);

impl Fonts {
    pub fn get(&self, language: Language) -> &Handle<Font> {
        &self.0[&language]
    }
}

impl Language {
    fn fonts_path(self) -> &'static str {
        match self {
            Language::En => "fonts/en/Oranienbaum-Regular.ttf",
            Language::Ru => "fonts/ru/Oranienbaum-Regular.ttf",
            Language::De => "fonts/de/Oranienbaum-Regular.ttf",
        }
    }
}

// loading systems
fn start_loading_fonts(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut handles = HashMap::new();
    for lang in [Language::En, Language::Ru, Language::De] {
        let handle: Handle<Font> = asset_server.load(lang.fonts_path());
        handles.insert(lang, handle);
    }
    commands.insert_resource(Fonts(handles));
}

fn all_fonts_loaded(asset_server: &AssetServer, handles: &Fonts) -> bool {
    if handles.0.is_empty() {
        return false;
    }
    handles
        .0
        .values()
        .all(|handle| match asset_server.load_state(handle.id()) {
            LoadState::Loaded => true,
            LoadState::Failed(err) => {
                eprintln!("Failed to load font {}. Error: {}", handle.id(), err);
                false
            }
            LoadState::Loading => {
                eprintln!("Loading fonts");
                false
            }
            _ => {
                eprintln!("something else");
                false
            }
        })
}

fn fonts_loading_ready(
    asset_server: Res<AssetServer>,
    handles: Res<Fonts>,
    mut next_state: ResMut<NextState<FontsLoadingState>>,
) {
    if all_fonts_loaded(&asset_server, &handles) {
        next_state.set(FontsLoadingState::Ready);
        println!("All fonts loaded");
    }
}

// states
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontsLoadingState {
    Loading,
    #[default]
    Ready,
}

// plugin
pub struct FontsLoaderPlugin;

impl Plugin for FontsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<FontsLoadingState>()
            .add_systems(OnEnter(FontsLoadingState::Loading), start_loading_fonts)
            .add_systems(
                Update,
                fonts_loading_ready.run_if(in_state(FontsLoadingState::Loading)),
            );
    }
}
