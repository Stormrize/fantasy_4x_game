use crate::world::game_resources::data::building::BuildingType;
use crate::world::game_resources::data::unit::UnitType;
use bevy::math::IVec2;

/// Команда, которую бот отдаст через те же сообщения, что и игрок.
#[derive(Debug, Clone)]
pub enum AiAction {
    Move {
        from: IVec2,
        to: IVec2,
    },
    Gather {
        pos: IVec2,
    },
    Build {
        pos: IVec2,
        building_type: BuildingType,
    },
    /// Наём юнита в городе: `Helper` — строитель, остальные — боевые.
    SpawnUnit {
        city_pos: IVec2,
        unit_type: UnitType,
    },
    UpgradeBuilding {
        pos: IVec2,
    },
    RevealMap {
        center: IVec2,
    },
}

/// Результат планирования хода бота.
#[derive(Debug, Default)]
pub struct BotTurnPlan {
    pub actions: Vec<AiAction>,
}
