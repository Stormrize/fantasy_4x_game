//! Меню настроек: сайдбар с вкладками + контент текущей вкладки + футер с
//! кнопками. Виджеты сразу пишут значения в `Persistent<SettingsConfig>`, а
//! более "тяжёлые" реакции на изменения (перевод интерфейса и т.п.)
//! откладываются до нажатия "Применить" — см. `change_tracking.rs`.

mod change_tracking;
mod layout;
mod tab_content;
mod tabs;
mod theme;
mod widgets;

use bevy::prelude::*;

use crate::ui::menus::AppState;

pub use change_tracking::PendingSettingsChanges;

pub(super) struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PendingSettingsChanges>();
        app.add_observer(change_tracking::on_settings_applied);
        app.add_systems(OnEnter(AppState::SettingsMenu), layout::spawn_settings_menu);
    }
}
