//! Вкладки меню настроек. Тот же enum используется как "категория" для
//! отслеживания изменений — см. `change_tracking.rs`.

use crate::ui::initial_loading::localization_loader_plugin::public_api::{Language, Localization};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SettingsTab {
    World,
    Localization,
    Sound,
    Graphics,
    Gameplay,
    Camera,
    Interface,
}

impl SettingsTab {
    pub const ALL: [SettingsTab; 7] = [
        Self::World,
        Self::Localization,
        Self::Sound,
        Self::Graphics,
        Self::Gameplay,
        Self::Camera,
        Self::Interface,
    ];

    /// Ключ локализации для заголовка вкладки — он же `TranslationKey` на
    /// кнопке этой вкладки.
    pub fn key(self) -> &'static str {
        match self {
            Self::World => "world",
            Self::Localization => "localization",
            Self::Sound => "voice",
            Self::Graphics => "graphic",
            Self::Gameplay => "gameplay",
            Self::Camera => "camera",
            Self::Interface => "interface",
        }
    }
}

// небольшой хелпер на будущее, если понадобится готовая строка заголовка
// где-то ещё, кроме кнопки вкладки:
#[allow(dead_code)]
pub fn tab_title<'a>(
    tab: SettingsTab,
    localization: &'a Localization,
    language: Language,
) -> &'a str {
    localization.get(tab.key(), language)
}
