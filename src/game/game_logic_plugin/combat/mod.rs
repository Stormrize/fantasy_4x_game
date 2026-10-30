//! Боевая система: модель, симуляция, боевой ИИ и интеграция с `SimulationTime`.

pub mod ai;
pub mod integration;
pub mod model;
pub mod simulation;

pub use integration::CombatPlugin;
