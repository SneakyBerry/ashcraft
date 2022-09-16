use deku::prelude::*;
use crate::setters;

setters!(
    pub struct 0x0080 => Corpse {
        0 => guid_low: u32,          // 0
        1 => guid_high: u32,         // 1
        2 => object_entry: i32,      // 3
        2 => object_scale_x: f32,    // 4
        2 => owner_guid_low: u32,    // 6
        2 => owner_guid_high: u32,   // 7
        2 => party_guid_low: u32,    // 8
        2 => party_guid_high: u32,   // 9
        2 => display_id: i32,        // 10
        2 => item: i32,              // 11
        2 => unk1: Unknown1,         // 30
        2 => unk2: Unknown2,         // 31
        2 => guild: i32,             // 32
        2 => flags: i32,             // 33
        2 => dynamic_flags: i32      // 34
    }
);

#[derive(Debug, Clone, DekuWrite)]
pub struct Unknown1 {
    unk1: u8,
    unk2: u8,
    unk3: u8,
    unk4: u8,
}

#[derive(Debug, Clone, DekuWrite)]
pub struct Unknown2 {
    unk1: u8,
    unk2: u8,
    unk3: u8,
    unk4: u8,
}
