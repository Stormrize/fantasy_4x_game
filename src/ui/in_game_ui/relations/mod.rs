//! Relations overlay: one card per player with its race, stockpile, mana and
//! a quick army / city count.
//!
//! | file         | what lives there                                      |
//! |--------------|-------------------------------------------------------|
//! | `panel.rs`   | the overlay shell: header, close button, scroll body   |
//! | `rebuild.rs` | the system that refills the body when data changes     |
//! | `card.rs`    | one player card (race, resources, counts, mana)        |
//! | `census.rs`  | units / buildings per owner                            |

mod card;
mod census;
mod panel;
mod rebuild;

pub(crate) use panel::spawn_relations_panel;
pub(crate) use rebuild::rebuild_relations;
