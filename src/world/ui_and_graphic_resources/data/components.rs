use bevy::prelude::Component;

#[derive(Component)]
pub struct TranslationKey(pub &'static str);
