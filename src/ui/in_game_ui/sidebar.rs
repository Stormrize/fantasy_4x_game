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

use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy::ui_widgets::ScrollArea;
use bevy_persistent::Persistent;

use crate::app_state::AppState;
use crate::game::events::action_messages::NewTile;
use crate::game::game_logic_plugin::action::rules::movement::biome_cost;
use crate::game::game_logic_plugin::combat::integration::{CombatCycle, CombatOrders, CombatPhase};
use crate::game::game_logic_plugin::combat::model::CombatCommand;
use crate::game::game_logic_plugin::components::{
    AlreadySelectedTile, JustSelectedTile, TileContent, TileLayer,
};
use crate::io::startup_loading::localization_loader_plugin::Language;
use crate::io::startup_loading::setting_loader_saver_plugin::SettingsConfig;
use crate::io::startup_loading::ui_assets_loader_plugin::RacesUiRegistry;
use crate::ui::UiContext;
use crate::ui::in_game_ui::state::{
    CombatUiState, HudSidebar, HudUiState, SidebarBody, SidebarSection, SidebarView, UnitTab,
};
use crate::ui::in_game_ui::theme;
use crate::world::components::{Id, PlayerRace};
use crate::world::game_configs::{GameLogicConfig, PlayerData, Players};
use crate::world::game_resources::building_map::BuildingMap;
use crate::world::game_resources::data::biome::BiomeType;
use crate::world::game_resources::data::building::{BuildingLevel, BuildingType, Levels};
use crate::world::game_resources::data::resource::{CommonKind, RareKind, ResourceType};
use crate::world::game_resources::data::unit::{
    CombatBasicStats, CurrentHp, HelperBasicStats, LevelAbilities, MovementPoints,
    MovementPointsMax, UnitLevel, UnitType,
};
use crate::world::game_resources::fog_map::FogMap;
use crate::world::game_resources::resource_map::ResourceMap;
use crate::world::game_resources::terrain_map::TerrainMap;
use crate::world::game_resources::unit_map::UnitsMap;
use crate::world::turn_manager::TurnManager;
use bevy::ecs::system::SystemParam;

// Query aliases

type CombatUnitsQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Id,
        &'static PlayerRace,
        &'static UnitType,
        &'static UnitLevel,
        &'static CombatBasicStats,
        &'static CurrentHp,
        &'static MovementPoints,
        &'static MovementPointsMax,
        &'static LevelAbilities,
    ),
>;

type HelpersQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Id,
        &'static PlayerRace,
        &'static HelperBasicStats,
        &'static MovementPoints,
        &'static MovementPointsMax,
    ),
>;

type BuildingsQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Id,
        &'static BuildingType,
        &'static BuildingLevel,
        &'static Levels,
    ),
>;

// System parameters

/// All resources read by the sidebar. Grouped to stay below Bevy's system
/// parameter limit and to make the rebuild system signature readable.
#[derive(SystemParam)]
pub(crate) struct SidebarResources<'w> {
    pub ui: UiContext<'w>,
    pub settings: Res<'w, Persistent<SettingsConfig>>,
    pub config: Res<'w, GameLogicConfig>,
    pub players: Res<'w, Players>,
    pub selected: Res<'w, AlreadySelectedTile>,
    pub units_map: Res<'w, UnitsMap>,
    pub buildings_map: Res<'w, BuildingMap>,
    pub resources_map: Res<'w, ResourceMap>,
    pub terrain: Res<'w, TerrainMap>,
    pub fog: Res<'w, FogMap>,
    pub turn_manager: Res<'w, TurnManager>,
    pub cycle: Res<'w, CombatCycle>,
    pub orders: Res<'w, CombatOrders>,
    pub combat_ui: Res<'w, CombatUiState>,
    pub view: ResMut<'w, SidebarView>,
}

/// Entity queries used to build the sidebar lists.
#[derive(SystemParam)]
pub(crate) struct SidebarQueries<'w, 's> {
    pub body: Query<
        'w,
        's,
        (Entity, &'static ScrollPosition, Option<&'static Children>),
        With<SidebarBody>,
    >,
    pub combat_units: CombatUnitsQuery<'w, 's>,
    pub helpers: HelpersQuery<'w, 's>,
    pub buildings: BuildingsQuery<'w, 's>,
}

// Data gathered from the ECS for one rebuild

struct AbilityView {
    name: String,
    damage: u16,
    mana: u16,
    cooldown: u16,
    range: u8,
    area: u8,
}

struct UnitView {
    pos: IVec2,
    owner: Id,
    race: PlayerRace,
    kind: UnitType,
    level: Option<UnitLevel>,
    hp: u16,
    max_hp: u16,
    attack: u16,
    defense: u16,
    range: u16,
    moves: u16,
    moves_max: u16,
    build_points: Option<u8>,
    abilities: Vec<AbilityView>,
    tile_content: TileContent,
}

struct BuildingView {
    pos: IVec2,
    owner: Id,
    kind: BuildingType,
    level: BuildingLevel,
    gold: u16,
    food: u16,
    science: u16,
    housing: u16,
    spawn_capacity: u16,
    maintenance: u16,
}

/// Units stored in map order plus a `pos -> view` lookup that is `O(1)`.
struct UnitIndex {
    units: Vec<UnitView>,
    cells: Vec<Option<usize>>,
    size: usize,
}

impl UnitIndex {
    fn at(&self, pos: IVec2) -> Option<&UnitView> {
        if !pos_in_bounds(pos, self.size) {
            return None;
        }
        let index = pos.x as usize * self.size + pos.y as usize;
        self.cells
            .get(index)
            .copied()
            .flatten()
            .and_then(|view_index| self.units.get(view_index))
    }
}

/// Buildings stored in map order plus a `pos -> view` lookup that is `O(1)`.
struct BuildingIndex {
    buildings: Vec<BuildingView>,
    cells: Vec<Option<usize>>,
    size: usize,
}

impl BuildingIndex {
    fn at(&self, pos: IVec2) -> Option<&BuildingView> {
        if !pos_in_bounds(pos, self.size) {
            return None;
        }
        let index = pos.x as usize * self.size + pos.y as usize;
        self.cells
            .get(index)
            .copied()
            .flatten()
            .and_then(|view_index| self.buildings.get(view_index))
    }
}

/// One discovered biome type. Only unique types are listed, never every tile.
struct BiomeEntry {
    kind: BiomeType,
    tiles: usize,
}

/// One discovered resource kind. Only unique kinds are listed.
enum ResourceEntry {
    Common {
        kind: CommonKind,
        tiles: usize,
        amount: u32,
    },
    Rare {
        kind: RareKind,
        tiles: usize,
        power: u32,
    },
}

impl ResourceEntry {
    fn matches(&self, resource: &ResourceType) -> bool {
        match (self, resource) {
            (Self::Common { kind, .. }, ResourceType::Common(common)) => *kind == common.kind,
            (Self::Rare { kind, .. }, ResourceType::Rare(rare)) => *kind == rare.kind,
            _ => false,
        }
    }

    fn key(&self) -> &'static str {
        match self {
            Self::Common { kind, .. } => common_kind_key(*kind),
            Self::Rare { kind, .. } => rare_kind_key(*kind),
        }
    }

    fn tiles(&self) -> usize {
        match self {
            Self::Common { tiles, .. } | Self::Rare { tiles, .. } => *tiles,
        }
    }

    fn detail(&self, ui: &UiContext, language: Language) -> String {
        match self {
            Self::Common { amount, .. } => {
                format!("{}: {amount}", ui.get(language, "amount"))
            }
            Self::Rare { power, .. } => format!("{}: {power}", ui.get(language, "power")),
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Common { kind, .. } => common_color(*kind),
            Self::Rare { .. } => theme::GOLD_TEXT,
        }
    }
}

/// Aggregated view of everything the player has explored.
struct Discovery {
    biomes: Vec<BiomeEntry>,
    resources: Vec<ResourceEntry>,
    resource_tiles: usize,
}

// Spawning

pub(crate) fn spawn_sidebar(
    commands: &mut Commands,
    ui: &UiContext,
    language: Language,
    players: &Players,
    config: &GameLogicConfig,
    races: &RacesUiRegistry,
) {
    let panel = commands
        .spawn((
            HudSidebar,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(78.0),
                left: Val::Px(10.0),
                bottom: Val::Px(16.0),
                width: Val::Px(318.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(8.0),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(theme::PANEL_BG),
            BorderColor::all(theme::BORDER),
            Pickable::default(),
            GlobalZIndex(10),
            DespawnOnExit(AppState::InGameRenderer),
        ))
        .id();

    spawn_player_card(commands, panel, ui, language, players, config, races);

    commands.spawn((
        SidebarBody,
        ScrollArea,
        Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(10.0),
            padding: UiRect::right(Val::Px(4.0)),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        ChildOf(panel),
    ));

    theme::spawn_translation(
        commands,
        panel,
        ui,
        language,
        "sidebar_hint",
        9.0,
        theme::MUTED,
    );
}

/// Hides or shows the whole sidebar panel according to [`HudUiState::sidebar_open`].
pub(crate) fn sync_sidebar_visibility(
    hud: Res<HudUiState>,
    mut sidebars: Query<&mut Node, With<HudSidebar>>,
) {
    let wanted = if hud.sidebar_open {
        Display::Flex
    } else {
        Display::None
    };

    for mut node in &mut sidebars {
        if node.display != wanted {
            node.display = wanted;
        }
    }
}

fn spawn_player_card(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    players: &Players,
    config: &GameLogicConfig,
    races: &RacesUiRegistry,
) {
    let Some(player) = players
        .0
        .iter()
        .find(|player| player.id == config.player.id)
    else {
        return;
    };

    let card = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(theme::SLOT_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::IGNORE,
            ChildOf(parent),
        ))
        .id();

    if let Some(race_data) = races.0.get(&player.player_race) {
        theme::spawn_race_icon(commands, card, race_data.ui_icon.clone(), 38.0);
    } else {
        theme::spawn_dot(
            commands,
            card,
            theme::race_color(&player.player_race.0),
            38.0,
        );
    }

    let text = commands.spawn((theme::column(1.0), ChildOf(card))).id();

    theme::spawn_label(
        commands,
        text,
        ui,
        language,
        theme::race_display_name(&player.player_race.0),
        16.0,
        theme::GOLD_TEXT,
    );
    theme::spawn_label(
        commands,
        text,
        ui,
        language,
        format!("{} {}", ui.get(language, "player"), player.id.0),
        9.0,
        theme::MUTED,
    );

    commands.spawn((theme::spacer(), Pickable::IGNORE, ChildOf(card)));

    let close = commands
        .spawn((
            Node {
                width: Val::Px(22.0),
                height: Val::Px(22.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(theme::BUTTON_BG),
            BorderColor::all(theme::BORDER_SOFT),
            Pickable::default(),
            ChildOf(card),
        ))
        .id();

    theme::spawn_label(commands, close, ui, language, "x", 11.0, theme::TEXT);
    theme::add_hover(commands, close, theme::BUTTON_BG, theme::BUTTON_BG_HOVER);

    commands
        .entity(close)
        .observe(|mut click: On<PointerClick>, mut hud: ResMut<HudUiState>| {
            click.propagate(false);
            hud.sidebar_open = false;
        });
}

// Rebuild system

pub(crate) fn rebuild_sidebar(
    mut commands: Commands,
    resources: SidebarResources,
    queries: SidebarQueries,
) {
    let SidebarResources {
        ui,
        settings,
        config,
        players,
        selected,
        units_map,
        buildings_map,
        resources_map,
        terrain,
        fog,
        turn_manager,
        cycle,
        orders,
        combat_ui,
        mut view,
    } = resources;
    let SidebarQueries {
        body: body_query,
        combat_units,
        helpers,
        buildings,
    } = queries;

    let state_changed = selected.is_changed()
        || settings.is_changed()
        || players.is_changed()
        || units_map.is_changed()
        || buildings_map.is_changed()
        || resources_map.is_changed()
        || terrain.is_changed()
        || fog.is_changed()
        || turn_manager.is_changed()
        || cycle.is_changed()
        || orders.is_changed()
        || combat_ui.is_changed()
        || view.is_changed();

    println!("cycle changed to {:?}", cycle.phase);

    let Ok((body, scroll, children)) = body_query.single() else {
        return;
    };
    let is_initial = children.is_none_or(|children| children.is_empty());
    if !state_changed && !is_initial {
        return;
    }

    let scroll = scroll.0;
    commands.entity(body).despawn_children();
    commands.entity(body).insert(ScrollPosition(scroll));

    let language = settings.language;
    let local_id = config.player.id;
    let local_player = players.0.iter().find(|player| player.id == local_id);
    let in_combat = !matches!(cycle.phase, CombatPhase::Idle);
    let planning = matches!(cycle.phase, CombatPhase::PlayerPlanning);

    let units = gather_units(
        &units_map,
        &fog,
        &buildings_map,
        &resources_map,
        &combat_units,
        &helpers,
    );
    let cities = gather_buildings(&buildings_map, &buildings);
    let discovery = gather_discovery(&terrain, &fog, &resources_map);
    let selection = *selected;

    // `bypass_change_detection` is important here: mutably dereferencing the
    // `ResMut` would mark `SidebarView` as changed on every frame, which makes
    // `view.is_changed()` true forever and rebuilds (and despawns) the whole
    // body every frame. That breaks click observers, because the press and the
    // release end up on two different entities.
    sync_view(view.bypass_change_detection(), selection, &units);

    let own_units: Vec<&UnitView> = units
        .units
        .iter()
        .filter(|unit| unit.owner == local_id)
        .collect();
    let known_enemies: Vec<&UnitView> = units
        .units
        .iter()
        .filter(|unit| unit.owner != local_id && enemy_visible(unit, &fog))
        .collect();
    let own_cities: Vec<&BuildingView> = cities
        .buildings
        .iter()
        .filter(|city| city.owner == local_id)
        .collect();
    let scrolls = local_player
        .map(|player| player.inventory.len())
        .unwrap_or(0);

    spawn_summary(
        &mut commands,
        body,
        &ui,
        language,
        own_units.len(),
        own_cities.len(),
        &discovery,
        scrolls,
    );

    spawn_units_section(
        &mut commands,
        body,
        &ui,
        language,
        &units,
        &own_units,
        selection,
        in_combat,
        &view,
        local_id,
    );

    if in_combat {
        spawn_enemies_section(
            &mut commands,
            body,
            &ui,
            language,
            &known_enemies,
            selection,
            planning,
        );
        spawn_orders(
            &mut commands,
            body,
            &ui,
            language,
            &orders,
            combat_ui.armed_skill,
        );
    }

    spawn_buildings_section(
        &mut commands,
        body,
        &ui,
        language,
        &cities,
        &own_cities,
        selection,
        local_id,
        &view,
    );
    spawn_resources_section(
        &mut commands,
        body,
        &ui,
        language,
        &discovery,
        selection,
        &resources_map,
        local_id,
        &view,
    );
    spawn_terrain_section(
        &mut commands,
        body,
        &ui,
        language,
        &discovery,
        selection,
        &terrain,
        &view,
    );

    if let Some(player) = local_player {
        spawn_inventory(&mut commands, body, &ui, language, player);
    }
}

/// Opens the section that matches the newly selected tile and switches the unit
/// tab to the group the selected unit belongs to. Only writes when the selection
/// actually changed, so it doesn't force a rebuild every frame.
fn sync_view(view: &mut SidebarView, selected: AlreadySelectedTile, units: &UnitIndex) {
    let key = selected.pos.zip(selected.layer);
    if view.last_selection == key {
        return;
    }
    view.last_selection = key;

    let Some((pos, layer)) = key else {
        return;
    };

    match layer {
        TileLayer::Unit => {
            if !view.units_open {
                view.units_open = true;
            }
            if let Some(unit) = units.at(pos) {
                let tab = if unit.kind == UnitType::Helper {
                    UnitTab::Builder
                } else {
                    UnitTab::Combat
                };
                if view.unit_tab != tab {
                    view.unit_tab = tab;
                }
            }
        }
        TileLayer::Building => {
            if !view.buildings_open {
                view.buildings_open = true;
            }
        }
        TileLayer::Resource => {
            if !view.resources_open {
                view.resources_open = true;
            }
        }
        TileLayer::Terrain | TileLayer::Fog => {
            if !view.terrain_open {
                view.terrain_open = true;
            }
        }
    }
}

// Gathering

fn gather_units(
    units_map: &UnitsMap,
    fog: &FogMap,
    buildings_map: &BuildingMap,
    resources_map: &ResourceMap,
    combat_units: &CombatUnitsQuery,
    helpers: &HelpersQuery,
) -> UnitIndex {
    let size = units_map.size;
    let mut units = Vec::new();
    let mut cells = vec![None; size * size];

    for (index, slot) in units_map.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let pos = IVec2::new((index / size) as i32, (index % size) as i32);
        let tile_content = tile_content_at(pos, units_map, buildings_map, resources_map, fog);

        let view =
            if let Ok((id, race, unit_type, level, basic, hp, movement, movement_max, abilities)) =
                combat_units.get(entity)
            {
                let visible_abilities = (*level as usize).min(abilities.0.len().saturating_sub(1));
                let ability_views = abilities
                    .0
                    .iter()
                    .take(visible_abilities + 1)
                    .map(|ability| AbilityView {
                        name: ability.name.to_string(),
                        damage: ability.damage,
                        mana: ability.mana_amount_needs,
                        cooldown: ability.cool_down_turns,
                        range: ability.range,
                        area: ability.area,
                    })
                    .collect();

                UnitView {
                    pos,
                    owner: *id,
                    race: race.clone(),
                    kind: *unit_type,
                    level: Some(*level),
                    hp: hp.0.min(basic.health),
                    max_hp: basic.health,
                    attack: basic.attack,
                    defense: basic.defense,
                    range: basic.range,
                    moves: movement.0,
                    moves_max: movement_max.0,
                    build_points: None,
                    abilities: ability_views,
                    tile_content,
                }
            } else if let Ok((id, race, basic, movement, movement_max)) = helpers.get(entity) {
                UnitView {
                    pos,
                    owner: *id,
                    race: race.clone(),
                    kind: UnitType::Helper,
                    level: None,
                    hp: basic.health,
                    max_hp: basic.health,
                    attack: 0,
                    defense: basic.defense,
                    range: 0,
                    moves: movement.0,
                    moves_max: movement_max.0,
                    build_points: Some(basic.build_gather_points),
                    abilities: Vec::new(),
                    tile_content,
                }
            } else {
                continue;
            };

        cells[index] = Some(units.len());
        units.push(view);
    }

    UnitIndex { units, cells, size }
}

fn gather_buildings(buildings_map: &BuildingMap, buildings: &BuildingsQuery) -> BuildingIndex {
    let size = buildings_map.size;
    let mut views = Vec::new();
    let mut cells = vec![None; size * size];

    for (index, slot) in buildings_map.map.iter().enumerate() {
        let Some(entity) = *slot else { continue };
        let Ok((id, kind, level, levels)) = buildings.get(entity) else {
            continue;
        };
        let pos = IVec2::new((index / size) as i32, (index % size) as i32);
        let stats = levels.stats(*level);

        cells[index] = Some(views.len());
        views.push(BuildingView {
            pos,
            owner: *id,
            kind: *kind,
            level: *level,
            gold: stats.gold(),
            food: stats.food(),
            science: stats.science(),
            housing: stats.housing(),
            spawn_capacity: stats.spawn_capacity(),
            maintenance: stats.maintenance(),
        });
    }

    BuildingIndex {
        buildings: views,
        cells,
        size,
    }
}

/// Walks the explored part of the map once and aggregates unique biomes and
/// resource kinds. `O(size * size)` per rebuild, not per lookup.
fn gather_discovery(terrain: &TerrainMap, fog: &FogMap, resources_map: &ResourceMap) -> Discovery {
    const BIOME_ORDER: [BiomeType; 5] = [
        BiomeType::SmallBiome,
        BiomeType::MediumBiome,
        BiomeType::LargeBiome,
        BiomeType::Ground,
        BiomeType::Water,
    ];

    let size = terrain.size;
    let mut biome_counts = [0usize; 5];
    let mut common_tiles = [0usize; 3];
    let mut common_amount = [0u32; 3];
    let mut rare_tiles = [0usize; 4];
    let mut rare_power = [0u32; 4];
    let mut resource_tiles = 0usize;

    for (index, biome) in terrain.map.iter().enumerate() {
        let row = index / size;
        let col = index % size;
        if !fog.is_general_visible_player(row, col) {
            continue;
        }

        biome_counts[biome.biome_type as usize] += 1;

        let Some(Some(resource)) = resources_map.map.get(index) else {
            continue;
        };
        resource_tiles += 1;
        match resource {
            ResourceType::Common(common) => {
                let slot = common.kind.index();
                common_tiles[slot] += 1;
                common_amount[slot] = common_amount[slot].saturating_add(common.amount);
            }
            ResourceType::Rare(rare) => {
                let slot = rare.kind as usize;
                rare_tiles[slot] += 1;
                rare_power[slot] = rare_power[slot].max(rare.power);
            }
        }
    }

    let biomes = BIOME_ORDER
        .iter()
        .filter_map(|kind| {
            let tiles = biome_counts[*kind as usize];
            (tiles > 0).then_some(BiomeEntry { kind: *kind, tiles })
        })
        .collect();

    let mut resources = Vec::new();
    for kind in CommonKind::ALL {
        let slot = kind.index();
        if common_tiles[slot] > 0 {
            resources.push(ResourceEntry::Common {
                kind,
                tiles: common_tiles[slot],
                amount: common_amount[slot],
            });
        }
    }
    for kind in RareKind::ALL {
        let slot = kind as usize;
        if rare_tiles[slot] > 0 {
            resources.push(ResourceEntry::Rare {
                kind,
                tiles: rare_tiles[slot],
                power: rare_power[slot],
            });
        }
    }

    Discovery {
        biomes,
        resources,
        resource_tiles,
    }
}

fn tile_content_at(
    pos: IVec2,
    units_map: &UnitsMap,
    buildings_map: &BuildingMap,
    resources_map: &ResourceMap,
    fog: &FogMap,
) -> TileContent {
    if !pos_in_bounds(pos, units_map.size) {
        return TileContent::default();
    }
    let (x, y) = (pos.x as usize, pos.y as usize);
    TileContent::from_world(
        true,
        units_map.is_some(x, y),
        buildings_map.is_some(x, y),
        resources_map.is_some(x, y),
        !fog.is_general_visible_player(x, y),
    )
}

fn enemy_visible(unit: &UnitView, fog: &FogMap) -> bool {
    pos_in_bounds(unit.pos, fog.size)
        && (fog.debug_full_visibility
            || fog.is_full_visible_player(unit.pos.x as usize, unit.pos.y as usize))
}

fn pos_in_bounds(pos: IVec2, size: usize) -> bool {
    pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < size && (pos.y as usize) < size
}

fn resource_at(pos: IVec2, resources_map: &ResourceMap) -> Option<&ResourceType> {
    if !pos_in_bounds(pos, resources_map.size) {
        return None;
    }
    resources_map.get(pos.x as usize, pos.y as usize)
}

fn biome_at(pos: IVec2, terrain: &TerrainMap) -> Option<BiomeType> {
    if !pos_in_bounds(pos, terrain.size) {
        return None;
    }
    Some(terrain.get_biome(pos.x as usize, pos.y as usize).biome_type)
}

// Sections

#[allow(clippy::too_many_arguments)]
fn spawn_summary(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    units: usize,
    buildings: usize,
    discovery: &Discovery,
    scrolls: usize,
) {
    let block = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                padding: UiRect::all(Val::Px(8.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(theme::PANEL_BG_SOFT),
            Pickable::IGNORE,
            ChildOf(body),
        ))
        .id();

    let mut counts_node = theme::row(10.0);
    counts_node.flex_wrap = FlexWrap::Wrap;
    let counts = commands
        .spawn((counts_node, Pickable::IGNORE, ChildOf(block)))
        .id();
    spawn_count_chip(commands, counts, ui, language, "units", units);
    spawn_count_chip(commands, counts, ui, language, "buildings", buildings);
    spawn_count_chip(
        commands,
        counts,
        ui,
        language,
        "discovered_biomes",
        discovery.biomes.len(),
    );
    spawn_count_chip(
        commands,
        counts,
        ui,
        language,
        "discovered_resources",
        discovery.resource_tiles,
    );
    spawn_count_chip(commands, counts, ui, language, "inventory", scrolls);
}

fn spawn_count_chip(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
    value: usize,
) {
    let chip = commands.spawn((theme::row(4.0), ChildOf(parent))).id();
    theme::spawn_translation(commands, chip, ui, language, key, 9.0, theme::MUTED);
    theme::spawn_label(
        commands,
        chip,
        ui,
        language,
        value.to_string(),
        10.0,
        theme::TEXT,
    );
}

/// Clickable section header that folds/unfolds the tree section below it.
fn spawn_collapsible_header(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
    open: bool,
    section: SidebarSection,
) {
    let header = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(6.0),
                padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(theme::SLOT_BG),
            Pickable::default(),
            ChildOf(parent),
        ))
        .id();

    theme::spawn_label(
        commands,
        header,
        ui,
        language,
        if open { "-" } else { "+" },
        12.0,
        theme::GOLD_TEXT,
    );
    theme::spawn_label(
        commands,
        header,
        ui,
        language,
        theme::resolve(ui, language, key).to_uppercase(),
        11.0,
        theme::GOLD_TEXT,
    );

    theme::add_hover(commands, header, theme::SLOT_BG, theme::SLOT_BG_HOVER);

    commands.entity(header).observe(
        move |mut click: On<PointerClick>, mut view: ResMut<SidebarView>| {
            click.propagate(false);
            view.toggle(section);
        },
    );
}

fn spawn_unit_tab(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    key: &'static str,
    tab: UnitTab,
    active: bool,
) {
    let bg = if active {
        theme::PRIMARY_BG
    } else {
        theme::BUTTON_BG
    };
    let button = theme::spawn_text_button(
        commands,
        parent,
        ui,
        language,
        key,
        bg,
        theme::BUTTON_BG_HOVER,
    );

    commands.entity(button).observe(
        move |mut click: On<PointerClick>, mut view: ResMut<SidebarView>| {
            click.propagate(false);
            if view.unit_tab != tab {
                view.unit_tab = tab;
            }
        },
    );
}

#[allow(clippy::too_many_arguments)]
fn spawn_units_section(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    units: &UnitIndex,
    own_units: &[&UnitView],
    selected: AlreadySelectedTile,
    in_combat: bool,
    view: &SidebarView,
    local_id: Id,
) {
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();
    let header_key = if in_combat { "your_units" } else { "units" };
    spawn_collapsible_header(
        commands,
        section,
        ui,
        language,
        header_key,
        view.units_open,
        SidebarSection::Units,
    );

    if !view.units_open {
        return;
    }

    let tabs = commands.spawn((theme::row(6.0), ChildOf(section))).id();
    spawn_unit_tab(
        commands,
        tabs,
        ui,
        language,
        "combat_tab",
        UnitTab::Combat,
        view.unit_tab == UnitTab::Combat,
    );
    spawn_unit_tab(
        commands,
        tabs,
        ui,
        language,
        "builder_tab",
        UnitTab::Builder,
        view.unit_tab == UnitTab::Builder,
    );

    let want_helper = view.unit_tab == UnitTab::Builder;
    let listed: Vec<&UnitView> = own_units
        .iter()
        .copied()
        .filter(|unit| (unit.kind == UnitType::Helper) == want_helper)
        .collect();

    if listed.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_units",
            10.0,
            theme::MUTED,
        );
        return;
    }

    // O(1) selected lookup through the map index instead of a linear search.
    let selected_unit = if selected.layer == Some(TileLayer::Unit) {
        selected.pos.and_then(|pos| units.at(pos))
    } else {
        None
    };

    for unit in listed {
        let is_selected = selected_unit.is_some_and(|selected| std::ptr::eq(selected, unit));
        let row = commands
            .spawn((theme::list_row(is_selected), ChildOf(section)))
            .id();

        theme::add_hover(
            commands,
            row,
            theme::list_row_bg(is_selected),
            theme::SLOT_BG_HOVER,
        );

        theme::spawn_dot(commands, row, unit_color(unit), 8.0);

        let text = commands.spawn((theme::grow_column(1.0), ChildOf(row))).id();
        theme::spawn_label(
            commands,
            text,
            ui,
            language,
            unit_title(ui, language, unit),
            12.0,
            theme::TEXT,
        );
        theme::spawn_label(
            commands,
            text,
            ui,
            language,
            unit_summary(unit),
            9.0,
            theme::MUTED,
        );

        attach_select(commands, row, unit.pos, TileLayer::Unit);

        if is_selected {
            spawn_unit_details(commands, section, ui, language, unit, local_id);
        }
    }
}

fn spawn_enemies_section(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    enemies: &[&UnitView],
    selected: AlreadySelectedTile,
    planning: bool,
) {
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();
    theme::spawn_section_header(commands, section, ui, language, "known_enemies");

    if enemies.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_visible_enemies",
            10.0,
            theme::MUTED,
        );
        return;
    }

    for unit in enemies {
        let is_selected = selected.pos == Some(unit.pos) && selected.layer == Some(TileLayer::Unit);
        let row = commands
            .spawn((theme::list_row(is_selected), ChildOf(section)))
            .id();

        theme::add_hover(
            commands,
            row,
            theme::list_row_bg(is_selected),
            theme::SLOT_BG_HOVER,
        );

        theme::spawn_dot(commands, row, unit_color(unit), 8.0);

        let text = commands.spawn((theme::grow_column(1.0), ChildOf(row))).id();
        theme::spawn_label(
            commands,
            text,
            ui,
            language,
            unit_title(ui, language, unit),
            12.0,
            theme::TEXT,
        );
        theme::spawn_label(
            commands,
            text,
            ui,
            language,
            unit_summary(unit),
            9.0,
            theme::MUTED,
        );

        if planning {
            attach_queue_order(commands, row, unit.pos, unit.tile_content.clone());
        } else {
            attach_select(commands, row, unit.pos, TileLayer::Unit);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_buildings_section(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    cities: &BuildingIndex,
    own_cities: &[&BuildingView],
    selected: AlreadySelectedTile,
    local_id: Id,
    view: &SidebarView,
) {
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();
    spawn_collapsible_header(
        commands,
        section,
        ui,
        language,
        "buildings",
        view.buildings_open,
        SidebarSection::Buildings,
    );

    if !view.buildings_open {
        return;
    }

    if own_cities.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_buildings",
            10.0,
            theme::MUTED,
        );
        return;
    }

    let selected_city = if selected.layer == Some(TileLayer::Building) {
        selected.pos.and_then(|pos| cities.at(pos))
    } else {
        None
    };

    for city in own_cities.iter().copied() {
        let is_selected = selected_city.is_some_and(|selected| std::ptr::eq(selected, city));
        let row = commands
            .spawn((theme::list_row(is_selected), ChildOf(section)))
            .id();

        theme::add_hover(
            commands,
            row,
            theme::list_row_bg(is_selected),
            theme::SLOT_BG_HOVER,
        );

        theme::spawn_dot(commands, row, theme::GOLD_TEXT, 8.0);

        let text = commands.spawn((theme::grow_column(1.0), ChildOf(row))).id();
        theme::spawn_label(
            commands,
            text,
            ui,
            language,
            format!(
                "{} {}",
                ui.get(language, building_key(city.kind)),
                level_roman(city.level)
            ),
            12.0,
            theme::TEXT,
        );
        theme::spawn_label(
            commands,
            text,
            ui,
            language,
            format!(
                "+{}g +{}f +{}s  ·  ({}, {})",
                city.gold, city.food, city.science, city.pos.x, city.pos.y
            ),
            9.0,
            theme::MUTED,
        );

        attach_select(commands, row, city.pos, TileLayer::Building);

        if is_selected {
            spawn_building_details(commands, section, ui, language, city, local_id);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_resources_section(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    discovery: &Discovery,
    selected: AlreadySelectedTile,
    resources_map: &ResourceMap,
    local_id: Id,
    view: &SidebarView,
) {
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();
    spawn_collapsible_header(
        commands,
        section,
        ui,
        language,
        "resources",
        view.resources_open,
        SidebarSection::Resources,
    );

    if !view.resources_open {
        return;
    }

    if discovery.resources.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_resources",
            10.0,
            theme::MUTED,
        );
        return;
    }

    let selected_resource = if selected.layer == Some(TileLayer::Resource) {
        selected.pos.and_then(|pos| resource_at(pos, resources_map))
    } else {
        None
    };

    for entry in &discovery.resources {
        let is_selected = selected_resource.is_some_and(|resource| entry.matches(resource));
        let row = commands
            .spawn((theme::list_row(is_selected), ChildOf(section)))
            .id();

        theme::spawn_dot(commands, row, entry.color(), 8.0);

        let text = commands.spawn((theme::grow_column(1.0), ChildOf(row))).id();
        theme::spawn_translation(commands, text, ui, language, entry.key(), 12.0, theme::TEXT);
        theme::spawn_label(
            commands,
            text,
            ui,
            language,
            format!(
                "{} {}  ·  {}",
                entry.tiles(),
                ui.get(language, "tiles"),
                entry.detail(ui, language)
            ),
            9.0,
            theme::MUTED,
        );

        if is_selected && let Some(resource) = selected_resource {
            spawn_resource_details(commands, section, ui, language, resource, local_id);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_terrain_section(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    discovery: &Discovery,
    selected: AlreadySelectedTile,
    terrain: &TerrainMap,
    view: &SidebarView,
) {
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();
    spawn_collapsible_header(
        commands,
        section,
        ui,
        language,
        "terrain",
        view.terrain_open,
        SidebarSection::Terrain,
    );

    if !view.terrain_open {
        return;
    }

    if discovery.biomes.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_terrain",
            10.0,
            theme::MUTED,
        );
        return;
    }

    let selected_biome = if selected.layer == Some(TileLayer::Terrain) {
        selected.pos.and_then(|pos| biome_at(pos, terrain))
    } else {
        None
    };

    for entry in &discovery.biomes {
        let is_selected = selected_biome == Some(entry.kind);
        let row = commands
            .spawn((theme::list_row(is_selected), ChildOf(section)))
            .id();

        theme::spawn_dot(commands, row, biome_color(entry.kind), 8.0);

        let text = commands.spawn((theme::grow_column(1.0), ChildOf(row))).id();
        theme::spawn_translation(
            commands,
            text,
            ui,
            language,
            biome_key(entry.kind),
            12.0,
            theme::TEXT,
        );
        theme::spawn_label(
            commands,
            text,
            ui,
            language,
            format!("{} {}", entry.tiles, ui.get(language, "tiles")),
            9.0,
            theme::MUTED,
        );

        theme::spawn_label(
            commands,
            row,
            ui,
            language,
            move_cost_label(ui, language, entry.kind),
            10.0,
            theme::MOVE_POINT,
        );
    }
}

fn spawn_unit_details(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    unit: &UnitView,
    local_id: Id,
) {
    let card = commands
        .spawn((theme::card(theme::SLOT_BG), ChildOf(parent)))
        .id();

    let title = commands.spawn((theme::title_row(), ChildOf(card))).id();
    theme::spawn_label(
        commands,
        title,
        ui,
        language,
        unit_title(ui, language, unit),
        13.0,
        unit_color(unit),
    );
    theme::spawn_label(
        commands,
        title,
        ui,
        language,
        theme::race_display_name(&unit.race.0),
        9.0,
        theme::MUTED,
    );

    let owner = owner_label(ui, language, unit.owner, local_id);
    theme::spawn_stat_row(commands, card, ui, language, "owner", owner, theme::TEXT);

    theme::spawn_stat_row(
        commands,
        card,
        ui,
        language,
        "health",
        format!("{} / {}", unit.hp, unit.max_hp),
        hp_color(unit.hp, unit.max_hp),
    );
    theme::spawn_health_bar(commands, card, 160.0, unit.hp, unit.max_hp);

    theme::spawn_stat_row(
        commands,
        card,
        ui,
        language,
        "moves",
        format!("{} / {}", unit.moves, unit.moves_max),
        theme::MOVE_POINT,
    );

    if unit.kind == UnitType::Helper {
        theme::spawn_stat_row(
            commands,
            card,
            ui,
            language,
            "build_charges",
            unit.build_points.unwrap_or(0).to_string(),
            theme::TEXT,
        );
    } else {
        theme::spawn_stat_row(
            commands,
            card,
            ui,
            language,
            "attack_defense",
            format!("{} / {}", unit.attack, unit.defense),
            theme::TEXT,
        );
        theme::spawn_stat_row(
            commands,
            card,
            ui,
            language,
            "range",
            unit.range.to_string(),
            theme::TEXT,
        );
    }

    if !unit.abilities.is_empty() {
        theme::spawn_translation(
            commands,
            card,
            ui,
            language,
            "abilities",
            9.0,
            theme::GOLD_TEXT,
        );
        for ability in &unit.abilities {
            let stats = format!(
                "{} {} · {} {} · {} {} · {} {} · {} {}",
                ui.get(language, "damage"),
                ability.damage,
                ui.get(language, "mana"),
                ability.mana,
                ui.get(language, "cooldown"),
                ability.cooldown,
                ui.get(language, "range"),
                ability.range,
                ui.get(language, "area"),
                ability.area,
            );
            theme::spawn_wrapped_text(
                commands,
                card,
                ui,
                language,
                format!("{}: {stats}", ability.name),
                10.0,
                theme::MUTED,
            );
        }
    }
}

fn spawn_building_details(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    city: &BuildingView,
    local_id: Id,
) {
    let card = commands
        .spawn((theme::card(theme::SLOT_BG), ChildOf(parent)))
        .id();

    let title = commands.spawn((theme::title_row(), ChildOf(card))).id();
    theme::spawn_translation(
        commands,
        title,
        ui,
        language,
        building_key(city.kind),
        13.0,
        theme::GOLD_TEXT,
    );
    theme::spawn_label(
        commands,
        title,
        ui,
        language,
        level_roman(city.level),
        10.0,
        theme::TEXT,
    );

    let owner = owner_label(ui, language, city.owner, local_id);
    theme::spawn_stat_row(commands, card, ui, language, "owner", owner, theme::TEXT);
    theme::spawn_stat_row(
        commands,
        card,
        ui,
        language,
        "yields",
        format!("{}g / {}f / {}s", city.gold, city.food, city.science),
        theme::TEXT,
    );
    theme::spawn_stat_row(
        commands,
        card,
        ui,
        language,
        "maintenance",
        format!("{}g", city.maintenance),
        theme::MUTED,
    );
    theme::spawn_stat_row(
        commands,
        card,
        ui,
        language,
        "housing_spawn",
        format!("{} / {}", city.housing, city.spawn_capacity),
        theme::TEXT,
    );
}

fn spawn_resource_details(
    commands: &mut Commands,
    parent: Entity,
    ui: &UiContext,
    language: Language,
    resource: &ResourceType,
    local_id: Id,
) {
    let card = commands
        .spawn((theme::card(theme::PANEL_BG_SOFT), ChildOf(parent)))
        .id();

    match resource {
        ResourceType::Common(common) => {
            theme::spawn_translation(
                commands,
                card,
                ui,
                language,
                common_kind_key(common.kind),
                13.0,
                theme::TEXT,
            );
            theme::spawn_stat_row(
                commands,
                card,
                ui,
                language,
                "amount",
                common.amount.to_string(),
                theme::TEXT,
            );
            if let Some(owner) = common.owner_id {
                let claimed_by = owner_label(ui, language, owner, local_id);
                theme::spawn_stat_row(
                    commands,
                    card,
                    ui,
                    language,
                    "claimed_by",
                    claimed_by,
                    theme::TEXT,
                );
            }
        }
        ResourceType::Rare(rare) => {
            theme::spawn_translation(
                commands,
                card,
                ui,
                language,
                rare_kind_key(rare.kind),
                13.0,
                theme::GOLD_TEXT,
            );
            theme::spawn_stat_row(
                commands,
                card,
                ui,
                language,
                "power",
                rare.power.to_string(),
                theme::TEXT,
            );
            theme::spawn_wrapped_translation(
                commands,
                card,
                ui,
                language,
                rare_description_key(rare.kind),
                10.0,
                theme::MUTED,
            );
        }
    }
}

fn spawn_orders(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    orders: &CombatOrders,
    armed_skill: u8,
) {
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();
    theme::spawn_section_header(commands, section, ui, language, "orders");

    theme::spawn_stat_row(
        commands,
        section,
        ui,
        language,
        "skill",
        skill_name(ui, language, armed_skill),
        theme::GOLD_TEXT,
    );

    if orders.0.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "no_orders",
            10.0,
            theme::MUTED,
        );
        return;
    }

    for (index, order) in orders.0.iter().enumerate() {
        let row = commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    column_gap: Val::Px(6.0),
                    padding: UiRect::axes(Val::Px(7.0), Val::Px(3.0)),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(theme::SLOT_BG),
                Pickable::default(),
                ChildOf(section),
            ))
            .id();

        theme::spawn_label(
            commands,
            row,
            ui,
            language,
            format!(
                "({}, {}) → ({}, {})  ·  {}",
                order.attacker_pos.x,
                order.attacker_pos.y,
                order.target_pos.x,
                order.target_pos.y,
                skill_name(ui, language, order.skill_type)
            ),
            10.0,
            theme::TEXT,
        );

        let remove = commands
            .spawn((
                Node {
                    width: Val::Px(18.0),
                    height: Val::Px(18.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(theme::DANGER_BG),
                BorderColor::all(theme::BORDER_SOFT),
                Pickable::default(),
                ChildOf(row),
            ))
            .id();

        theme::spawn_label(commands, remove, ui, language, "x", 10.0, theme::TEXT);

        commands.entity(remove).observe(
            move |mut click: On<PointerClick>, mut orders: ResMut<CombatOrders>| {
                click.propagate(false);
                if index < orders.0.len() {
                    orders.0.remove(index);
                }
            },
        );
        theme::add_hover(commands, remove, theme::DANGER_BG, theme::DANGER_BG_HOVER);
    }
}

fn spawn_inventory(
    commands: &mut Commands,
    body: Entity,
    ui: &UiContext,
    language: Language,
    player: &PlayerData,
) {
    let section = commands.spawn((theme::column(4.0), ChildOf(body))).id();
    theme::spawn_section_header(commands, section, ui, language, "inventory");

    if player.inventory.is_empty() {
        theme::spawn_translation(
            commands,
            section,
            ui,
            language,
            "empty_inventory",
            10.0,
            theme::MUTED,
        );
        return;
    }

    for scroll in &player.inventory {
        let mut row_node = theme::row(7.0);
        row_node.width = Val::Percent(100.0);
        row_node.padding = UiRect::axes(Val::Px(7.0), Val::Px(3.0));
        row_node.border_radius = BorderRadius::all(Val::Px(4.0));
        let row = commands
            .spawn((
                row_node,
                BackgroundColor(theme::SLOT_BG),
                Pickable::IGNORE,
                ChildOf(section),
            ))
            .id();

        theme::spawn_dot(commands, row, theme::GOLD_TEXT, 7.0);

        let text = commands.spawn((theme::grow_column(1.0), ChildOf(row))).id();
        theme::spawn_translation(
            commands,
            text,
            ui,
            language,
            rare_kind_key(scroll.kind),
            11.0,
            theme::TEXT,
        );
        theme::spawn_wrapped_translation(
            commands,
            text,
            ui,
            language,
            rare_description_key(scroll.kind),
            9.0,
            theme::MUTED,
        );
        theme::spawn_label(
            commands,
            text,
            ui,
            language,
            format!("{} {}", ui.get(language, "power"), scroll.power),
            9.0,
            theme::MUTED,
        );
    }
}

// Row interactions

/// Selects a tile by both position and layer. Rows know which layer they
/// represent, so selecting a building under a unit still shows the building.
fn attach_select(commands: &mut Commands, row: Entity, pos: IVec2, layer: TileLayer) {
    commands.entity(row).observe(
        move |mut click: On<PointerClick>, mut selected: ResMut<AlreadySelectedTile>| {
            click.propagate(false);
            selected.pos = Some(pos);
            selected.layer = Some(layer);
        },
    );
}

fn attach_queue_order(commands: &mut Commands, row: Entity, pos: IVec2, content: TileContent) {
    commands.entity(row).observe(
        move |mut click: On<PointerClick>,
              selected: Res<AlreadySelectedTile>,
              units_map: Res<UnitsMap>,
              ids: Query<&Id>,
              combat_levels: Query<&UnitLevel>,
              config: Res<GameLogicConfig>,
              cycle: Res<CombatCycle>,
              combat_ui: Res<CombatUiState>,
              mut orders: ResMut<CombatOrders>,
              mut new_tile: MessageWriter<NewTile>| {
            click.propagate(false);

            let attacker = selected
                .pos
                .filter(|_| selected.layer == Some(TileLayer::Unit))
                .filter(|pos| {
                    pos_in_bounds(*pos, units_map.size)
                        && units_map
                            .map
                            .get(pos.x as usize * units_map.size + pos.y as usize)
                            .and_then(|entity| entity.as_ref())
                            .is_some_and(|entity| {
                                ids.get(*entity).is_ok_and(|id| *id == config.player.id)
                                    && combat_levels.get(*entity).is_ok()
                            })
                });

            if let Some(attacker_pos) =
                attacker.filter(|_| matches!(cycle.phase, CombatPhase::PlayerPlanning))
            {
                orders.0.retain(|order| order.attacker_pos != attacker_pos);
                orders.0.push(CombatCommand {
                    attacker_pos,
                    target_pos: pos,
                    skill_type: combat_ui.armed_skill,
                });
                return;
            }

            new_tile.write(NewTile(JustSelectedTile {
                pos,
                tile_content: content.clone(),
            }));
        },
    );
}

// Formatting helpers

fn owner_label(ui: &UiContext, language: Language, owner: Id, local_id: Id) -> String {
    if owner == local_id {
        ui.get(language, "you").to_string()
    } else {
        format!("{} {}", ui.get(language, "player"), owner.0)
    }
}

/// `HP 4/4  ·  MP 2/2  ·  (3, 7)` for a list row.
fn unit_summary(unit: &UnitView) -> String {
    format!(
        "HP {}/{}  ·  MP {}/{}  ·  ({}, {})",
        unit.hp, unit.max_hp, unit.moves, unit.moves_max, unit.pos.x, unit.pos.y
    )
}

/// `Fire Mages Damage II`; builders have no level.
fn unit_title(ui: &UiContext, language: Language, unit: &UnitView) -> String {
    let kind = ui.get(language, unit_kind_key(unit.kind));

    match unit.level {
        Some(level) => format!(
            "{} {kind} {}",
            theme::race_display_name(&unit.race.0),
            unit_level_roman(level)
        ),
        None => kind.to_string(),
    }
}

fn skill_name(ui: &UiContext, language: Language, skill_type: u8) -> String {
    match skill_type {
        0 => ui.get(language, "skill_attack").to_string(),
        1..=3 => format!("{} {skill_type}", ui.get(language, "skill")),
        _ => ui.get(language, "skill").to_string(),
    }
}

fn unit_kind_key(kind: UnitType) -> &'static str {
    match kind {
        UnitType::DamageDealer => "unit_damage",
        UnitType::Support => "unit_support",
        UnitType::Skill => "unit_skill",
        UnitType::Helper => "unit_builder",
    }
}

fn unit_color(unit: &UnitView) -> Color {
    match unit.kind {
        UnitType::DamageDealer => theme::ENEMY,
        UnitType::Support => Color::srgb(0.482, 0.773, 0.443),
        UnitType::Skill => theme::RES_SCIENCE,
        UnitType::Helper => theme::RES_GOLD,
    }
}

fn building_key(kind: BuildingType) -> &'static str {
    match kind {
        BuildingType::Capital => "building_capital",
        BuildingType::CapturedCity => "building_captured_city",
        BuildingType::BarbarianCity => "building_barbarian_city",
        BuildingType::SpawningBuilding => "building_spawning",
        BuildingType::House => "building_house",
        BuildingType::MediumBuilding => "building_medium",
        BuildingType::LargeBuilding => "building_large",
        BuildingType::ResearchCenter => "building_research",
        BuildingType::UniqueRaceBuilding => "building_unique",
    }
}

fn level_roman(level: BuildingLevel) -> &'static str {
    match level {
        BuildingLevel::Standard => "I",
        BuildingLevel::Advanced => "II",
        BuildingLevel::Complex => "III",
    }
}

fn unit_level_roman(level: UnitLevel) -> &'static str {
    match level {
        UnitLevel::Beginner => "I",
        UnitLevel::Master => "II",
        UnitLevel::Holy => "III",
    }
}

fn biome_key(biome: BiomeType) -> &'static str {
    match biome {
        BiomeType::Water => "biome_water",
        BiomeType::SmallBiome => "biome_small",
        BiomeType::MediumBiome => "biome_medium",
        BiomeType::LargeBiome => "biome_large",
        BiomeType::Ground => "biome_ground",
    }
}

fn biome_color(biome: BiomeType) -> Color {
    match biome {
        BiomeType::Water => theme::MANA,
        BiomeType::SmallBiome => theme::RES_FOOD,
        BiomeType::MediumBiome => theme::RES_GOLD,
        BiomeType::LargeBiome => theme::RES_SCIENCE,
        BiomeType::Ground => theme::MUTED,
    }
}

fn move_cost_label(ui: &UiContext, language: Language, biome: BiomeType) -> String {
    let cost = biome_cost(biome);
    if cost == 0 {
        "-".to_string()
    } else {
        format!("{cost} {}", ui.get(language, "move_points"))
    }
}

fn common_kind_key(kind: CommonKind) -> &'static str {
    match kind {
        CommonKind::Food => "resource_food",
        CommonKind::Gold => "resource_gold",
        CommonKind::Science => "resource_science",
    }
}

fn common_color(kind: CommonKind) -> Color {
    match kind {
        CommonKind::Food => theme::RES_FOOD,
        CommonKind::Gold => theme::RES_GOLD,
        CommonKind::Science => theme::RES_SCIENCE,
    }
}

fn rare_kind_key(kind: RareKind) -> &'static str {
    match kind {
        RareKind::ManaScroll => "rare_mana_scroll",
        RareKind::HealScroll => "rare_heal_scroll",
        RareKind::AttackScroll => "rare_attack_scroll",
        RareKind::RevealScroll => "rare_reveal_scroll",
    }
}

fn rare_description_key(kind: RareKind) -> &'static str {
    match kind {
        RareKind::ManaScroll => "rare_mana_scroll_desc",
        RareKind::HealScroll => "rare_heal_scroll_desc",
        RareKind::AttackScroll => "rare_attack_scroll_desc",
        RareKind::RevealScroll => "rare_reveal_scroll_desc",
    }
}

fn hp_color(hp: u16, max_hp: u16) -> Color {
    let ratio = if max_hp == 0 {
        0.0
    } else {
        hp as f32 / max_hp as f32
    };
    if ratio > 0.5 {
        theme::HP_GOOD
    } else if ratio > 0.25 {
        theme::HP_WARN
    } else {
        theme::HP_LOW
    }
}
