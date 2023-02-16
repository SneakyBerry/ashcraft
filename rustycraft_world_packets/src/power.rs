use deku::prelude::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuWrite, DekuRead)]
#[deku(type = "u8")]
pub enum Power {
    #[default]
    Mana = 0x0,
    Rage = 0x1,
    Focus = 0x2,
    Energy = 0x3,
    Happiness = 0x4,
    Rune = 0x5,
    RunicPower = 0x6,
}
