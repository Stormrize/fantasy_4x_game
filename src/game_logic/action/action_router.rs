use bevy::prelude::{CurveExt, MessageReader};
use crate::game_logic::selection::events_resources_structs::messages::ActionSelectionEvent;

pub fn route_action(
    mut action_events: MessageReader<ActionSelectionEvent>,
) {
    for event in action_events.read() {
        
    }
}