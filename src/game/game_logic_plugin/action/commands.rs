use bevy::app::App;
use bevy::ecs::system::command::write_message;
use bevy::prelude::{Commands, Entity, Event, IVec2, Message};
use crate::game_logic::selection::tile_clicked::TileClickedEvent;

//Already selected command
#[derive(Message)]
pub struct MoveEntityCommand {
    pub entity_pos: IVec2,
    pub target_pos: IVec2,
}

#[derive(Message)]
pub struct AttackEntityCommand {
    pub entity_pos: IVec2,
    pub target_entity_pos: IVec2,
}

#[derive(Message)]
pub struct PickResourceCommand {
    pub entity_pos: IVec2,
    pub target_resource_pos: IVec2,

}

//Just selected command
#[derive(Message)]
pub struct NewSelectedEntityCommand {
    pub entity_pos: IVec2,
}

#[derive(Message)]
pub struct BuildBuildingCommand {
    pub entity: Entity,
    pub target_building_pos: IVec2,
}

#[derive(Message)]
pub struct SpawnUnitCommand;


pub trait AppMessageExt {
    fn register_all_commands(&mut self) -> &mut Self;
}

impl AppMessageExt for App {
    fn register_all_commands(&mut self) -> &mut Self {
        self.add_message::<MoveEntityCommand>()
            .add_message::<AttackEntityCommand>()
            .add_message::<PickResourceCommand>()
            .add_message::<NewSelectedEntityCommand>()
            .add_message::<TileClickedEvent>()
    }
}