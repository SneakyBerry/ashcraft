use crate::objects::size_helper::FieldSize;
use deku::prelude::*;
use rustycraft_derive::IntoUpdateFields;

use crate::guid::Guid;
use crate::objects::object::Object;
use crate::objects::UpdateFields;

#[derive(Debug, Default, Clone, IntoUpdateFields, Builder)]
#[meta(offset = 0x0006, tag = 0x0003)]
pub struct Item {
    #[nested]
    pub object: Object,
    pub owner: Option<Guid>,
    pub contained: Option<Guid>,
    pub creator: Option<Guid>,
    pub gift_creator: Option<Guid>,
    pub stack_count: Option<u32>,
    pub duration: Option<u32>,
    pub spell_charges: [Option<i32>; 5],
    pub flags: Option<u32>,
    pub enchantment: [Option<ItemEnchantment>; 12],
    pub property_seed: Option<u32>,
    pub random_properties_id: Option<i32>,
    pub durability: Option<u32>,
    pub max_durability: Option<u32>,
    pub create_played_time: Option<u32>,
}

#[derive(Debug, Clone, Copy, Default, DekuRead, DekuWrite)]
pub struct ItemEnchantment {
    pub id: u32,
    pub duration: u32,
    pub charges: u32,
}
pub enum EnchantmentSlot {
    Perm = 0,
    Temp = 1,
    Sock = 2,
    Sock2 = 3,
    Sock3 = 4,
    Bonus = 5,
    Prismatic = 6, // added at apply special permanent enchantment

    Prop0 = 7,  // used with RandomSuffix
    Prop1 = 8,  // used with RandomSuffix
    Prop2 = 9,  // used with RandomSuffix and RandomProperty
    Prop3 = 10, // used with RandomProperty
    Prop4 = 11, // used with RandomProperty
    Max = 12,
}
