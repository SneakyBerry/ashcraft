use crate::guid::Guid;
use crate::objects::size_helper::FieldSize;
use crate::objects::object::Object;
use crate::objects::UpdateFields;
use rustycraft_derive::IntoUpdateFields;

#[derive(Debug, Default, Clone, IntoUpdateFields, Builder)]
#[meta(offset = 0x0006, tag = 0x0081)]
pub struct Corpse {
    #[nested]
    pub object: Object,
    pub owner: Option<Guid>,
    pub party: Option<Guid>,
    pub display_id: Option<u32>,
    pub item: [Option<u32>; 19],
    pub bytes_1: Option<[u8; 4]>,
    pub bytes_2: Option<[u8; 4]>,
    pub guild_id: Option<u32>,
    pub flags: Option<u32>,
    pub dynamic_flags: Option<u32>,
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
