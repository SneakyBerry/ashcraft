use crate::common::helpers::ArrayWrapped;
use crate::guid::Guid;
use crate::objects::object::ObjectUpdate;
use ashcraft_derive::CalcUpdate;

#[derive(Debug, Default, Clone, CalcUpdate, Builder)]
#[meta(offset = 0x0006, tag = 0x0081)]
pub struct CorpseUpdate {
    #[nested]
    pub object: ObjectUpdate,
    pub owner: Guid,
    pub party: Guid,
    pub display_id: u32,
    pub item: ArrayWrapped<u32, 19>,
    pub bytes_1: [u8; 4],
    pub bytes_2: [u8; 4],
    pub guild_id: u32,
    pub flags: u32,
    pub dynamic_flags: u32,
}

pub enum CorpseFlags {
    CorpseFlagNone = 0x00,
    CorpseFlagBones = 0x01,
    CorpseFlagUnk1 = 0x02,
    CorpseFlagUnk2 = 0x04,
    CorpseFlagHideHelm = 0x08,
    CorpseFlagHideCloak = 0x10,
    CorpseFlagLootable = 0x20,
}

pub enum CorpseDynFlags {
    CorpseDynFlagLootable = 0x0001,
}
