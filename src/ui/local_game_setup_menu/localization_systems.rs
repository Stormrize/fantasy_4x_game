use crate::ui::initial_loading::fonts_loader_plugin::fonts::Fonts;
use crate::ui::initial_loading::localization_loader_plugin::public_api::{
    CurrentLanguage, Localization,
};
use bevy::prelude::{DetectChanges, Query, Res, Text};
use bevy::text::TextFont;

use bevy::prelude::*;

// note работает но проблема с sliderами сложно перевести их изза архитектуры переделаю
// pub(super) fn update_text_based_on_language(
//     mut text_query: Query<(&mut Text, &mut TextFont, &TranslationKey)>,
//     localization: Res<Localization>,
//     fonts: Res<Fonts>,
//     current_language: Res<CurrentLanguage>,
// ) {
//     if !current_language.is_changed() {
//         return;
//     }
//
//     let lang = current_language.0;
//
//     for (mut text, mut font, translation_key) in text_query.iter_mut() {
//         text.0 = localization.get(translation_key.key, lang).to_string();
//
//         font.font = fonts.get(lang).clone();
//     }
// }
