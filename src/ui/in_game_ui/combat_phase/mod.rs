//! Combat-phase HUD: the bottom command bar and the "explaining" help panel.
//!
//! The bar exposes the Frozen Synapse-inspired planning loop:
//! select one of your units, arm a skill, click a visible enemy in the sidebar,
//! then simulate the round. Orders themselves are stored in `CombatOrders`.
//!
//! | file            | what lives there                                        |
//! |-----------------|---------------------------------------------------------|
//! | `bar.rs`        | layout of the bottom bar, assembles the pieces          |
//! | `help_panel.rs` | the "how combat works" panel                            |
//! | `army_nav.rs`   | `\|<  <  >  >\|` buttons that walk through your army     |
//! | `skills.rs`     | skill buttons + armed-skill highlight                   |
//! | `status.rs`     | phase status text + its update                          |
//! | `buttons.rs`    | the small square button used by the bar                 |

mod army_nav;
mod bar;
mod buttons;
mod help_panel;
mod skills;
mod status;

pub(crate) use bar::spawn_combat_bar;
pub(crate) use skills::update_skill_highlight;
pub(crate) use status::update_combat_status;
