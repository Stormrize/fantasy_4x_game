#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum FantasyRace {
    #[default]
    FireMages, 
    StoneMages,
    WaterMages,
}