use bevy::prelude::{Entity, IVec2, Message};

#[derive(Message)]
pub struct RenderMoveEntity {
    pub entity: Entity,
    pub current_pos: IVec2,
    pub new_pos: IVec2,
}

#[derive(Message)]
pub struct RenderAttackEntity {
    pub entity: Entity,
}
