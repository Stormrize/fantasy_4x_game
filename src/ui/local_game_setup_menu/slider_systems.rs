use crate::ui::local_game_setup_menu::components_resources::{
    AI_DIFFICULTY_VARIANTS, BOT_COUNT_MIN, GameSetupSelection, MAP_SHAPE_VARIANTS,
    MAP_SIZE_VARIANTS, SliderTarget,
};

pub(super) fn apply_slider_value(
    target: SliderTarget,
    index: usize,
    selection: &mut GameSetupSelection,
) {
    match target {
        SliderTarget::MapSize => selection.map_size = MAP_SIZE_VARIANTS[index],
        SliderTarget::MapShape => selection.map_shape = MAP_SHAPE_VARIANTS[index],
        SliderTarget::AiDifficulty => selection.ai_difficulty = AI_DIFFICULTY_VARIANTS[index],
        SliderTarget::BotCount => selection.bot_count = BOT_COUNT_MIN + index,
    }
}

pub(super) fn slider_label(target: SliderTarget, index: usize) -> String {
    match target {
        SliderTarget::MapSize => format!("{:?}", MAP_SIZE_VARIANTS[index]),
        SliderTarget::MapShape => format!("{:?}", MAP_SHAPE_VARIANTS[index]),
        SliderTarget::AiDifficulty => format!("{:?}", AI_DIFFICULTY_VARIANTS[index]),
        SliderTarget::BotCount => (BOT_COUNT_MIN + index).to_string(),
    }
}

pub(super) fn slider_handle_left_percent(index: usize, steps: usize) -> f32 {
    let fraction = if steps <= 1 {
        0.0
    } else {
        index as f32 / (steps - 1) as f32
    };
    // 2.0 ~ половина ширины ручки (4% ширины трека), чтобы примерно центрировать
    (fraction * 100.0 - 2.0).clamp(0.0, 96.0)
}
