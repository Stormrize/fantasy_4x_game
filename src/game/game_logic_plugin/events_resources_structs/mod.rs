use crate::game::game_logic_plugin::events_resources_structs::action_messages::{
    Attack, Build, Clear, Cycle, Delete, EndTurn, Gather, Info, Move, NewTile, RevealMap,
    SpawnUnitCommand, TileClickedEvent, Upgrade, WorldView,
};
use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::{
    RenderAttackEntity, RenderBuildEntity, RenderGatherEntity, RenderMoveEntity,
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
            // selection
            .add_message::<Cycle>()
            .add_message::<NewTile>()
            .add_message::<Clear>()
            // render selection messages
            .add_message::<RenderTerrain>()
            .add_message::<RenderResource>()
            .add_message::<RenderBuilding>()
            .add_message::<RenderUnit>()
            .add_message::<RenderFog>()
            // action messages
            .add_message::<Move>()
            .add_message::<Attack>()
            .add_message::<Gather>()
            .add_message::<Build>()
            .add_message::<SpawnUnitCommand>()
            .add_message::<RevealMap>()
            // general
            .add_message::<WorldView>()
            .add_message::<EndTurn>()
            .add_message::<Info>()
            .add_message::<Delete>()
            .add_message::<Upgrade>()
            // render action messages
            .add_message::<RenderMoveEntity>()
            .add_message::<RenderAttackEntity>()
            .add_message::<RenderGatherEntity>()
            .add_message::<RenderBuildEntity>()
    }
}
