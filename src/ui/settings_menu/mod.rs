//! Settings-меню в тёмно-фэнтезийном стиле, построенное на headless-виджетах
//! из `bevy::ui_widgets` (Slider/Checkbox/RadioGroup/Button).
//!
//! ## Подключение
//! 1. Добавьте `mod settings_menu;` туда, где у вас лежит этот файл.
//! 2. В приложении **один раз** добавьте `UiWidgetsPlugins` (если ещё не
//!    добавлен) — это плагин из самого bevy, этот модуль его не дублирует:
//!    ```ignore
//!    app.add_plugins(bevy::ui_widgets::UiWidgetsPlugins);
//!    ```
//! 3. Добавьте `SettingsMenuPlugin` (даёт закрытие по Escape):
//!    ```ignore
//!    app.add_plugins(settings_menu::SettingsMenuPlugin);
//!    ```
//! 4. Откройте меню там, где вам нужно (клик по "Настройки" в главном меню,
//!    пауза и т.п.):
//!    ```ignore
//!    fn open_settings(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Persistent<SettingsConfig>>) {
//!        let font = asset_server.load("fonts/your_font.ttf");
//!        settings_menu::spawn_settings_menu(&mut commands, font, &settings);
//!    }
//!    ```
//!
//! ## Важно
//! - Крейт `bevy_ui_widgets` помечен как experimental — API может измениться
//!   в следующих версиях bevy, тогда потребуется мелкая правка импортов.
//! - Варианты enum'ов `Language` / `GraphicsQuality` / `GameSpeed` /
//!   `ColorblindMode` в `build.rs` — заглушки (в присланном коде не было
//!   определений самих enum'ов). Замените на настоящие варианты, иначе
//!   не скомпилируется.
//! - Путь `crate::settings::...` в `widgets.rs` / `build.rs` и путь
//!   `bevy_persistent::Persistent` — тоже поправьте под свой проект, если
//!   отличаются.

mod build;
mod theme;
mod widgets;

pub use build::spawn_settings_menu;
pub use theme::*;

use bevy::app::{App, Plugin, Startup, Update};

use crate::ui::menus::AppState;
use bevy::prelude::OnEnter;

/// Минимальный плагин: только закрытие меню по Escape.
/// Сам виджет-функционал (слайдеры/чекбоксы/радио) не требует отдельных
/// систем — вся логика навешана через `.observe(...)` прямо при спавне.
pub(super) struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SettingsMenu), spawn_settings_menu);
    }
}

// fn close_on_escape(
//     keys: Res<ButtonInput<KeyCode>>,
//     mut commands: Commands,
//     q_root: Query<Entity>,
// ) {
//     if keys.just_pressed(KeyCode::Escape) {
//         if let Ok(root) = q_root.single() {
//             commands.entity(root).despawn();
//         }
//     }
// }
