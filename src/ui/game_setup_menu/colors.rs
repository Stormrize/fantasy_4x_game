//! Цвета, локальные для меню настройки игры (тёмное фэнтези).
//! Общие цвета (COLOR_PANEL_BG, COLOR_SIDEBAR_BG, COLOR_ACCENT_GOLD,
//! COLOR_TEXT_PRIMARY) берутся напрямую из `crate::ui::colors`, здесь их
//! не дублируем.

use bevy::prelude::Color;

use crate::ui::colors::COLOR_ACCENT_GOLD;

pub const COLOR_DROPDOWN_BG: Color = Color::srgb(0.145, 0.122, 0.102);
pub const COLOR_DROPDOWN_HOVER: Color = Color::srgb(0.196, 0.161, 0.125);
pub const COLOR_DROPDOWN_OPTION_BG: Color = Color::srgb(0.114, 0.094, 0.078);

pub const COLOR_ICON_BORDER_HOVER: Color = Color::srgb(0.902, 0.749, 0.412);

pub const COLOR_POPUP_BG: Color = Color::srgb(0.122, 0.102, 0.086);
pub const COLOR_POPUP_OVERLAY: Color = Color::srgba(0.0, 0.0, 0.0, 0.65);

pub const COLOR_BUTTON_START_BG: Color = COLOR_ACCENT_GOLD;
pub const COLOR_BUTTON_START_HOVER: Color = Color::srgb(0.902, 0.749, 0.412);
pub const COLOR_BUTTON_START_TEXT: Color = Color::srgb(0.094, 0.078, 0.070);

pub const COLOR_TRACK_BG: Color = Color::srgb(0.137, 0.118, 0.106);
