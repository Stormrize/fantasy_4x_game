//! Тёмно-фэнтезийная палитра и мелкие стили-хелперы для settings-меню.
//! Если нужно поменять внешний вид — правьте только этот файл, остальной
//! код о цветах ничего не знает.

use bevy::prelude::Color;

// ---------- Палитра ----------
pub const COLOR_PANEL_BG: Color = Color::srgb(0.094, 0.078, 0.070);
pub const COLOR_SIDEBAR_BG: Color = Color::srgb(0.071, 0.059, 0.051);
pub const COLOR_ACCENT_GOLD: Color = Color::srgb(0.769, 0.604, 0.290);
pub const COLOR_ACCENT_CRIMSON: Color = Color::srgb(0.478, 0.122, 0.122);
pub const COLOR_TAB_BG: Color = Color::srgb(0.114, 0.098, 0.086);
pub const COLOR_TAB_ACTIVE_BG: Color = Color::srgb(0.227, 0.094, 0.094);
pub const COLOR_TRACK_BG: Color = Color::srgb(0.137, 0.118, 0.106);
pub const COLOR_TEXT_PRIMARY: Color = Color::srgb(0.902, 0.851, 0.741);
pub const COLOR_TEXT_MUTED: Color = Color::srgb(0.588, 0.541, 0.471);
pub const COLOR_BUTTON_BG: Color = Color::srgb(0.157, 0.129, 0.110);
pub const COLOR_DANGER_BG: Color = Color::srgb(0.302, 0.098, 0.098);
pub const COLOR_APPLY_BG: Color = Color::srgb(0.102, 0.200, 0.188);
