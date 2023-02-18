use crate::inventory::InventoryTypeU8;
use deku::prelude::*;

#[derive(Debug, Clone, Copy, DekuWrite, DekuRead, Builder)]
pub struct CharacterGear {
    pub equipment_display_id: u32,
    pub inventory_type: InventoryTypeU8,
    pub enchantment: u32,
}
