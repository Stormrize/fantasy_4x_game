//! Burger (pause) menu: resume, relations, return to the main menu or exit the
//! game. Opened from the top bar or with `Esc`.
//!
//! | file         | what lives there                                          |
//! |--------------|-----------------------------------------------------------|
//! | `overlay.rs` | dimmed overlay, panel, title and the menu layout          |
//! | `actions.rs` | the menu buttons and what each of them does               |
//!
//! To add a menu entry, add a `spawn_*_button` in `actions.rs` and call it from
//! `spawn_menu_buttons`.

mod actions;
mod overlay;

pub(crate) use overlay::spawn_burger_overlay;
