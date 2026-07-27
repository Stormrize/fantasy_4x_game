#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Unit {
    WaterMage,
    FireMage,
    StoneMage,
    ResourceGatherer,
    #[default]
    None
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Level {
    #[default]
    None,
    Beginner,
    Master,
    Unique,
}