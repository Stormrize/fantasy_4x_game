use bevy::asset::Asset;
use bevy::image::Image;
use bevy::prelude::{Handle, TypePath};
use serde::Deserialize;
use smol_str::SmolStr;

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct RawRaceData {
    pub id: String,
    pub description: String,
    pub ui_icon_path: String,
    pub visual_path: String,
}

#[derive(Debug, Clone)]
pub struct UiRaceData {
    pub id: SmolStr,
    pub description: String,
    pub ui_icon: Handle<Image>,
    pub visual_path: String,
}
