use std::collections::HashMap;
use bevy::prelude::{Assets, Handle, Resource, StandardMaterial};
use serde::Deserialize;

#[derive(Resource)]
pub struct RacesData {
    pub races_data: HashMap<String, RaceData>,
}

#[derive(Resource)]
pub struct Assets_ {
    pub terrain_assets: Vec<Handle<StandardMaterial>>,
    pub resource_assets: Vec<Handle<StandardMaterial>>,
    pub buildings_assets: Vec<Handle<StandardMaterial>>,
    pub units_assets: Vec<Handle<StandardMaterial>>,
}


// build_assets


#[derive(Deserialize, Debug)]
struct RaceData {
    pub id: String,
    pub description: String,
    pub ui_icon_path: String,
}


fn load_races() -> HashMap<String, RaceData> {
    let mages = ["fire", "water"];
    let mut data: HashMap<String, RaceData> = HashMap::new();

    for i in 0..mages.len() {
        let path = "assets/core/races/".to_owned() + mages[i]+ "_mages" + "/identifier.ron";
        let race_str = std::fs::read_to_string(path).unwrap();
        let race_data: RaceData = ron::from_str(race_str.as_str()).unwrap();
        dbg!(&race_data);
        data.insert(race_data.id.clone(), race_data);
    }
    data
}