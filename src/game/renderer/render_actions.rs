use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::RenderMoveEntity;
use crate::game::renderer::units_renderer::UnitVisual;
use bevy::prelude::{MessageReader, Query, Res, Transform};
use bevy::time::Time;

pub fn render_move(
    mut move_entity_event: MessageReader<RenderMoveEntity>,
    logical_units_q: Query<&UnitVisual>,
    mut visual_units_q: Query<&mut Transform>,
    time: Res<Time>,
) {
    for action in move_entity_event.read() {
        let Ok(entity) = logical_units_q.get(action.entity) else {
            continue;
        };
        let mut transform = visual_units_q.get_mut(entity.0).unwrap();

        transform.translation.x = action.new_pos.x as f32;
        transform.translation.z = action.new_pos.y as f32;
    }
}

pub fn attack_move() {}
