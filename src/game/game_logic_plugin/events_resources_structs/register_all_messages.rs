use crate::game::game_logic_plugin::events_resources_structs::{
    action_messages::{
        AttackEntityCommand, BuildBuildingCommand, MoveEntityCommand, PickResourceCommand,
        SpawnUnitCommand,
    },
    render_messages::{
        RenderBuilding, RenderFog, RenderResource, RenderTerrain, RenderUnit, TileClickedEvent,
    },
};
use bevy::prelude::App;

pub trait AppMessageExt {
    fn register_all_messages(&mut self) -> &mut Self;
}

impl AppMessageExt for App {
    fn register_all_messages(&mut self) -> &mut Self {
        self.add_message::<TileClickedEvent>()
            // render messages
            .add_message::<RenderTerrain>()
            .add_message::<RenderResource>()
            .add_message::<RenderBuilding>()
            .add_message::<RenderUnit>()
            .add_message::<RenderFog>()
            // action messages
            .add_message::<MoveEntityCommand>()
            .add_message::<AttackEntityCommand>()
            .add_message::<PickResourceCommand>()
            .add_message::<BuildBuildingCommand>()
            .add_message::<SpawnUnitCommand>()
    }
}
