// use crate::guid::Guid;
// use crate::objects::{ObjectInner, UpdateObject};
// use crate::BitVec;
use deku::prelude::*;
// use crate::objects::traits::sealed::Sealed;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes1 {
    skin: u8,
    face: u8,
    hairstyle: u8,
    hair_color: u8,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes2 {
    facial: u8,
    party: u8,
    bank_bag_slots: u8,
    rest_state: u8,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes3 {
    gender: u8,
    inebriation: u8,
    pvp_title: u8,
    arena_faction: u8,
}

#[derive(Debug, Clone, Copy, DekuRead, DekuWrite)]
pub struct QuestLogItem {
    #[deku(endian = "little")]
    id: u32,
    #[deku(endian = "little")]
    state: u32,
    #[deku(endian = "little")]
    counts1: u16,
    #[deku(endian = "little")]
    counts2: u16,
    #[deku(endian = "little")]
    time: u32,
}

#[derive(Debug, Clone, Copy, DekuRead, DekuWrite)]
pub struct PlayerEnchantment {
    #[deku(endian = "little")]
    id: u32,
    #[deku(endian = "little")]
    permanent: u16,
    #[deku(endian = "little")]
    temporary: u16,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes4 {
    flags: u8,
    raf_grantable_level: u8,
    action_bar_toggles: u8,
    lifetime_max_pvp_rank: u8,
}


#[derive(Debug, Clone, Default, DekuRead, DekuWrite)]
pub struct PlayerFieldBytes2Offsets {
    #[deku(endian = "little")]
    override_spells_id: u16,
    ignore_power_regen_prediction_mask: u8,
    aura_vision: u8,
}

enum PlayerFlags {
    GroupLeader = 0x00000001,
    Afk = 0x00000002,
    Dnd = 0x00000004,
    Gm = 0x00000008,
    Ghost = 0x00000010,
    Resting = 0x00000020,
    Unk6 = 0x00000040,
    Unk7 = 0x00000080,         // pre-3.0.3 FLAGS_FFA_PVP flag for FFA PVP state
    ContestedPvp = 0x00000100, // Player has been involved in a PvP combat and will be attacked by contested guards
    InPvp = 0x00000200,
    HideHelm = 0x00000400,
    HideCloak = 0x00000800,
    PlayedLongTime = 0x00001000, // played long time
    PlayedTooLong = 0x00002000,  // played too long time
    IsOutOfBounds = 0x00004000,
    Developer = 0x00008000,     // <Dev> prefix for something?
    Unk16 = 0x00010000,         // pre-3.0.3 FLAGS_SANCTUARY flag for player entered sanctuary
    TaxiBenchmark = 0x00020000, // taxi benchmark mode (on/off) (2.0.1)
    PvpTimer = 0x00040000,      // 3.0.2, pvp timer active (after you disable pvp manually)
    Uber = 0x00080000,
    Unk20 = 0x00100000,
    Unk21 = 0x00200000,
    Commentator2 = 0x00400000,
    PlayerAllowOnlyAbility = 0x00800000, // used by bladestorm and killing spree, allowed only spells with SPELL_ATTR0_REQ_AMMO, SPELL_EFFECT_ATTACK, checked only for active player
    Unk24 = 0x01000000,                  // disabled all melee ability on tab include autoattack
    NoXpGain = 0x02000000,
    Unk26 = 0x04000000,
    Unk27 = 0x08000000,
    Unk28 = 0x10000000,
    Unk29 = 0x20000000,
    Unk30 = 0x40000000,
    Unk31 = 0x80000000,
}

enum EquipmentSlots
// 19 slots
{
    Head = 0,
    Neck = 1,
    Shoulders = 2,
    Body = 3,
    Chest = 4,
    Waist = 5,
    Legs = 6,
    Feet = 7,
    Wrists = 8,
    Hands = 9,
    Finger1 = 10,
    Finger2 = 11,
    Trinket1 = 12,
    Trinket2 = 13,
    Back = 14,
    Mainhand = 15,
    Offhand = 16,
    Ranged = 17,
    Tabard = 18,
    End = 19,
}


pub struct VisibleItem {
    pub entry_id: u32,
    pub enchantment: PlayerEnchantment
}


#[macro_export]
macro_rules! player_fields {
    (
        Offset: $offset:tt;
        impl Player for $struct_name:ident
    ) => {
        impl_accessors!(
            Offset: $offset;
            Size: 0x049A;
            impl $struct_name {
                0x0000 => player_duel_arbiter: Guid;
                0x0002 => player_flags: u32;
                0x0003 => player_guild_id: u32;
                0x0004 => player_guild_rank: u32;
                0x0005 => player_bytes: Bytes1;
                0x0006 => player_bytes_2: Bytes2;
                0x0007 => player_bytes_3: Bytes3;
                0x0008 => player_duel_team: u32;
                0x0009 => player_guild_timestamp: u32;
                0x000A => player_quests: [QuestLogItem; 24];
                0x0087 => player_visible_items: [VisibleItem; 18];
                0x00AD => player_chosen_title: u32;
                0x00AE => player_fake_inebriation: i32;
                0x00AF => player_field_pad_0: None;

                0x00B0 => player_field_inv_slot_head: [Guid; 23];
                0x0000 => player_field_pack_slot: [Guid; 16];
                0x0000 => player_field_bank_slot: [Guid; 29];
                0x0000 => player_field_bankbag_slot: [Guid; 7];
                0x0000 => player_field_vendorbuyback_slot: [Guid; 24];
                0x0000 => player_field_keyring_slot: [Guid; 32];
                0x0000 => player_field_currencytoken_slot: [Guid; 64];
                0x0000 => player_farsight: Guid;
                0x0000 => player_field_known_titles: [u64; 3];
                0x0000 => player_field_known_currencies: u64;
                0x0000 => player_xp: u32;
                0x0000 => player_next_level_xp: u32;
                0x0000 => player_skills_info: [(u16, u16); 384];
                0x0000 => player_character_points: [u32; 32];
                //
                0x0000 => player_track_creatures: bitmask;
                0x0000 => player_track_resources: bitmask;
                //
                0x0000 => player_block_percentage: f32;
                0x0000 => player_dodge_percentage: f32;
                0x0000 => player_parry_percentage: f32;
                //
                0x0000 => player_expertise: u32;
                0x0000 => player_offhand_expertise: u32;
                //
                0x0000 => player_crit_percentage: f32;
                0x0000 => player_ranged_crit_percentage: f32;
                0x0000 => player_offhand_crit_percentage: f32;
                0x0000 => player_spell_crit_percentage: [f32; 7];
                0x0000 => player_shield_block: u32;
                0x0000 => player_shield_block_crit_percentage: f32;
                //
                0x0000 => player_explored_zones: [bit; 128];
                //
                0x0000 => player_rest_state_experience: u32;
                0x0000 => player_field_coinage: u32;
                0x0000 => player_field_mod_damage_done_pos: [u32; 7]; // TODO: map it
                0x0000 => player_field_mod_damage_done_neg: [u32; 7];
                0x0000 => player_field_mod_damage_done_pct: [u32; 7];
                0x0000 => player_field_mod_healing_done_pos: f32;
                0x0000 => player_field_mod_healing_pct: u32;
                0x0000 => player_field_mod_healing_done_pct: f32;
                0x0000 => player_field_mod_target_resistance: u32;
                0x0000 => player_field_mod_target_physical_resistance: u32;
                //
                0x0000 => player_field_bytes: Bytes4;
                //
                0x0000 => player_ammo_id: u32;
                0x0000 => player_self_res_spell: u32;
                0x0000 => player_field_pvp_medals: u32;
                0x0000 => player_field_buyback_price: [u32; 12];
                0x0000 => player_field_buyback_timestamp: [u32; 12];
                0x0000 => player_field_kills: u32;
                0x0000 => player_field_today_contribution: u32;
                0x0000 => player_field_yesterday_contribution: u32;
                0x0000 => player_field_lifetime_honorable_kills: u32;
                //
                0x0000 => player_field_bytes2: PlayerFieldBytes2Offsets;
                //
                0x0000 => player_field_watched_faction_index: u32;
                0x0000 => player_field_combat_rating: [u32; 25];
                0x0000 => player_field_arena_team_info: [u32; 21];
                0x0000 => player_field_honor_currency: u32;
                0x0000 => player_field_arena_currency: u32;
                0x0000 => player_field_max_level: u32;
                0x0000 => player_field_daily_quests: [u32; 25];
                0x0000 => player_blood_rune_regen: f32;
                0x0000 => player_unholy_rune_regen: f32;
                0x0000 => player_frost_rune_regen: f32;
                0x0000 => player_death_rune_regen: f32;
                0x0000 => player_no_reagent_cost: [u32; 3];
                0x0000 => player_field_glyph_slots: [u32; 6];
                0x0000 => player_field_glyphs: [u32; 6];
                0x0000 => player_glyphs_enabled: u32;
                0x0000 => player_pet_spell_power: u32;
            }
        );
    };
}
