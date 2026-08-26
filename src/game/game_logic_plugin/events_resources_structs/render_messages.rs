use crate::game::game_logic_plugin::events_resources_structs::action_messages::{
    AttackEntityCommand, BuildBuildingCommand, MoveEntityCommand, PickResourceCommand,
    SpawnUnitCommand,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(Message, Debug)]
pub struct TileClickedEvent {
    pub pos: IVec2,
}

//---------------------------------------------------------------------------------------------------------------------------//

#[derive(Message)]
pub struct RenderTerrain {
    pub pos: IVec2,
}

#[derive(Message)]
pub struct RenderResource {
    pub pos: IVec2,
}

#[derive(Message)]
pub struct RenderBuilding {
    pub pos: IVec2,
}

#[derive(Message)]
pub struct RenderUnit {
    pub pos: IVec2,
}

#[derive(Message)]
pub struct RenderFog {
    pub pos: IVec2,
}

#[derive(SystemParam)]
pub struct RenderMessages<'w> {
    pub terrain: MessageWriter<'w, RenderTerrain>,
    pub resource: MessageWriter<'w, RenderResource>,
    pub building: MessageWriter<'w, RenderBuilding>,
    pub unit: MessageWriter<'w, RenderUnit>,
    pub fog: MessageWriter<'w, RenderFog>,
}
