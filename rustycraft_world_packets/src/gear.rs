use crate::inventory::InventoryType;
use deku::prelude::*;

#[derive(Debug, Copy, Clone, DekuWrite, DekuRead, Builder)]
pub struct CharacterGear {
    pub equipment_display_id: u32,
    pub inventory_type: InventoryType,
    pub enchantment: u32,
}
