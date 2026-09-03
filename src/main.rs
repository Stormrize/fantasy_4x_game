//mod input;
mod game;
mod network;
mod ui;
mod utils_debug;
mod world;

//use crate::game::GamePlugin;
use crate::ui::UiPlugin;
use ::bevy::prelude::*;
use bevy::input_focus::{InputFocus, InputFocusVisible};
use bevy::ui_widgets::UiWidgetsPlugins;

fn main() {
    println!("Start");
    App::new()
        .add_plugins((DefaultPlugins, UiWidgetsPlugins))
        .add_plugins(UiPlugin)
        .init_resource::<InputFocus>()
        .init_resource::<InputFocusVisible>()
        //.add_plugins(GamePlugin)
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(LogDiagnosticsPlugin::default())
        .run();
    println!("End");
}

// use bevy::{
//     prelude::*,
//     // 1. ОБЯЗАТЕЛЬНО ИМПОРТИРУЕМ ПЛАГИН НАВИГАЦИИ
//     input_focus::tab_navigation::TabNavigationPlugin,
//     ui_widgets::{Slider, SliderRange, SliderThumb, SliderValue, ValueChange},
// };
// use bevy::input_focus::{InputFocus, InputFocusVisible};
// use bevy::ui_widgets::UiWidgetsPlugins;
//
// fn main() {
//     App::new()
//         // 2. ДОБАВЛЯЕМ TabNavigationPlugin К СТАНДАРТНЫМ ПЛАГИНАМ
//         .add_plugins((DefaultPlugins, TabNavigationPlugin, UiWidgetsPlugins))
//         .init_resource::<InputFocus>()
//         .init_resource::<InputFocusVisible>()
//         .add_systems(Startup, setup)
//         .run();
// }
//
// fn setup(mut commands: Commands) {
//     // UI Камера с поддержкой Picking/UiTarget (нужно, чтобы UI ловил клики мыши)
//     commands.spawn((
//         Camera2d,
//         // В Bevy 0.19 этот маркер сообщает системе рендеринга и ввода,
//         // что камера должна обрабатывать UI-элементы и Picking
//         // IsDefaultUiCamera,
//     ));
//
//     // Корневой контейнер
//     commands.spawn(Node {
//         width: Val::Percent(100.0),
//         height: Val::Percent(100.0),
//         align_items: AlignItems::Center,
//         justify_content: JustifyContent::Center,
//         ..default()
//     })
//         .with_children(|parent| {
//
//             // --- ТРЕК СЛАЙДЕРА ---
//             parent.spawn((
//                 Node {
//                     width: Val::Px(300.0),
//                     height: Val::Px(10.0),
//                     border_radius: BorderRadius::all(Val::Px(5.0)),
//                     align_items: AlignItems::Center,
//                     ..default()
//                 },
//                 BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
//                 Slider::default(),
//                 SliderValue(0.5),
//                 SliderRange::new(0.0, 1.0),
//             ))
//                 // Теперь этот триггер гарантированно сработает при перетаскивании!
//                 .observe(|trigger: On<ValueChange<f32>>| {
//                     info!("Ура, слайдер заработал! Значение: {:.2}", trigger.value);
//                 })
//                 .with_children(|slider_track| {
//
//                     // --- ПОЛЗУНОК (THUMB) ---
//                     slider_track.spawn((
//                         Node {
//                             width: Val::Px(20.0),
//                             height: Val::Px(20.0),
//                             border_radius: BorderRadius::all(Val::Px(10.0)),
//                             position_type: PositionType::Absolute,
//                             ..default()
//                         },
//                         BackgroundColor(Color::srgb(0.3, 0.7, 0.3)),
//                         SliderThumb,
//                     ));
//                 });
//         });
// }
//
