//! Левая панель: иконка на каждую расу. Клик по иконке открывает попап
//! с описанием расы и кнопкой "Начать игру" (см. `popup.rs`).

use bevy::picking::events::{PointerClick, PointerOut, PointerOver};
use bevy::platform::collections::HashMap;
use bevy::prelude::{ChildOf, Commands, Entity, ImageNode, Node, On, Query, Res, UiRect, With};
use bevy::ui::{BorderColor, Val};
use bevy::utils::default;
use bevy_persistent::Persistent;

use super::colors::COLOR_ICON_BORDER_HOVER;
use super::popup::{RacePopupRoot, open_race_popup};
use crate::io::startup_loading::fonts_loader_plugin::Fonts;
use crate::io::startup_loading::localization_loader_plugin::Localization;
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::ui::colors::COLOR_ACCENT_GOLD;
use crate::world::components::PlayerRace;
use crate::world::ui_and_graphic_resources::data::ui_race_data::UiRaceData;

pub fn spawn_race_panel(
    commands: &mut Commands,
    parent: Entity,
    menu_root: Entity,
    races: &HashMap<PlayerRace, UiRaceData>,
) {
    for (race, data) in races {
        spawn_race_icon(commands, parent, menu_root, race.clone(), data.clone());
    }
}

fn spawn_race_icon(
    commands: &mut Commands,
    parent: Entity,
    menu_root: Entity,
    race: PlayerRace,
    data: UiRaceData,
) {
    let icon = commands
        .spawn((
            Node {
                width: Val::Percent(15.0),
                height: Val::Percent(15.0),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(20.0),
                    bottom: Val::Px(20.0),
                },
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            ImageNode::new(data.ui_icon.clone()),
            BorderColor::all(COLOR_ACCENT_GOLD),
            ChildOf(parent),
        ))
        .id();

    commands
        .entity(icon)
        .observe(
            move |_trigger: On<PointerClick>,
                  mut commands: Commands,
                  localization: Res<Localization>,
                  fonts: Res<Fonts>,
                  settings: Res<Persistent<SettingsConfig>>,
                  existing_popups: Query<Entity, With<RacePopupRoot>>| {
                // на экране одновременно только один попап — закрываем старый
                for popup in &existing_popups {
                    commands.entity(popup).despawn();
                }

                open_race_popup(
                    &mut commands,
                    menu_root,
                    race.clone(),
                    &data,
                    &localization,
                    &fonts,
                    settings.language,
                );
            },
        )
        .observe(
            |trigger: On<PointerOver>, mut colors: Query<&mut BorderColor>| {
                if let Ok(mut color) = colors.get_mut(trigger.entity) {
                    *color = BorderColor::all(COLOR_ICON_BORDER_HOVER);
                }
            },
        )
        .observe(
            |trigger: On<PointerOut>, mut colors: Query<&mut BorderColor>| {
                if let Ok(mut color) = colors.get_mut(trigger.entity) {
                    *color = BorderColor::all(COLOR_ACCENT_GOLD);
                }
            },
        );
}
