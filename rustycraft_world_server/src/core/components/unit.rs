use crate::core::components::common::Health;
use bevy::prelude::*;
use rustycraft_world_packets::objects::prelude::{CalcUpdate, UnitUpdate};
use rustycraft_world_packets::objects::UpdateFields;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Component)]
pub struct Unit(bool, UnitUpdate, UnitUpdate);
impl Unit {
    pub fn new() -> Self {
        Self(true, UnitUpdate::default(), UnitUpdate::default())
    }

    pub fn render_update(&mut self) -> UpdateFields {
        if !self.0 {
            UpdateFields::new()
        } else {
            self.2.get_diff(Some(&self.1))
        }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.1, &mut self.2);
        self.2 = UnitUpdate::default();
        self.0 = false;
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

impl Deref for Unit {
    type Target = UnitUpdate;

    fn deref(&self) -> &Self::Target {
        &self.2
    }
}

impl DerefMut for Unit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0 = true;
        &mut self.2
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
