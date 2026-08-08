use bevy::image::Image;
use bevy::prelude::{Handle, Resource, Res};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct RaceData {
    pub id: String,
    pub description: String,
    #[asset(key = "ui_icon")]
    pub ui_icon_path: Handle<Image>,
    pub visual_path: String,
}