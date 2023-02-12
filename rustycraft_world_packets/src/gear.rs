use crate::inventory::InventoryType;
use deku::prelude::*;

#[derive(Debug, Clone, Copy, DekuWrite, DekuRead, Valuable, Builder)]
pub struct CharacterGear {
    pub equipment_display_id: u32,
    pub inventory_type: InventoryType,
    pub enchantment: u32,
}
