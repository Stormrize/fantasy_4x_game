//! Refills the relations body, but only while the overlay is open and only
//! when something it shows has changed.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{HudUiState, RelationsBody};
use crate::world::components::Id;
use crate::world::game_configs::{GameLogicConfig, Players};
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::unit_map::UnitsMap;

use super::card::{PlayerCard, spawn_player_card};
use super::census::Census;

/// Everything the overlay reads, grouped to keep the system signature short.
#[derive(SystemParam)]
pub(crate) struct RelationsResources<'w> {
    pub ui: UiContext<'w>,
    pub settings: Res<'w, Persistent<SettingsConfig>>,
    pub players: Res<'w, Players>,
    pub config: Res<'w, GameLogicConfig>,
    pub races: Res<'w, RacesUiRegistry>,
    pub units_map: Res<'w, UnitsMap>,
    pub buildings_map: Res<'w, BuildingMap>,
}

impl RelationsResources<'_> {
    fn any_changed(&self) -> bool {
        self.players.is_changed()
            || self.units_map.is_changed()
            || self.buildings_map.is_changed()
            || self.settings.is_changed()
    }
}

pub(crate) fn rebuild_relations(
    mut commands: Commands,
    hud: Res<HudUiState>,
    resources: RelationsResources,
    body_query: Query<(Entity, &ScrollPosition), With<RelationsBody>>,
    ids: Query<&Id>,
) {
    if !hud.relations_open {
        return;
    }

    if !resources.any_changed() && !hud.is_changed() {
        return;
    }

    let Ok((body, scroll)) = body_query.single() else {
        return;
    };

    // Clear the body but keep the scroll offset.
    let scroll = scroll.0;
    commands.entity(body).despawn_children();
    commands.entity(body).insert(ScrollPosition(scroll));

    let language = resources.settings.language;
    let census = Census::new(&resources.units_map, &resources.buildings_map, &ids);

    for player in &resources.players.0 {
        let card = PlayerCard {
            player,
            is_local: player.id == resources.config.player.id,
            units: census.units_of(player.id),
            buildings: census.buildings_of(player.id),
        };
        spawn_player_card(
            &mut commands,
            body,
            &resources.ui,
            language,
            &resources.races,
            &card,
        );
    }
}
