use bevy::platform::collections::HashMap;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

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

#[derive(Resource, Debug, Clone, Copy)]
pub struct CurrentLanguage(pub Language);

pub fn set_language(current_language: &mut CurrentLanguage, lang: Language) {
    current_language.0 = lang;
}

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
