use crate::ui::local_game_setup_menu::components_resources::{
    GameSetupSelection, Slider, SliderHandle, SliderValueLabel,
};
use crate::ui::local_game_setup_menu::slider_systems::{
    apply_slider_value, slider_handle_left_percent, slider_label,
};
use bevy::prelude::{
    ButtonInput, Changed, Children, Interaction, MouseButton, Node, Query, Res, ResMut, Text, Val,
    With,
};
use bevy::ui::RelativeCursorPosition;

pub(super) fn start_slider_drag(
    mut sliders: Query<(&Interaction, &mut Slider), Changed<Interaction>>,
) {
    for (interaction, mut slider) in &mut sliders {
        if *interaction == Interaction::Pressed {
            slider.dragging = true;
        }
    }
}

pub(super) fn stop_slider_drag(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut sliders: Query<&mut Slider>,
) {
    if mouse_button.just_released(MouseButton::Left) {
        for mut slider in &mut sliders {
            slider.dragging = false;
        }
    }
}

pub(super) fn update_dragging_sliders(
    mut sliders: Query<(&mut Slider, &RelativeCursorPosition)>,
    mut selection: ResMut<GameSetupSelection>,
) {
    for (mut slider, relative_cursor) in &mut sliders {
        if !slider.dragging {
            continue;
        }
        let Some(normalized) = relative_cursor.normalized else {
            continue;
        };
        // normalized.x: -0.5 (левый край) .. 0.5 (правый край), центр = 0
        let x = (normalized.x + 0.5).clamp(0.0, 1.0);
        let steps = slider.steps;
        let new_index = ((x * (steps - 1) as f32).round() as usize).min(steps - 1);
        if new_index != slider.current {
            slider.current = new_index;
            apply_slider_value(slider.target, new_index, &mut selection);
        }
    }
}

pub(super) fn update_slider_handle_visuals(
    sliders: Query<(&Slider, &Children), Changed<Slider>>,
    mut handles: Query<&mut Node, With<SliderHandle>>,
) {
    for (slider, children) in &sliders {
        for &child in children {
            if let Ok(mut node) = handles.get_mut(child) {
                node.left = Val::Percent(slider_handle_left_percent(slider.current, slider.steps));
            }
        }
    }
}

pub(super) fn update_slider_labels(
    sliders: Query<&Slider, Changed<Slider>>,
    mut labels: Query<(&mut Text, &SliderValueLabel)>,
) {
    for slider in &sliders {
        for (mut text, label) in &mut labels {
            if label.0 == slider.target {
                *text = Text::new(slider_label(slider.target, slider.current));
            }
        }
    }
}
