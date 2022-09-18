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
    CorpseDynflagLootable = 0x0001,
}

pub mod corpse {
    #[macro_export]
    macro_rules! corpse_fields {
        (
            impl Corpse for $struct_name:ident
        ) => {
            impl_accessors!(
                Offset: 0x0006;
                Size: 0x001E;
                impl $struct_name {
                    0x0000 => corpse_owner: Guid;
                    0x0002 => corpse_party: Guid;
                    0x0004 => corpse_display_id: u32;
                    0x0005 => corpse_item: [u32; 19];
                    0x0018 => corpse_bytes_1: u32;
                    0x0019 => corpse_bytes_2: u32;
                    0x001A => corpse_guild_id: u32;
                    0x001B => corpse_flags: [bool; 1];
                    0x001C => corpse_dynamic_flags: [bool; 1];
                    0x001D => corpse_padding: ();
                }
            );
        };
    }
}
