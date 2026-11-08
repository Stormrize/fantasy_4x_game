use crate::game::game_logic_plugin::events_resources_structs::render_action_messages::RenderMoveEntity;
use crate::game::renderer::units_renderer::UnitVisual;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use bevy::math::Vec2;
use bevy::prelude::{MessageReader, Query, Res, Transform};
use bevy::time::Time;

#[derive(Component, Default)]
pub struct SmoothMove {
    pub from: Vec2,
    pub to: Vec2,
    pub speed: f32,
}

pub fn render_move(
    mut move_entity_event: MessageReader<RenderMoveEntity>,
    mut commands: Commands,
    logical_units_q: Query<&UnitVisual>,
    mut visual_units_q: Query<(&mut Transform, Option<&mut SmoothMove>)>,
) {
    for action in move_entity_event.read() {
        let Ok(unit_visual) = logical_units_q.get(action.entity) else {
            continue;
        };
        let Ok((transform, smooth_opt)) = visual_units_q.get_mut(unit_visual.0) else {
            continue;
        };

        let from = Vec2::new(transform.translation.x, transform.translation.z);
        let to = Vec2::new(action.new_pos.x as f32, action.new_pos.y as f32);

        match smooth_opt {
            Some(mut smooth) => {
                // if move - update goal, start from current position
                smooth.from = from;
                smooth.to = to;
            }
            None => {
                commands.entity(unit_visual.0).insert(SmoothMove {
                    from,
                    to,
                    speed: 10.0,
                });
            }
        }
    }
}

pub fn animate_smooth_move(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut SmoothMove)>,
) {
    let dt = time.delta_secs();

    for (entity, mut transform, smooth) in &mut q {
        let current = Vec2::new(transform.translation.x, transform.translation.z);
        let delta = smooth.to - current;
        let dist = delta.length();

        let step = smooth.speed * dt;

        if dist <= step || dist < f32::EPSILON {
            transform.translation.x = smooth.to.x;
            transform.translation.z = smooth.to.y;
            commands.entity(entity).remove::<SmoothMove>();
        } else {
            let dir = delta / dist;
            let new_pos = current + dir * step;
            transform.translation.x = new_pos.x;
            transform.translation.z = new_pos.y;
        }
    }
}

pub fn attack_move() {}
