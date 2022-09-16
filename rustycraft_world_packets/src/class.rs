use deku::prelude::*;

#[derive(Debug, DekuWrite, DekuRead)]
#[deku(type = "u8")]
pub enum Class {
    Warrior = 0x01,
    Paladin = 0x02,
    Hunter = 0x03,
    Rogue = 0x04,
    Priest = 0x05,
    DeathKnight = 0x06,
    Shaman = 0x07,
    Mage = 0x08,
    Warlock = 0x09,
    Druid = 0x0A,
}
