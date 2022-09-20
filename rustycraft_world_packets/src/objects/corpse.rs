use deku::prelude::*;

use crate::guid::Guid;
use crate::objects::base::{BaseObject, Storage};
use crate::objects::{InnerState, UpdateMaskObjectType};

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Corpse {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: InnerState,
}
impl Corpse {
    pub fn new(guid: Guid) -> Box<Self> {
        let mut object = Self::default();
        <Self as BaseObject<0x0000>>::set_guid(&mut object, guid);
        <Self as BaseObject<0x0000>>::set_object_type(
            &mut object,
            UpdateMaskObjectType::Corpse as u32,
        );
        Box::new(object)
    }
}

enum CorpseFlags {
    CorpseFlagNone = 0x00,
    CorpseFlagBones = 0x01,
    CorpseFlagUnk1 = 0x02,
    CorpseFlagUnk2 = 0x04,
    CorpseFlagHideHelm = 0x08,
    CorpseFlagHideCloak = 0x10,
    CorpseFlagLootable = 0x20,
}

enum CorpseDynFlags {
    CorpseDynFlagLootable = 0x0001,
}

pub trait CorpseFields: BaseObject<0x0006> {
    fn set_corpse_owner(&mut self, corpse_owner: Guid) -> &mut Self {
        self.set_value(corpse_owner, 0x0000)
    }
    fn get_corpse_owner(&self) -> Option<Guid> {
        self.get_value(0x0000)
    }

    fn set_corpse_party(&mut self, corpse_party: Guid) -> &mut Self {
        self.set_value(corpse_party, 0x0002)
    }
    fn get_corpse_party(&self) -> Option<Guid> {
        self.get_value(0x0002)
    }

    fn set_corpse_display_id(&mut self, corpse_display_id: u32) -> &mut Self {
        self.set_value(corpse_display_id, 0x0004)
    }
    fn get_corpse_display_id(&self) -> Option<u32> {
        self.get_value(0x0004)
    }

    fn set_corpse_item(&mut self, corpse_item: u32, index: usize) -> &mut Self {
        assert!(index < 19, "Index is out of range");
        self.set_value(corpse_item, 0x0005 + index)
    }
    fn get_corpse_item(&self, index: u16) -> Option<u32> {
        assert!(index < 19, "Index is out of range");
        self.get_value(0x0005)
    }

    fn set_corpse_bytes_1(&mut self, corpse_bytes_1: [u8; 4]) -> &mut Self {
        self.set_value(corpse_bytes_1, 0x0018)
    }
    fn get_corpse_bytes_1(&self) -> Option<[u8; 4]> {
        self.get_value(0x0018)
    }

    fn set_corpse_bytes_2(&mut self, corpse_bytes_2: [u8; 4]) -> &mut Self {
        self.set_value(corpse_bytes_2, 0x0019)
    }
    fn get_corpse_bytes_2(&self) -> Option<[u8; 4]> {
        self.get_value(0x0019)
    }
    fn set_corpse_guild_id(&mut self, corpse_guild_id: u32) -> &mut Self {
        self.set_value(corpse_guild_id, 0x001A)
    }
    fn get_corpse_guild_id(&self) -> Option<u32> {
        self.get_value(0x001A)
    }
    fn set_corpse_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x001B, mask)
    }
    fn unset_corpse_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x001B, !(mask))
    }
    fn get_corpse_flags(&mut self) -> Option<u32> {
        self.get_value(0x001B)
    }
    fn set_corpse_dynamic_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x001C, mask)
    }
    fn unset_corpse_dynamic_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x001C, !(mask))
    }
    fn get_corpse_dynamic_flags(&mut self) -> Option<u32> {
        self.get_value(0x001C)
    }
}

impl CorpseFields for Corpse {}
impl Storage for Corpse {
    fn get_inner(&self) -> &InnerState {
        &self.inner
    }
    fn get_inner_mut(&mut self) -> &mut InnerState {
        &mut self.inner
    }
}
