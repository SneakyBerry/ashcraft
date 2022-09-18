// use crate::guid::Guid;
// use crate::objects::{ObjectInner, UpdateObject};
// use crate::BitVec;
use deku::prelude::*;
// use crate::objects::traits::sealed::Sealed;

#[derive(Debug, Clone)]
pub struct Bytes1 {
    pub skin: u8,
    pub face: u8,
    pub hairstyle: u8,
    pub hair_color: u8,
}

#[derive(Debug, Clone)]
pub struct Bytes2 {
    pub facial: u8,
    pub party: u8,
    pub bank_bag_slots: u8,
    pub rest_state: u8,
}

#[derive(Debug, Clone)]
pub struct Bytes3 {
    pub gender: u8,
    pub inebriation: u8,
    pub pvp_title: u8,
    pub arena_faction: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct QuestLogItem {
    pub id: u32,
    pub state: u32,
    pub counts1: u16,
    pub counts2: u16,
    pub time: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerEnchantment {
    pub id: u32,
    pub permanent: u16,
    pub temporary: u16,
}

#[derive(Debug, Clone)]
pub struct Bytes4 {
    pub flags: u8,
    pub raf_grantable_level: u8,
    pub action_bar_toggles: u8,
    pub lifetime_max_pvp_rank: u8,
}


#[derive(Debug, Clone, Default)]
pub struct PlayerFieldBytes2Offsets {
    pub override_spells_id: u16,
    pub ignore_power_regen_prediction_mask: u8,
    pub aura_vision: u8,
}

pub enum PlayerFlags {
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

pub enum EquipmentSlots
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


pub struct Rune {
    pub blood: u32,
    pub unholy: u32,
    pub frost: u32,
    pub death: u32,
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
                0x00AF => player_field_pad_0: ();

                0x00B0 => player_field_inv_slot_head: [Guid; 23];
                0x00DE => player_field_pack_slot: [Guid; 16];
                0x00FE => player_field_bank_slot: [Guid; 29];
                0x0136 => player_field_bankbag_slot: [Guid; 7];
                0x0144 => player_field_vendorbuyback_slot: [Guid; 24];
                0x015C => player_field_keyring_slot: [Guid; 32];
                0x019C => player_field_currencytoken_slot: [Guid; 64];
                0x01DC => player_farsight: Guid;
                0x01DE => player_field_known_titles: [u64; 3];
                0x01E4 => player_field_known_currencies: u64;
                0x01E6 => player_xp: u32;
                0x01E7 => player_next_level_xp: u32;
                0x01E8 => player_skills_info: [(u16, u16); 384];
                0x0368 => player_character_points: [u32; 2];
                //
                0x036A => player_track_creatures: bool;
                0x036B => player_track_resources: bool;
                //
                0x036C => player_block_percentage: f32;
                0x036D => player_dodge_percentage: f32;
                0x036E => player_parry_percentage: f32;
                //
                0x036F => player_expertise: u32;
                0x0370 => player_offhand_expertise: u32;
                //
                0x0371 => player_crit_percentage: f32;
                0x0372 => player_ranged_crit_percentage: f32;
                0x0373 => player_offhand_crit_percentage: f32;
                0x0374 => player_spell_crit_percentage: [f32; 7];
                0x037B => player_shield_block: u32;
                0x037C => player_shield_block_crit_percentage: f32;
                //
                0x037D => player_explored_zones: [bool; 128];
                //
                0x03FD => player_rest_state_experience: u32;
                0x03FE => player_field_coinage: u32;
                0x03FF => player_field_mod_damage_done_pos: [u32; 7]; // TODO: map it
                0x0406 => player_field_mod_damage_done_neg: [u32; 7];
                0x040D => player_field_mod_damage_done_pct: [u32; 7];
                0x0414 => player_field_mod_healing_done_pos: f32;
                0x0415 => player_field_mod_healing_pct: u32;
                0x0416 => player_field_mod_healing_done_pct: f32;
                0x0417 => player_field_mod_target_resistance: u32;
                0x0418 => player_field_mod_target_physical_resistance: u32;
                //
                0x0419 => player_field_bytes: Bytes4;
                //
                0x041A => player_ammo_id: u32;
                0x041B => player_self_res_spell: u32;
                0x041C => player_field_pvp_medals: u32;
                0x041D => player_field_buyback_price: [u32; 12];
                0x0429 => player_field_buyback_timestamp: [u32; 12];
                0x0435 => player_field_kills: u32;
                0x0436 => player_field_today_contribution: u32;
                0x0437 => player_field_yesterday_contribution: u32;
                0x0438 => player_field_lifetime_honorable_kills: u32;
                //
                0x0439 => player_field_bytes2: PlayerFieldBytes2Offsets;
                //
                0x043A => player_field_watched_faction_index: u32;
                0x043B => player_field_combat_rating: [u32; 25];
                0x0454 => player_field_arena_team_info: [u32; 21];
                0x0469 => player_field_honor_currency: u32;
                0x046A => player_field_arena_currency: u32;
                0x046B => player_field_max_level: u32;
                0x046C => player_field_daily_quests: [u32; 25];
                0x0485 => player_rune_regen: Rune;
                0x0489 => player_no_reagent_cost: [u32; 3];
                0x048C => player_field_glyph_slots: [u32; 6];
                0x0492 => player_field_glyphs: [u32; 6];
                0x0498 => player_glyphs_enabled: u32;
                0x0499 => player_pet_spell_power: u32;
            }
        );
    };
}
