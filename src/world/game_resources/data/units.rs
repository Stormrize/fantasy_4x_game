
#[derive(Default, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Unit {
    //вода
    WaterMageBeginner(String),
    WaterMageMaster(String),

    //огонь
    FireMageBeginner(String),
    FireMageMaster(String),

    //камень
    StoneMageBeginner(String),
    StoneMageMaster(String),

    ResourceGatherer(String),
    #[default]
    None
}