use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy_persistent::Persistent;

use super::tabs::SettingsTab;
use crate::io::startup_loading::localization_loader_plugin::{Language, Localization};
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::ui::TranslationKey;

/// Вкладки, изменённые с последнего "Применить", но ещё не разосланные.
#[derive(Resource, Default)]
pub struct PendingSettingsChanges(pub HashSet<SettingsTab>);

/// Летит при нажатии "Применить" со списком реально изменившихся вкладок.
#[derive(Event)]
pub struct SettingsApplied(pub HashSet<SettingsTab>);

pub fn on_settings_applied(
    trigger: On<SettingsApplied>,
    localization: Res<Localization>,
    settings: Res<Persistent<SettingsConfig>>,
    mut texts: Query<(&TranslationKey, &mut Text)>,
) {
    for kind in &trigger.0 {
        match kind {
            SettingsTab::Localization => {
                change_language(&localization, settings.language, &mut texts)
            }
            SettingsTab::Sound => apply_audio_settings(&settings),
            SettingsTab::Graphics => apply_graphics_settings(&settings),
            SettingsTab::World
            | SettingsTab::Gameplay
            | SettingsTab::Camera
            | SettingsTab::Interface => {}
        }
    }
}

/// Перечитывает язык и обновляет текст всех UI-элементов с `TranslationKey`.
/// Если у вас уже есть такая система в `update_ui_system` — лучше вызвать
/// её отсюда вместо дублирования этого цикла.
fn change_language(
    localization: &Localization,
    language: Language,
    texts: &mut Query<(&TranslationKey, &mut Text)>,
) {
    for (key, mut text) in texts.iter_mut() {
        *text = Text::new(localization.get(key.0, language));
    }
}

/// TODO: применить громкости к реальным аудио-каналам/ресурсу звука.
fn apply_audio_settings(_settings: &Persistent<SettingsConfig>) {}

/// TODO: применить fullscreen/resolution/vsync/ui_scale/graphics_quality
/// (скорее всего через `ResMut<Window>` / пересоздание рендер-настроек).
fn apply_graphics_settings(_settings: &Persistent<SettingsConfig>) {}
