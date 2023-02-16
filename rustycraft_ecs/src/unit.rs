use crate::impl_update;
use crate::object::Object;
use bevy_ecs::prelude::*;
use rustycraft_world_packets::common::emotes::Emote;
use rustycraft_world_packets::common::school::SchoolIndexed;
use rustycraft_world_packets::common::stats::StatsIndexed;
use rustycraft_world_packets::objects::prelude::{
    AttackPower, ClassSpecific, NPCFlags, ObjectUpdate, UnitData, UnitDynFlags, UnitFlags,
    UnitFlags2, UnitUpdate,
};

#[derive(Debug, Clone, PartialEq, Component, Builder)]
pub struct Unit {
    pub data: UnitData,

    pub virtual_items: Equipment,
    pub faction_template: u32,
    pub flags_1: UnitFlags,
    pub flags_2: UnitFlags2,
    pub aura_state: u32,

    pub base_attack: u32,
    pub off_attack: u32,
    pub range_attack: u32,

    pub bounding_radius: f32,
    pub combat_reach: f32,

    pub display_id: u32,
    pub native_display_id: u32,
    pub mount_display_id: Option<u32>,
    pub min_damage: f32,
    pub max_damage: f32,
    pub min_offhand_damage: f32,
    pub max_offhand_damage: f32,
    pub class_specific: ClassSpecific,
    pub dynamic_flags: UnitDynFlags,
    pub mod_cast_speed: Option<f32>,
    pub created_by_spell: Option<u32>,
    pub npc_flags: NPCFlags,
    pub emote_state: Emote,
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

impl Unit {
    pub fn into_update_unit(&self, object: &ObjectUpdate, old: Option<&Unit>) -> UnitUpdate {
        let mut update = UnitUpdate::default();
        update.object = object.clone();
        update.flags_1 = if let Some(flags_1) = old.map(|o| &o.flags_1) {
            if flags_1 == &self.flags_1 {
                Some(self.flags_1.clone())
            } else {
                None
            }
        } else {
            None
        };
        // impl_update!(self, old, update {
        //     data => data,
        //     // virtual_items => virtual_items,
        //     faction_template => faction_template,
        //     flags_1 => flags_1,
        //     flags_2 => flags_2,
        //     aura_state => aura_state,
        //     // base_attack => base_attack,
        //     // off_attack => off_attack,
        //     // range_attack => range_attack,
        //     bounding_radius => bounding_radius,
        //     combat_reach => combat_reach,
        //     display_id => display_id,
        //     native_display_id => native_display_id,
        //     mount_display_id => mount_display_id,
        //     min_damage => min_damage,
        //     max_damage => max_damage,
        //     min_offhand_damage => min_offhand_damage,
        //     max_offhand_damage => max_offhand_damage,
        //     class_specific => class_specific,
        //     dynamic_flags => dynamic_flags,
        //     mod_cast_speed => mod_cast_speed,
        //     created_by_spell => created_by_spell,
        //     npc_flags => npc_flags,
        //     emote_state => emote_state,
        //     stat => stat,
        //     stat_pos_effects => stat_pos_effects,
        //     stat_neg_effects => stat_neg_effects,
        //     resistance => resistance,
        //     resistance_pos => resistance_pos,
        //     resistance_neg => resistance_neg,
        //     base_mana => base_mana,
        //     base_health => base_health,
        //     bytes_2 => bytes_2,
        //     attack_power_melee => attack_power_melee,
        //     attack_power_ranged => attack_power_ranged,
        //     min_ranged_damage => min_ranged_damage,
        //     max_ranged_damage => max_ranged_damage,
        //     power_cost_modifier => power_cost_modifier,
        //     power_cost_multiplier => power_cost_multiplier,
        //     max_health_modifier => max_health_modifier,
        //     hover_height => hover_height
        // });
        update
    }
}

#[derive(Debug, Eq, PartialEq, Component)]
pub struct Pet {
    pub pet_number: u32,
    pub pet_name_timestamp: u32,
    pub pet_experience: u32,
    pub pet_next_level_exp: u32,
}

#[derive(Debug, PartialEq, Copy, Clone, Component)]
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Equipment {
    pub left_arm: Option<u32>,
    pub right_arm: Option<u32>,
    pub gun: Option<u32>,
}
