use std::collections::HashMap;
use crate::world::ui_resources::ui_race_data::UiRacesData;
use crate::world::ui_resources::data::race_data::RaceData;

fn load_races() -> UiRacesData {
    let mages = ["fire", "water"];
    let mut data: HashMap<String, RaceData> = HashMap::new();

    //todo
    
    for i in 0..mages.len() {
        let path = "assets/core/races/".to_owned() + mages[i]+ "_mages" + "/identifier.ron";
        let race_str = std::fs::read_to_string(path).unwrap();
        let race_data: RaceData = ron::from_str(race_str.as_str()).unwrap();
        data.insert(race_data.id.clone(), race_data);
    }
    UiRacesData { races_data: data }
}