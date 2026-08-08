use bevy::prelude::*;
use crate::game_logic::selection::events_resources_structs::components::{TileContent, TileLayer};

#[derive(Default, Message)]
pub struct TileClickedEvent {
    pub grid_pos: IVec2,
}

#[derive(Message)]
pub struct NewTileSelectionEvent {
    pub tile: TileContent,
}

#[derive(Message)]
pub struct CycleSelectionEvent {
    pub tile: TileContent,
}

#[derive(Message)]
pub struct ActionSelectionEvent {
    pub selected_pos: IVec2,
    pub target_tile: TileContent,
    pub selected_layer: TileLayer,
}

#[derive(Message)]
pub struct ClearSelectionEvent;

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

//---------------------------------------------------------------------------------------------------------------------------//
//TODO сделать trait AppMessageExt глобальным и добавлять все message в начале

pub trait AppMessageExt {
    fn register_interactive_messages(&mut self) -> &mut Self;
}

impl AppMessageExt for App {
    fn register_interactive_messages(&mut self) -> &mut Self {
        self.add_message::<TileClickedEvent>()
//---------------------------------------------------------------------------------------------------------------------------//
            .add_message::<NewTileSelectionEvent>()
            .add_message::<CycleSelectionEvent>()
            .add_message::<ActionSelectionEvent>()
            .add_message::<ClearSelectionEvent>()
//---------------------------------------------------------------------------------------------------------------------------//
            .add_message::<RenderTerrain>()
            .add_message::<RenderResource>()
            .add_message::<RenderBuilding>()
            .add_message::<RenderUnit>()
            .add_message::<RenderFog>()
    }
}