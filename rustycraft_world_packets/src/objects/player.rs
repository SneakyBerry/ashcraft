use crate::common::helpers::ArrayWrapped;
use crate::common::school::SchoolIndexed;
use deku::prelude::*;
use rustycraft_derive::CalcUpdate;

use crate::define_flags;
use crate::guid::Guid;
use crate::objects::unit::UnitUpdate;

/* TODO: Think about how to reduce the size with same convenience as struct
   Sparse struct? Builder pattern?
*/
#[derive(Debug, Default, Clone, CalcUpdate, Builder)]
#[meta(offset = 0x0094, tag = 0x0019)]
pub struct PlayerUpdate {
    #[nested]
    pub unit: UnitUpdate,
    pub duel_arbiter: Guid,
    pub flags: PlayerFlags,
    pub guild_id: u32,
    pub guild_rank: u32,
    pub bytes: CharacterStyle1,
    pub bytes_2: CharacterStyle2,
    pub bytes_3: Bytes3,
    pub duel_team: u32,
    pub guild_timestamp: u32,
    pub player_quests: ArrayWrapped<QuestLogItem, 25>,
    pub visible_items: ArrayWrapped<EquipedItem, 19>,
    pub chosen_title: u32,
    pub fake_inebriation: i32,
    pub unknown_af: i32,
    pub equipped_items: ArrayWrapped<Guid, 23>,
    pub pack_slot: ArrayWrapped<Guid, 16>,
    pub bank_slot: ArrayWrapped<Guid, 28>,
    pub bank_bag_slot: ArrayWrapped<Guid, 7>,
    pub vendor_buyback_slot: ArrayWrapped<Guid, 12>,
    pub keyring_slot: ArrayWrapped<Guid, 32>,
    pub currency_token_slot: ArrayWrapped<Guid, 32>,
    pub far_sight: Guid,
    pub known_titles: ArrayWrapped<u64, 3>,
    pub known_currencies: u64,
    pub xp: u32,
    pub next_level_xp: u32,
    pub skill_info: ArrayWrapped<(u16, u16), 384>,
    pub character_points: ArrayWrapped<u32, 2>,
    pub track_creatures: u32,
    pub track_resources: u32,
    pub block_percentage: f32,
    pub dodge_percentage: f32,
    pub parry_percentage: f32,
    pub expertise: u32,
    pub offhand_expertise: u32,
    pub crit_percentage: f32,
    pub ranged_crit_percentage: f32,
    pub offhand_crit_percentage: f32,
    pub spell_crit_percentage: SchoolIndexed<f32>,
    pub shield_block: u32,
    pub shield_block_crit_percentage: f32,
    pub explored_zones: ArrayWrapped<u32, 128>,
    pub rest_state_experience: u32,
    pub coinage: u32,
    pub mod_damage_done_pos: SchoolIndexed<u32>,
    pub mod_damage_done_neg: SchoolIndexed<u32>,
    pub mod_damage_done_pct: SchoolIndexed<u32>,
    pub mod_healing_done_pos: f32,
    pub mod_healing_pct: u32,
    pub mod_healing_done_pct: f32,
    pub mod_target_resistance: u32,
    pub mod_target_physical_resistance: u32,
    pub bytes_4: Bytes4,
    pub ammo_id: u32,
    pub self_res_spell: u32,
    pub pvp_medals: u32,
    pub buyback_price: ArrayWrapped<u32, 12>,
    pub buyback_timestamp: ArrayWrapped<u32, 12>,
    pub kills: u32,
    pub today_contribution: u32,
    pub yesterday_contribution: u32,
    pub lifetime_honorable_kills: u32,
    pub bytes2: PlayerFieldBytes2Offsets,
    pub watched_faction_index: u32,
    pub combat_rating: ArrayWrapped<u32, 25>,
    pub arena_team_info: ArrayWrapped<u32, 21>,
    pub honor_currency: u32,
    pub arena_currency: u32,
    pub max_level: u32,
    pub daily_quests: ArrayWrapped<u32, 25>,
    pub rune_regen: Rune,
    pub no_reagent_cost: ArrayWrapped<u32, 3>,
    pub glyph_slots: ArrayWrapped<u32, 6>,
    pub glyphs: ArrayWrapped<u32, 6>,
    pub glyphs_enabled: u32,
    pub pet_spell_power: u32,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct CharacterStyle1 {
    pub skin: u8,
    pub face: u8,
    pub hairstyle: u8,
    pub hair_color: u8,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct CharacterStyle2 {
    pub facial: u8,
    pub party: u8,
    pub bank_bag_slots: u8,
    pub rest_state: u8,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct Bytes3 {
    pub gender: u8,
    pub inebriation: u8,
    pub pvp_title: u8,
    pub arena_faction: u8,
}

#[derive(Debug, Default, Clone, Copy, DekuRead, DekuWrite)]
pub struct QuestLogItem {
    pub id: u32,
    pub state: u32,
    pub counts1: u16,
    pub counts2: u16,
    pub counts3: u16,
    pub counts4: u16,
    pub time: u32,
}

#[derive(Debug, Default, Clone, Copy, DekuRead, DekuWrite)]
pub struct EquipedItem {
    pub id: u32,
    pub permanent: u16,
    pub temporary: u16,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct Bytes4 {
    pub flags: u8,
    pub raf_grantable_level: u8,
    pub action_bar_toggles: u8,
    pub lifetime_max_pvp_rank: u8,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct PlayerFieldBytes2Offsets {
    pub override_spells_id: u16,
    pub ignore_power_regen_prediction_mask: u8,
    pub aura_vision: u8,
}

define_flags!(
    StructName: PlayerFlags
    InnerType: u32{
        GROUP_LEADER      = 0x00000001,
        AFK               = 0x00000002,
        DND               = 0x00000004,
        GM                = 0x00000008,
        GHOST             = 0x00000010,
        RESTING           = 0x00000020,
        UNK6              = 0x00000040,
        UNK7              = 0x00000080,               // pre-3.0.3 FFA_PVP flag for FFA PVP state
        CONTESTED_PVP     = 0x00000100,               // Player has been involved in a PvP combat and will be attacked by contested guards
        IN_PVP            = 0x00000200,
        HIDE_HELM         = 0x00000400,
        HIDE_CLOAK        = 0x00000800,
        PLAYED_LONG_TIME  = 0x00001000,               // played long time
        PLAYED_TOO_LONG   = 0x00002000,               // played too long time
        IS_OUT_OF_BOUNDS  = 0x00004000,
        DEVELOPER         = 0x00008000,               // <Dev> prefix for something?
        UNK16             = 0x00010000,               // pre-3.0.3 SANCTUARY flag for player entered sanctuary
        TAXI_BENCHMARK    = 0x00020000,               // taxi benchmark mode (on/off) (2.0.1)
        PVP_TIMER         = 0x00040000,               // 3.0.2, pvp timer active (after you disable pvp manually)
        UBER              = 0x00080000,
        UNK20             = 0x00100000,
        UNK21             = 0x00200000,
        COMMENTATOR2      = 0x00400000,
        ALLOW_ONLY_ABILITY      = 0x00800000,                // used by bladestorm and killing spree, allowed only spells with SPELL_ATTR0_REQ_AMMO, SPELL_EFFECT_ATTACK, checked only for active player
        UNK24             = 0x01000000,                // disabled all melee ability on tab include autoattack
        NO_XP_GAIN        = 0x02000000,
        UNK26             = 0x04000000,
        UNK27             = 0x08000000,
        UNK28             = 0x10000000,
        UNK29             = 0x20000000,
        UNK30             = 0x40000000,
        UNK31             = 0x80000000
    }
);

#[derive(Debug, Clone, Copy)]
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
    MainHand = 15,
    OffHand = 16,
    Ranged = 17,
    Tabard = 18,
    End = 19,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct Rune {
    pub blood: u32,
    pub unholy: u32,
    pub frost: u32,
    pub death: u32,
}
