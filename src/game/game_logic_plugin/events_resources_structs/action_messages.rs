use crate::game::game_logic_plugin::events_resources_structs::components::{
    JustSelectedTile, TileLayer,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Entity, IVec2, Message, MessageWriter};

// tile clicked
#[derive(Message, Debug)]
pub struct TileClickedEvent {
    pub pos: IVec2,
}

// selection systems
#[derive(Message, Debug, Clone)]
pub struct Cycle(pub JustSelectedTile);

#[derive(Message, Debug, Clone)]
pub struct NewTile(pub JustSelectedTile);

#[derive(Message, Debug, Clone)]
pub struct Clear;

// general
#[derive(Message)]
pub struct EndTurn;

#[derive(Message)]
pub struct Upgrade {
    pub entity_pos: IVec2,
    pub layer: TileLayer,
}

#[derive(Message)]
pub struct Delete {
    pub entity_pos: IVec2,
    pub layer: TileLayer,
}

#[derive(Message)]
pub struct Info {
    pub entity_pos: IVec2,
    pub layer: TileLayer,
}

#[derive(Message)]
pub struct WorldView;

// unit action
#[derive(Message)]
pub struct Move {
    pub entity_pos: IVec2,
    pub target_pos: IVec2,
}

#[derive(Message)]
pub struct Attack {
    pub entity_pos: IVec2,
    pub skill_type: u8,
}

#[derive(Message)]
pub struct Gather(pub IVec2);

#[derive(Message)]
pub struct Build(pub IVec2);

#[derive(Message)]
pub struct SpawnUnitCommand;

#[derive(SystemParam)]
pub struct ActionWriters<'w> {
    pub attack_cmd: MessageWriter<'w, Attack>,
    pub gather_cmd: MessageWriter<'w, Gather>,
    pub build_cmd: MessageWriter<'w, Build>,
    pub spawn_cmd: MessageWriter<'w, SpawnUnitCommand>,
    pub info: MessageWriter<'w, Info>,
    pub upgrade: MessageWriter<'w, Upgrade>,
    pub delete: MessageWriter<'w, Delete>,
    pub end_turn: MessageWriter<'w, EndTurn>,
    pub world_view: MessageWriter<'w, WorldView>,
}
