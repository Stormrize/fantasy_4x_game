use bevy::prelude::MessageReader;
use crate::game::game_logic_plugin::selection::events_resources_structs::messages::ActionSelectionEvent;

pub fn route_action(mut action_events: MessageReader<ActionSelectionEvent>) {
    for event in action_events.read() {}
}
