use deku::prelude::*;

#[derive(Debug, Copy, Clone, DekuWrite, DekuRead, Valuable)]
#[deku(type = "u8")]
pub enum InventoryType {
    NonEquip = 0x0,
    Head = 0x1,
    NeckOrRelic = 0x2,
    Shoulders = 0x3,
    Body = 0x4,
    Chest = 0x5,
    Waist = 0x6,
    Legs = 0x7,
    Feet = 0x8,
    Wrists = 0x9,
    Hands = 0xa,
    Finger = 0xb,
    Trinket = 0xc,
    Weapon = 0xd,
    Shield = 0xe,
    Ranged = 0xf,
    Cloak = 0x10,
    TwoHandedWeapon = 0x11,
    Bag = 0x12,
    Tabard = 0x13,
    Robe = 0x14,
    WeaponMainHand = 0x15,
    WeaponOffHand = 0x16,
    Holdable = 0x17,
    Ammo = 0x18,
    Thrown = 0x19,
    RangedRight = 0x1a,
    Quiver = 0x1b,
}
