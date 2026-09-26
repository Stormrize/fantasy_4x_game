use bevy::asset::LoadState;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

// localization
#[derive(Resource, Debug, Clone)]
pub struct Localization {
    pub languages: HashMap<Language, HashMap<String, String>>,
}

impl Localization {
    pub fn get<'a>(&'a self, key: &'a str, language: Language) -> &'a str {
        self.languages
            .get(&language)
            .and_then(|map| map.get(key))
            .map(String::as_str)
            .unwrap_or(key)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Language {
    #[default]
    En,
    Ru,
    De,
}

impl Language {
    pub(super) fn localization_path(self) -> &'static str {
        match self {
            Language::En => "locales/en/main.ron",
            Language::Ru => "locales/ru/main.ron",
            Language::De => "locales/de/main.ron",
        }
    }
}

// raw data
#[derive(Asset, TypePath, Debug, Clone, Deserialize)]
struct RawLocalization(pub HashMap<String, String>);

#[derive(Resource, Default)]
struct LocalizationHandles(HashMap<Language, Handle<RawLocalization>>);

// loading systems
fn start_loading_localization(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut handles = HashMap::new();
    for lang in [Language::En, Language::Ru, Language::De] {
        let handle: Handle<RawLocalization> = asset_server.load(lang.localization_path());
        handles.insert(lang, handle);
    }
    commands.insert_resource(LocalizationHandles(handles));
}

fn all_localizations_loaded(asset_server: &AssetServer, handles: &LocalizationHandles) -> bool {
    if handles.0.is_empty() {
        return false;
    }
    handles
        .0
        .values()
        .all(|handle| match asset_server.load_state(handle.id()) {
            LoadState::Loaded => true,
            LoadState::Failed(err) => {
                eprintln!(
                    "Failed to load localization {}. Error: {}",
                    handle.id(),
                    err
                );
                false
            }
            _ => false,
        })
}

fn localization_loading_ready(
    asset_server: Res<AssetServer>,
    localization_handles: Res<LocalizationHandles>,
    raw_assets: Res<Assets<RawLocalization>>,
    mut next_state: ResMut<NextState<LocalizationState>>,
    mut commands: Commands,
) {
    if all_localizations_loaded(&asset_server, &localization_handles) {
        let mut languages = HashMap::new();

        for (lang, handle) in localization_handles.0.iter() {
            let Some(raw) = raw_assets.get(handle) else {
                return;
            };
            languages.insert(*lang, raw.0.clone());
        }

        commands.insert_resource(Localization { languages });
        commands.remove_resource::<LocalizationHandles>();
        next_state.set(LocalizationState::Ready);
        println!("localization loaded");
    }
}

// states
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LocalizationState {
    #[default]
    Disabled,
    Loading,
    Ready,
}

// plugin
pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<RawLocalization>::new(&["ron"]))
            .init_state::<LocalizationState>()
            .add_systems(
                OnEnter(LocalizationState::Loading),
                start_loading_localization,
            )
            .add_systems(
                Update,
                localization_loading_ready.run_if(in_state(LocalizationState::Loading)),
            );
    }
}
