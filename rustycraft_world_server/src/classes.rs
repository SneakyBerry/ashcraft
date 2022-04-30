use deku::prelude::*;
use enum_iterator::IntoEnumIterator;


#[derive(Debug, Clone, DekuWrite, DekuRead, IntoEnumIterator)]
#[deku(type = "u8")]
#[repr(u8)]
pub enum Classes {
    Warrior = 1,
    Paladin = 2,
    Hunter = 3,
    Rogue = 4,
    Priest = 5,
    DeathKnight = 6,
    Shaman = 7,
    Mage = 8,
    Warlock = 9,
    Monk = 10,
    Druid = 11,
    DemonHunter = 12,
}