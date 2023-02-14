use bevy_ecs::prelude::*;
use rustycraft_world_packets::common::emotes::Emote;
use rustycraft_world_packets::common::school::SchoolIndexed;
use rustycraft_world_packets::common::stats::StatsIndexed;
use rustycraft_world_packets::objects::prelude::{AttackPower, ClassSpecific, NPCFlags, UnitData, UnitDynFlags, UnitFlags, UnitFlags2};

pub struct Unit {
    // Maybe get it based on childs?
    // pub charm: Option<Guid>,
    // pub summon: Option<Guid>,
    // pub critter: Option<Guid>,

    // can get it from parent
    // pub charmed_by: Option<Guid>,
    // pub summoned_by: Option<Guid>,
    // pub created_by: Option<Guid>,

    // pub target: Option<Guid>,   // Should calculate it based on ECS state
    // pub channel_object: Option<Guid>, //  ?
    // pub channel_spell: Option<u32>,   //  ?

    /* Get it from entitiy components
    pub data: UnitData,
    pub health: Option<u32>,
    pub power: [Option<u32>; 7]
    pub max_health: Option<u32>,
    pub max_power: [Option<u32>; 7],
    pub power_regen_flat_modifier: [Option<u32>; 7],
    pub power_regen_interrupted_flat_modifier: [Option<u32>; 7],
    pub level: Option<u32>,
     */
    pub virtual_items: Equipment,
    pub faction_template: u32,
    pub flags_1: Option<UnitFlags>,
    pub flags_2: Option<UnitFlags2>,
    pub aura_state: u32,

    pub base_attack: u32,
    pub off_attack: u32,
    pub range_attack: u32,

    pub bounding_radius: f32,
    pub combat_reach: f32,

    pub display_id: u32,
    pub native_display_id: u32,
    pub mount_display_id: u32,
    pub min_damage: f32,
    pub max_damage: f32,
    pub min_offhand_damage: f32,
    pub max_offhand_damage: f32,
    pub class_specific: ClassSpecific,
    pub pet: Option<Pet>,
    pub dynamic_flags: UnitDynFlags,
    pub mod_cast_speed: Option<f32>,
    pub created_by_spell: Option<u32>,
    pub npc_flags: NPCFlags,
    pub npc_emote_state: Emote,
    pub stat: StatsIndexed<u32>,
    pub stat_pos_effects: StatsIndexed<u32>,
    pub stat_neg_effects: StatsIndexed<u32>,
    pub resistance: SchoolIndexed<u32>,
    pub resistance_pos: SchoolIndexed<u32>,
    pub resistance_neg: SchoolIndexed<u32>,
    pub base_mana: u32,
    pub base_health: u32,
    pub bytes_2: [u8; 4],
    pub attack_power_melee: AttackPower,
    pub attack_power_ranged: AttackPower,
    pub min_ranged_damage: f32,
    pub max_ranged_damage: f32,
    pub power_cost_modifier: SchoolIndexed<u32>,
    pub power_cost_multiplier: SchoolIndexed<u32>,
    pub max_health_modifier: u32,
    pub hover_height: Option<u32>,
}

pub struct Pet {
    pub pet_number: u32,
    pub pet_name_timestamp: u32,
    pub pet_experience: u32,
    pub pet_next_level_exp: u32,
}

#[derive(Debug, Component)]
pub struct UnitStats {
    pub health: f32,
    pub mana: f32,
    pub rage: f32,
    pub focus: f32,
    pub energy: f32,
    pub happiness: f32,
    pub rune: f32,
    pub runic_power: f32,
}

pub struct Equipment {
    pub left_arm: Option<u32>,
    pub right_arm: Option<u32>,
    pub gun: Option<u32>,
}
