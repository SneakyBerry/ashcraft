use deku::prelude::*;

use crate::guid::Guid;
use crate::objects::size_helper::FieldSize;
use crate::objects::unit::Unit;
use crate::objects::UpdateFields;
use rustycraft_derive::IntoUpdateFields;

#[derive(Debug, Clone, IntoUpdateFields, Builder)]
#[meta(offset = 0x0094, tag = 0x0019)]
pub struct Player {
    #[nested]
    pub unit: Unit,
    pub duel_arbiter: Option<Guid>,
    pub flags: Option<u32>,
    pub guild_id: Option<u32>,
    pub guild_rank: Option<u32>,
    pub bytes: Option<Bytes1>,
    pub bytes_2: Option<Bytes2>,
    pub bytes_3: Option<Bytes3>,
    pub duel_team: Option<u32>,
    pub guild_timestamp: Option<u32>,
    pub player_quests: [Option<QuestLogItem>; 25],
    pub visible_items: [Option<EquipedItem>; 19],
    pub chosen_title: Option<u32>,
    pub fake_inebriation: Option<i32>,
    pub unknown_af: Option<i32>,
    pub field_inv_slot_head: [Option<Guid>; 23],
    pub field_pack_slot: [Option<Guid>; 16],
    pub field_bank_slot: [Option<Guid>; 28],
    pub field_bank_bag_slot: [Option<Guid>; 7],
    pub field_vendor_buyback_slot: [Option<Guid>; 12],
    pub field_keyring_slot: [Option<Guid>; 32],
    pub field_currency_token_slot: [Option<Guid>; 32],
    pub farsight: Option<Guid>,
    pub field_known_titles: [Option<u64>; 3],
    pub field_known_currencies: Option<u64>,
    pub xp: Option<u32>,
    pub next_level_xp: Option<u32>,
    pub skill_info: [Option<(u16, u16)>; 384],
    pub character_points: [Option<u32>; 2],
    pub track_creatures: Option<u32>,
    pub track_resources: Option<u32>,
    pub block_percentage: Option<f32>,
    pub dodge_percentage: Option<f32>,
    pub parry_percentage: Option<f32>,
    pub expertise: Option<u32>,
    pub offhand_expertise: Option<u32>,
    pub crit_percentage: Option<f32>,
    pub ranged_crit_percentage: Option<f32>,
    pub offhand_crit_percentage: Option<f32>,
    pub spell_crit_percentage: [Option<f32>; 7],
    pub shield_block: Option<u32>,
    pub shield_block_crit_percentage: Option<f32>,
    pub explored_zones: [Option<u32>; 128],
    pub rest_state_experience: Option<u32>,
    pub field_coinage: Option<u32>,
    pub field_mod_damage_done_pos: [Option<u32>; 7],
    pub field_mod_damage_done_neg: [Option<u32>; 7],
    pub field_mod_damage_done_pct: [Option<u32>; 7],
    pub field_mod_healing_done_pos: Option<f32>,
    pub field_mod_healing_pct: Option<u32>,
    pub field_mod_healing_done_pct: Option<f32>,
    pub field_mod_target_resistance: Option<u32>,
    pub field_mod_target_physical_resistance: Option<u32>,
    pub field_bytes: Option<Bytes4>,
    pub ammo_id: Option<u32>,
    pub self_res_spell: Option<u32>,
    pub field_pvp_medals: Option<u32>,
    pub field_buyback_price: [Option<u32>; 12],
    pub field_buyback_timestamp: [Option<u32>; 12],
    pub field_kills: Option<u32>,
    pub field_today_contribution: Option<u32>,
    pub field_yesterday_contribution: Option<u32>,
    pub field_lifetime_honorable_kills: Option<u32>,
    pub field_bytes2: Option<PlayerFieldBytes2Offsets>,
    pub watched_faction_index: Option<u32>,
    pub combat_rating: [Option<u32>; 25],
    pub field_arena_team_info: [Option<u32>; 21],
    pub field_honor_currency: Option<u32>,
    pub field_arena_currency: Option<u32>,
    pub field_max_level: Option<u32>,
    pub field_daily_quests: [Option<u32>; 25],
    pub rune_regen: Option<Rune>,
    pub no_reagent_cost: [Option<u32>; 3],
    pub glyph_slots: [Option<u32>; 6],
    pub glyphs: [Option<u32>; 6],
    pub glyphs_enabled: Option<u32>,
    pub pet_spell_power: Option<u32>,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes1 {
    pub skin: u8,
    pub face: u8,
    pub hairstyle: u8,
    pub hair_color: u8,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes2 {
    pub facial: u8,
    pub party: u8,
    pub bank_bag_slots: u8,
    pub rest_state: u8,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes3 {
    pub gender: u8,
    pub inebriation: u8,
    pub pvp_title: u8,
    pub arena_faction: u8,
}

#[derive(Debug, Clone, Copy, DekuRead, DekuWrite)]

pub struct QuestLogItem {
    pub id: u32,
    pub state: u32,
    pub counts1: u16,
    pub counts2: u16,
    pub counts3: u16,
    pub counts4: u16,
    pub time: u32,
}

#[derive(Debug, Clone, Copy, DekuRead, DekuWrite)]

pub struct EquipedItem {
    pub id: u32,
    pub permanent: u16,
    pub temporary: u16,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes4 {
    pub flags: u8,
    pub raf_grantable_level: u8,
    pub action_bar_toggles: u8,
    pub lifetime_max_pvp_rank: u8,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]

pub struct PlayerFieldBytes2Offsets {
    pub override_spells_id: u16,
    pub ignore_power_regen_prediction_mask: u8,
    pub aura_vision: u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
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
    Offhand = 16,
    Ranged = 17,
    Tabard = 18,
    End = 19,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]

pub struct Rune {
    pub blood: u32,
    pub unholy: u32,
    pub frost: u32,
    pub death: u32,
}
