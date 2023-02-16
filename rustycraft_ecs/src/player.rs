use crate::common::{Guid, Health};
use crate::object::Object;
use crate::unit::Unit;
use bevy_ecs::prelude::*;
use rustycraft_world_packets::common::school::SchoolIndexed;
use rustycraft_world_packets::objects::prelude::{
    Bytes3, Bytes4, CharacterStyle1, CharacterStyle2, EquipedItem, PlayerFieldBytes2Offsets,
    PlayerFlags, QuestLogItem, Rune,
};

#[derive(Debug, Clone, Component, Builder)]
pub struct Player {
    pub duel_arbiter: Option<Guid>,
    pub flags: PlayerFlags,
    pub guild_id: Option<u32>,
    pub guild_rank: Option<u32>,
    pub character_style_1: CharacterStyle1,
    pub character_style_2: CharacterStyle2,
    pub bytes_3: Bytes3,
    pub duel_team: Option<u32>,
    pub guild_timestamp: Option<u32>,
    pub player_quests: [Option<QuestLogItem>; 25],
    pub visible_items: [Option<EquipedItem>; 19],
    pub chosen_title: Option<u32>,
    pub fake_inebriation: Option<i32>,
    pub unknown_af: Option<i32>,
    pub equipped_items: [Option<Guid>; 23],
    pub pack_slot: [Option<Guid>; 16],
    pub bank_slot: [Option<Guid>; 28],
    pub bank_bag_slot: [Option<Guid>; 7],
    pub vendor_buyback_slot: [Option<Guid>; 12],
    pub keyring_slot: [Option<Guid>; 32],
    pub currency_token_slot: [Option<Guid>; 32],
    pub far_sight: Option<Guid>,
    pub known_titles_masks: [u64; 3],
    pub known_currencies_mask: u64,
    pub xp: u32,
    pub next_level_xp: u32,
    pub skill_info: [Option<(u16, u16)>; 384],
    pub character_points: [Option<u32>; 2],
    pub track_creatures_mask: u32,
    pub track_resources_mask: u32,
    pub block_percentage: f32,
    pub dodge_percentage: f32,
    pub parry_percentage: f32,
    // ?
    pub expertise: u32,
    pub offhand_expertise: u32,
    pub crit_percentage: u32,
    pub ranged_crit_percentage: f32,
    pub offhand_crit_percentage: f32,
    pub spell_crit_percentage: SchoolIndexed<f32>,
    pub shield_block: u32,
    pub shield_block_crit_percentage: f32,
    pub explored_zones_masks: [u32; 128],
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
    // ???
    pub bytes_4: Bytes4,
    pub ammo_id: Option<u32>,
    pub self_res_spell: Option<u32>,
    pub pvp_medals: u32,
    pub buyback_price: [Option<u32>; 12],
    pub buyback_timestamp: [Option<u32>; 12],
    pub kills: u32,
    pub today_contribution: u32,
    pub yesterday_contribution: u32,
    pub lifetime_honorable_kills: u32,
    pub bytes2: PlayerFieldBytes2Offsets,
    pub watched_faction_index: Option<u32>,
    pub combat_rating: [Option<u32>; 25],
    pub arena_team_info: [Option<u32>; 21],
    pub honor_currency: u32,
    pub arena_currency: u32,
    pub max_level: u32,
    pub daily_quests: [Option<u32>; 25],
    pub rune_regen: Option<Rune>,
    pub no_reagent_cost: [Option<u32>; 3],
    pub glyph_slots: [Option<u32>; 6],
    pub glyphs: [Option<u32>; 6],
    pub glyphs_enabled: Option<u32>,
    pub pet_spell_power: Option<u32>,
}

#[derive(Debug, Bundle)]
struct PlayerBundle {
    pub object: Object,
    pub unit: Unit,
    pub player: Player,
    pub health: Health,
}