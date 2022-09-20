use deku::prelude::*;

use crate::guid::Guid;
use crate::object::ObjectType;
use crate::objects::base::{BaseObject, Storage};
use crate::objects::container::ContainerFields;
use crate::objects::object::ObjectFields;
use crate::objects::{InnerState, UpdateMaskObjectType};

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Item {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: InnerState,
}
impl Item {
    pub fn new(guid: Guid) -> Box<Self> {
        let mut object = Self::default();
        <Self as BaseObject<0x0000>>::set_guid(&mut object, guid);
        <Self as BaseObject<0x0000>>::set_object_type(
            &mut object,
            UpdateMaskObjectType::Item as u32,
        );
        Box::new(object)
    }
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

impl<T> ItemFields for T where T: BaseObject<0x0006> + ContainerFields {}
pub trait ItemFields: BaseObject<0x0006> {
    fn set_owner(&mut self, owner: Guid) -> &mut Self {
        self.set_value(owner, 0x0000)
    }
    fn get_owner(&self) -> Option<Guid> {
        self.get_value(0x0000)
    }
    fn set_contained(&mut self, contained: Guid) -> &mut Self {
        self.set_value(contained, 0x0002)
    }
    fn get_contained(&self) -> Option<Guid> {
        self.get_value(0x0002)
    }
    fn set_creator(&mut self, creator: Guid) -> &mut Self {
        self.set_value(creator, 0x0004)
    }
    fn get_creator(&self) -> Option<Guid> {
        self.get_value(0x0004)
    }
    fn set_gift_creator(&mut self, gift_creator: Guid) -> &mut Self {
        self.set_value(gift_creator, 0x0006)
    }
    fn get_gift_creator(&self) -> Option<Guid> {
        self.get_value(0x0006)
    }
    fn set_stack_count(&mut self, stack_count: u32) -> &mut Self {
        self.set_value(stack_count, 0x0008)
    }
    fn get_stack_count(&self) -> Option<u32> {
        self.get_value(0x0008)
    }
    fn set_duration(&mut self, duration: u32) -> &mut Self {
        self.set_value(duration, 0x0009)
    }
    fn get_duration(&self) -> Option<u32> {
        self.get_value(0x0009)
    }
    fn set_spell_charge(&mut self, spell_charge: i32, index: usize) -> &mut Self {
        if index >= 5 {
            panic!("Index is out of range")
        }
        self.set_value(spell_charge, 0x000A + index)
    }
    fn get_spell_charge(&self, index: usize) -> Option<i32> {
        self.get_value(0x000A + index)
    }
    fn set_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x000F, mask)
    }
    fn unset_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x000F, !(mask))
    }
    fn get_flags(&mut self) -> Option<u32> {
        self.get_value(0x000F)
    }
    fn set_enchantment(&mut self, enchantments: ItemEnchantment, index: usize) -> &mut Self {
        if index >= 12 {
            panic!("Index is out of range")
        }
        self.set_value(enchantments, 0x0010 + 3 * index)
    }
    fn get_enchantment(&self, index: usize) -> Option<ItemEnchantment> {
        if index >= 12 {
            panic!("Index is out of range")
        }
        self.get_value(0x0010 + 3 * index)
    }
    fn set_property_seed(&mut self, property_seed: u32) -> &mut Self {
        self.set_value(property_seed, 0x0034)
    }
    fn get_property_seed(&self) -> Option<u32> {
        self.get_value(0x0034)
    }
    fn set_random_properties_id(&mut self, random_properties_id: i32) -> &mut Self {
        self.set_value(random_properties_id, 0x0035)
    }
    fn get_random_properties_id(&self) -> Option<i32> {
        self.get_value(0x0035)
    }
    fn set_durability(&mut self, durability: u32) -> &mut Self {
        self.set_value(durability, 0x0036)
    }
    fn get_durability(&self) -> Option<u32> {
        self.get_value(0x0036)
    }
    fn set_max_durability(&mut self, max_durability: u32) -> &mut Self {
        self.set_value(max_durability, 0x0037)
    }
    fn get_max_durability(&self) -> Option<u32> {
        self.get_value(0x0037)
    }
    fn set_create_played_time(&mut self, create_played_time: u32) -> &mut Self {
        self.set_value(create_played_time, 0x0038)
    }
    fn get_create_played_time(&self) -> Option<u32> {
        self.get_value(0x0038)
    }
}

impl ItemFields for Item {}
impl Storage for Item {
    fn get_inner(&self) -> &InnerState {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut InnerState {
        &mut self.inner
    }
}
