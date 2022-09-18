use deku::prelude::*;

#[derive(Debug, Clone, DekuWrite, DekuRead)]
#[deku(type = "u8")]
pub enum Race {
    Human = 0x01,
    Orc = 0x02,
    Dwarf = 0x03,
    NightElf = 0x04,
    Undead = 0x05,
    Tauren = 0x06,
    Gnome = 0x07,
    Troll = 0x08,
    Goblin = 0x09,
    BloodElf = 0x0A,
    Draenei = 0x0B,
    FelOrc = 0x0C,
    Naga = 0x0D,
    Broken = 0x0E,
    Skeleton = 0x0F,
    Vrykul = 0x10,
    Tuskarr = 0x11,
    ForestTroll = 0x12,
    Taunka = 0x13,
    NorthrendSkeleton = 0x14,
    IceTroll = 0x15,
}
