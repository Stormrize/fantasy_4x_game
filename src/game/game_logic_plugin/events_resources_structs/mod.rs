use crate::game::game_logic_plugin::events_resources_structs::action_messages::{
    AttackEntityCommand, BuildBuildingCommand, MoveEntityCommand, PickResourceCommand,
    SpawnUnitCommand, TileClickedEvent,
};
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::{
    RenderAttackEntity, RenderMoveEntity,
};
use crate::game::game_logic_plugin::events_resources_structs::render_selection_messages::{
    RenderBuilding, RenderFog, RenderResource, RenderTerrain, RenderUnit,
};
use bevy::app::App;

pub mod action_messages;
pub mod components;
pub mod render_action_messages;
pub mod render_selection_messages;

pub trait AppMessageExt {
    fn register_all_messages(&mut self) -> &mut Self;
}

impl AppMessageExt for App {
    fn register_all_messages(&mut self) -> &mut Self {
        self.add_message::<TileClickedEvent>()
            // render selection messages
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
            // render action messages
            .add_message::<RenderMoveEntity>()
            .add_message::<RenderAttackEntity>()
    }
}
