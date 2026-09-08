// use std::ops::Range;
// use crate::ui::initial_loading::fonts_loader_plugin::fonts::Fonts;
// use crate::ui::initial_loading::localization_loader_plugin::public_api::{Language, Localization};
// use bevy::prelude::{default, AlignItems, BackgroundColor, BorderColor, BorderRadius, ChildOf, Color, Commands, Entity, FlexDirection, FontSize, FontSource, Node, PositionType, Text, TextColor, TextFont, UiRect, Val};
// use bevy::ui_widgets::{Slider, SliderOrientation, SliderRange, SliderStep, SliderThumb, TrackClick};
// use crate::ui::colors::{COLOR_ACCENT_CRIMSON, COLOR_ACCENT_GOLD, COLOR_TEXT_MUTED};
// use crate::ui::game_setup_menu::colors::COLOR_TRACK_BG;
//
// fn percent_of(value: f32, min: f32, max: f32) -> f32 {
//     if max > min {
//         ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
//     } else {
//         0.0
//     }
// }
//
// pub fn spawn_slider(
//     commands: &mut Commands,
//     parent: Entity,
//     localization: &Localization,
//     fonts: &Fonts,
//     language: Language,
//     bots_amount_range: (u8, u8),
//     bots_default_index: usize,
// ) {
//     let track = commands
//         .spawn((
//             Node {
//                 width: Val::Percent(60.0),
//                 align_items: AlignItems::Center,
//                 margin: UiRect::all(Val::Px(16.0)),
//                 border_radius: BorderRadius::all(Val::Px(7.0)),
//                 ..default()
//             },
//             Slider {
//                 track_click: TrackClick::Snap,
//                 orientation: SliderOrientation::Horizontal,
//             },
//             SliderRange::new(bots_amount_range.0 as f32, bots_amount_range.1 as f32),
//             SliderStep(1.0),
//             BackgroundColor(COLOR_TRACK_BG),
//             BorderColor::all(COLOR_ACCENT_GOLD),
//             ChildOf(parent),
//         ))
//         .id();
//
//     let percent0 = percent_of(bots_default_index as f32, bots_amount_range.0 as f32 , bots_amount_range.1 as f32);
//
//     let fill = commands
//         .spawn((
//             Node {
//                 position_type: PositionType::Absolute,
//                 left: Val::Percent(0.0),
//                 top: Val::Px(0.0),
//                 bottom: Val::Px(0.0),
//                 width: Val::Percent(percent0),
//                 border_radius: BorderRadius::all(Val::Px(7.0)),
//                 ..default()
//             },
//             BackgroundColor(COLOR_ACCENT_CRIMSON),
//             ChildOf(track),
//         ))
//         .id();
//
//     let thumb = commands
//         .spawn((
//             SliderThumb,
//             Node {
//                 position_type: PositionType::Absolute,
//                 left: Val::Percent(percent0),
//                 top: Val::Px(-3.0),
//                 width: Val::Auto,
//                 height: Val::Percent(100.0),
//                 aspect_ratio: Some(1.0),
//                 border_radius: BorderRadius::MAX,
//                 ..default()
//             },
//             BackgroundColor(COLOR_ACCENT_GOLD),
//             ChildOf(track),
//         ))
//         .id();
//
//
//     let value_text = commands
//         .spawn((
//             Text::new(format_value(initial, decimals, suffix)),
//             TextFont {
//                 font: FontSource::Handle(fonts.get(language).clone()),
//                 font_size: FontSize::Px(14.0),
//                 ..default()
//             },
//             TextColor(COLOR_TEXT_MUTED),
//             Node {
//                 width: Val::Px(56.0),
//                 ..default()
//             },
//             ChildOf(control),
//         ))
//         .id();
// }
