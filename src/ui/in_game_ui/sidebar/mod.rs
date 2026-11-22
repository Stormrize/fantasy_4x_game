//! Civilian sidebar: player card, world overview and collapsible information
//! sections.
//!
//! The sidebar is an information panel, not an action panel. It shows the owned
//! cities, the army, and the parts of the world the player has already
//! discovered. Selection details are merged into the lists: clicking a tile on
//! the map selects it by both position and layer, opens the matching section and
//! highlights the matching row with its details inline.
//!
//! The body is rebuilt only when the underlying game state changes.
//!
//! # Data flow of one rebuild
//!
//! ```text
//! ECS / resources ──gather──▶ UnitIndex, BuildingIndex, Discovery
//!                                   │
//!                         SidebarModel (+ selection, view, phase)
//!                                   │
//!                    sections::spawn_*  ──▶  UI entities
//! ```
//!
//! # Files
//!
//! | file              | responsibility                                         |
//! |-------------------|--------------------------------------------------------|
//! | `shell.rs`        | outer panel, player card, show/hide                    |
//! | `rebuild.rs`      | the rebuild system and selection → view sync           |
//! | `params.rs`       | `SystemParam` bundles and query aliases                |
//! | `gather.rs`       | ECS → plain view structs                               |
//! | `data.rs`         | view structs (`UnitView`, `Discovery`, ...)            |
//! | `grid.rs`         | `pos -> item` lookup and grid index math               |
//! | `model.rs`        | `SidebarModel`: everything a section needs to draw     |
//! | `sections/`       | one file per sidebar section                           |
//! | `widgets.rs`      | reusable UI pieces (headers, rows, buttons)            |
//! | `interactions.rs` | click observers attached to rows                       |
//! | `format.rs`       | localization keys, labels and colors                   |

mod data;
mod format;
mod gather;
mod grid;
mod interactions;
mod model;
mod params;
mod rebuild;
mod sections;
mod shell;
mod widgets;

pub(crate) use rebuild::rebuild_sidebar;
pub(crate) use shell::{spawn_sidebar, sync_sidebar_visibility};
