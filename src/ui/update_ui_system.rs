use crate::ui::initial_loading::fonts_loader_plugin::fonts::Fonts;
use crate::ui::initial_loading::localization_loader_plugin::public_api::Localization;
use crate::ui::initial_loading::player_settings_loader_plugin::settings_config::SettingsConfig;
use bevy::prelude::{Component, DetectChanges, Query, Res, Text, TextFont};

#[derive(Component)]
pub(super) struct TranslationKey {
    pub key: &'static str,
}
// pub fn update_ui_system(
//     mut text_query: Query<(&mut Text, &mut TextFont, &TranslationKey)>,
//     localization: Res<Localization>,
//     fonts: Res<Fonts>,
//     settings: Res<SettingsConfig>,
// ) {
//     if !settings.is_changed() {
//         return;
//     }
//     for (mut text, mut text_font, trans) in text_query.iter_mut() {
//         *text =
//     }
// }
