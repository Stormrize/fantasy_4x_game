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
    Beginner = 0,
    Master = 1,
    Unique = 2,
}