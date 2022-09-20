use crate::guid::Guid;
use crate::objects::private;

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

pub trait Corpse: private::Object<0x0006> {
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
