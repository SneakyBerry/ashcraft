use crate::guid::Guid;
use crate::objects::object::ObjectUpdate;
use deku::prelude::*;
use rustycraft_derive::CalcUpdate;
use crate::common::helpers::ArrayWrapped;

#[derive(Debug, Default, Clone, CalcUpdate, Builder)]
#[meta(offset = 0x0006, tag = 0x0003)]
pub struct ItemUpdate {
    #[nested]
    pub object: ObjectUpdate,
    pub owner: Guid,
    pub contained: Guid,
    pub creator: Guid,
    pub gift_creator: Guid,
    pub stack_count: u32,
    pub duration: u32,
    pub spell_charges: ArrayWrapped<i32, 5>,
    pub flags: u32,
    pub enchantment: ArrayWrapped<ItemEnchantment, 12>,
    pub property_seed: u32,
    pub random_properties_id: i32,
    pub durability: u32,
    pub max_durability: u32,
    pub create_played_time: u32,
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
