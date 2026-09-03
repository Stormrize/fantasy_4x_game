use crate::ui::initial_loading::localization_loader_plugin::LocalizationState;
use crate::ui::initial_loading::localization_loader_plugin::public_api::{Language, Localization};
use bevy::asset::LoadState;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::Deserialize;

// raw data
#[derive(Asset, TypePath, Debug, Clone, Deserialize)]
pub(super) struct RawLocalization(pub HashMap<String, String>);

#[derive(Resource, Default)]
pub(super) struct LocalizationHandles(HashMap<Language, Handle<RawLocalization>>);

// loading systems
pub(super) fn start_loading_localization(asset_server: Res<AssetServer>, mut commands: Commands) {
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

pub(super) fn localization_loading_ready(
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
