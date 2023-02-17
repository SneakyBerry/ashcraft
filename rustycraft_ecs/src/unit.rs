use crate::common::Health;
use crate::object::Object;
use bevy_ecs::prelude::*;
use bevy_transform::prelude::Transform;
use rustycraft_world_packets::objects::prelude::{CalcUpdate, UnitUpdate};
use rustycraft_world_packets::objects::UpdateFields;
use rustycraft_world_packets::power::Power;

#[derive(Debug, Clone, Component)]
pub struct Unit(UnitUpdate, UnitUpdate);
impl Unit {
    pub fn new() -> Self {
        Self(UnitUpdate::default(), UnitUpdate::default())
    }

    pub fn get_for_update(&mut self) -> &mut UnitUpdate {
        &mut self.1
    }

    pub fn render_update(&mut self) -> UpdateFields {
        let update = self.1.get_diff(Some(&self.0));
        std::mem::swap(&mut self.0, &mut self.1);
        self.1 = UnitUpdate::default();
        update
    }

    pub fn update_from_health(&mut self, health: &Health) {
        self.1.health = health.current().max(0.0) as u32;
    }

    pub fn update_from_pet(&mut self, pet: &Pet) {
        self.1.pet_number = pet.pet_number;
        self.1.pet_name_timestamp = pet.pet_name_timestamp;
        self.1.pet_experience = pet.pet_experience;
        self.1.pet_next_level_exp = pet.pet_next_level_exp;
    }

    pub fn update_from_equipment(&mut self, equipment: &Equipment) {
        if let Some(left_arm) = equipment.left_arm {
            self.1.virtual_item_slot_id[0] = left_arm;
        }
        if let Some(right_arm) = equipment.right_arm {
            self.1.virtual_item_slot_id[1] = right_arm;
        }
        if let Some(gun) = equipment.gun {
            self.1.virtual_item_slot_id[2] = gun;
        }
    }
}

#[derive(Debug, Eq, PartialEq, Component)]
pub struct Pet {
    pub pet_number: u32,
    pub pet_name_timestamp: u32,
    pub pet_experience: u32,
    pub pet_next_level_exp: u32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Equipment {
    pub left_arm: Option<u32>,
    pub right_arm: Option<u32>,
    pub gun: Option<u32>,
}

#[derive(Debug, Bundle)]
pub struct UnitBundle {
    pub unit: Unit,
    pub position: Transform,
    pub health: Health,
}
