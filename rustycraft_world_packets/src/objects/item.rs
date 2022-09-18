#[derive(Debug, Clone, Copy, Default)]
pub struct ItemEnchantment {
    pub id: u32,
    pub duration: u32,
    pub charges: u32,
}
pub enum EnchantmentSlot {
    PermEnchantmentSlot = 0,
    TempEnchantmentSlot = 1,
    SockEnchantmentSlot = 2,
    SockEnchantmentSlot2 = 3,
    SockEnchantmentSlot3 = 4,
    BonusEnchantmentSlot = 5,
    PrismaticEnchantmentSlot = 6, // added at apply special permanent enchantment

    PropEnchantmentSlot0 = 7,  // used with RandomSuffix
    PropEnchantmentSlot1 = 8,  // used with RandomSuffix
    PropEnchantmentSlot2 = 9,  // used with RandomSuffix and RandomProperty
    PropEnchantmentSlot3 = 10, // used with RandomProperty
    PropEnchantmentSlot4 = 11, // used with RandomProperty
    MaxEnchantmentSlot = 12,
}
pub mod item {
    #[macro_export]
    macro_rules! item_fields {
        (
            impl Item for $struct_name:ident
        ) => {
            impl_accessors!(
                Offset: 0x0006;
                Size: 0x003A;
                impl $struct_name {
                    0x0000 => item_owner: Guid;
                    0x0002 => item_contained: Guid;
                    0x0004 => item_creator: Guid;
                    0x0006 => item_gift_creator: Guid;
                    0x0008 => item_stack_count: u32;
                    0x0009 => item_duration: u32;
                    0x000A => item_spell_charges: [i32; 5];
                    0x000F => item_flags: [bool; 1];
                    0x0010 => item_enchantments: [ItemEnchantment; 12];
                    0x0034 => item_property_seed: u32;
                    0x0035 => item_random_properties_id: i32;
                    0x0036 => item_durability: u32;
                    0x0037 => item_max_durability: u32;
                    0x0038 => item_create_played_time: u32;
                    0x0039 => item_padding: ();
                }
            );
        };
    }
}
