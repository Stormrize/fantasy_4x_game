use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, IVec2, Message, MessageWriter, Res, ResMut};

//movement
#[derive(Message)]
pub struct MoveEntityCommand {
    pub entity_pos: IVec2,
    pub target_pos: IVec2,
}

#[derive(Message)]
pub struct AttackEntityCommand {
    pub entity_pos: IVec2,
    pub target_pos: IVec2,
}

// render
#[derive(Message)]
pub struct PickResourceCommand {
    pub entity_pos: IVec2,
    pub target_pos: IVec2,
}

#[derive(Message)]
pub struct BuildBuildingCommand {
    pub entity: Entity,
    pub target_pos: IVec2,
}

#[derive(Message)]
pub struct SpawnUnitCommand;

#[derive(SystemParam)]
pub struct ActionCommandWriters<'w> {
    pub move_cmd: MessageWriter<'w, MoveEntityCommand>,
    pub attack_cmd: MessageWriter<'w, AttackEntityCommand>,
    pub gather_cmd: MessageWriter<'w, PickResourceCommand>,
    pub build_cmd: MessageWriter<'w, BuildBuildingCommand>,
    pub spawn_cmd: MessageWriter<'w, SpawnUnitCommand>,
}
