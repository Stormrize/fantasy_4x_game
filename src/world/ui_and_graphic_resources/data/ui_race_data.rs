use bevy::asset::Handle;
use bevy::image::Image;
use smol_str::SmolStr;

#[derive(Debug, Clone)]
pub struct UiRaceData {
    pub id: SmolStr,
    pub description: String,
    pub ui_icon: Handle<Image>,
    pub visual_path: String,
    pub unit_data_path: String,
    pub building_data_path: String,
}
