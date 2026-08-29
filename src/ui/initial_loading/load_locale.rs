use bevy::asset::LoadState;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

#[derive(Asset, TypePath, Debug, Clone, Deserialize)]
pub struct RawLocalization(pub HashMap<String, String>);


// Languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Language {
    #[default]
    En,
    Ru,
    De,
}

impl Language {
    fn asset_path(self) -> &'static str {
        match self {
            Language::En => "locales/en/main.ron",
            Language::Ru => "locales/ru/main.ron",
            Language::De => "locales/de/main.ron",
        }
    }
}

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct CurrentLanguage(pub Language);


#[derive(Resource, Default)]
pub struct LocalizationHandles {
    pub handles: HashMap<Language, Handle<RawLocalization>>,
}


#[derive(Resource, Debug, Clone, Default)]
pub struct Localization {
    pub current: Language,
    pub map: HashMap<String, String>,
}

impl Localization {
    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.map.get(key).map(String::as_str).unwrap_or(key)
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LocalizationState {
    #[default]
    Loading,
    Ready,
}

pub fn start_loading_localization(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut handles = HashMap::new();
    for lang in [Language::En, Language::Ru, Language::De] {
        let handle: Handle<RawLocalization> = asset_server.load(lang.asset_path());
        handles.insert(lang, handle);
    }
    commands.insert_resource(LocalizationHandles { handles });
}
pub fn all_localizations_loaded(asset_server: &AssetServer, handles: &LocalizationHandles) -> bool {
    if handles.handles.is_empty() {
        return false;
    }
    handles
        .handles
        .values()
        .all(|handle| matches!(asset_server.load_state(handle.id()), LoadState::Loaded))
}

pub fn localization_ready(asset_server: Res<AssetServer>, handles: Res<LocalizationHandles>) -> bool {
    all_localizations_loaded(&asset_server, &handles)
}

fn finish_loading_localization(
    mut commands: Commands,
    handles: Res<LocalizationHandles>,
    raw_assets: Res<Assets<RawLocalization>>,
    current_lang: Res<CurrentLanguage>,
    mut next_state: ResMut<NextState<LocalizationState>>,
) {
    let Some(handle) = handles.handles.get(&current_lang.0) else {
        return;
    };
    let Some(raw) = raw_assets.get(handle) else {
        return;
    };

    commands.insert_resource(Localization {
        current: current_lang.0,
        map: raw.0.clone(),
    });
    next_state.set(LocalizationState::Ready);
}

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<RawLocalization>::new(&["ron"]))
            .init_state::<LocalizationState>()
            .init_resource::<CurrentLanguage>()
            .add_systems(OnEnter(LocalizationState::Loading), start_loading_localization)
            .add_systems(
                Update,
                finish_loading_localization
                    .run_if(in_state(LocalizationState::Loading))
                    .run_if(localization_ready),
            );
    }
}

// =====================================================================
// Пример использования в UI (когда LocalizationState::Ready):
//
// fn spawn_menu(localization: Res<Localization>, mut commands: Commands) {
//     commands.spawn(Text::new(localization.get("start_game")));
//     commands.spawn(Text::new(localization.get("resume_game")));
//     commands.spawn(Text::new(localization.get("multiplayer")));
//     commands.spawn(Text::new(localization.get("exit_game")));
//     commands.spawn(Text::new(localization.get("levels")));
//     commands.spawn(Text::new(localization.get("map_size")));
//     commands.spawn(Text::new(localization.get("map_shape")));
//     commands.spawn(Text::new(localization.get("ai_difficulty")));
//     commands.spawn(Text::new(localization.get("bots")));
// }
// =====================================================================