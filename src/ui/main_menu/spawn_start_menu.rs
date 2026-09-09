use crate::ui::initial_loading::localization_loader_plugin::public_api::Localization;
use bevy::app::AppExit;

use crate::ui::colors::{COLOR_ACCENT_GOLD, COLOR_DANGER_BG, COLOR_PANEL_BG};
use crate::ui::initial_loading::fonts_loader_plugin::fonts::Fonts;
use crate::ui::initial_loading::player_settings_loader_plugin::settings_config::SettingsConfig;
use crate::ui::menus::AppState;
use bevy::color::Color;
use bevy::prelude::{
    BorderColor, BorderRadius, Bundle, Button, ChildOf, Click, Commands, Component, DespawnOnExit,
    Entity, Font, FontSize, Handle, MessageWriter, NextState, On, Pointer, Query, Res, ResMut,
    Text, TextColor, TextFont, UiRect,
};
use bevy::text::FontSource;
use bevy::ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val};
use bevy::utils::default;
use bevy_persistent::Persistent;

#[derive(Component)]
pub(super) enum MainMenuButtons {
    StartButton,
    ResumeButton,
    MultiplayerButton,
    SettingsButton,
    ExitButton,
}

impl MainMenuButtons {
    const ALL_ENUMS: [MainMenuButtons; 5] = [
        Self::StartButton,
        Self::ResumeButton,
        Self::MultiplayerButton,
        Self::SettingsButton,
        Self::ExitButton,
    ];

    const ALL_KEYS: [&str; 5] = [
        "new_game",
        "resume_game",
        "multiplayer",
        "settings_menu",
        "exit_game",
    ];
}

pub(super) fn spawn_main_menu(
    mut commands: Commands,
    localization: Res<Localization>,
    fonts: Res<Fonts>,
    settings: Res<Persistent<SettingsConfig>>,
) {
    // делаем панель коричневого цвета на весь экран
    let panel = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            BackgroundColor(COLOR_PANEL_BG),
            DespawnOnExit(AppState::MainMenu),
        ))
        .id();

    // проходимся по кнопкам
    for (button_type, key) in MainMenuButtons::ALL_ENUMS
        .into_iter()
        .zip(MainMenuButtons::ALL_KEYS.into_iter())
    {
        // создаем контейнер для кнопок
        let container = commands
            .spawn((spawn_main_button_bundle(button_type), ChildOf(panel)))
            .id();

        // проверяем были ли кнопки нажаты через observers поэтому работает в setup
        check_button_clicked(&mut commands, container);

        // создаем текст в контейнере
        commands.spawn((
            spawn_text_bundle(
                localization.get(key, settings.language),
                fonts.get(settings.language),
            ),
            ChildOf(container),
        ));
    }
}

//---------------------------------------------------------------------------------------------------------------------------//

fn spawn_main_button_bundle(button_type: MainMenuButtons) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Percent(35.0),
            height: Val::Percent(8.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        },
        BorderColor::all(COLOR_ACCENT_GOLD),
        BackgroundColor(COLOR_DANGER_BG),
        button_type,
    )
}

fn spawn_text_bundle(translated_text: &str, font: &Handle<Font>) -> impl Bundle {
    (
        Text::new(translated_text),
        TextFont {
            font: FontSource::Handle(font.clone()),
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::WHITE),
    )
}

fn check_button_clicked(commands: &mut Commands, container: Entity) {
    commands.entity(container).observe(
        |trigger: On<Pointer<Click>>,
         mut next_state: ResMut<NextState<AppState>>,
         button_type: Query<&MainMenuButtons>,
         mut exit: MessageWriter<AppExit>| {
            if let Ok(button_type) = button_type.get(trigger.entity) {
                match button_type {
                    MainMenuButtons::ExitButton => {
                        exit.write(AppExit::Success);
                    }
                    MainMenuButtons::MultiplayerButton => {
                        next_state.set(AppState::MultiplayerSetupMenu);
                    }
                    MainMenuButtons::SettingsButton => next_state.set(AppState::SettingsMenu),
                    MainMenuButtons::StartButton => {
                        next_state.set(AppState::GameSetupMenu);
                    }
                    MainMenuButtons::ResumeButton => {
                        next_state.set(AppState::InGameRenderer);
                    }
                }
            }
        },
    );
}
