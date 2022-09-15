use crate::packets::inventory::InventoryType;
use deku::prelude::*;

#[derive(Debug, Copy, Clone, DekuWrite, DekuRead)]
pub struct CharacterGear {
    #[deku(endian = "little")]
    pub equipment_display_id: u32,
    pub inventory_type: InventoryType,
    #[deku(endian = "little")]
    pub enchantment: u32,
}
