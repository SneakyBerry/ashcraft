use crate::core::components::common::Health;
use bevy::prelude::*;
use rustycraft_world_packets::objects::prelude::{CalcUpdate, PlayerUpdate};
use rustycraft_world_packets::objects::UpdateFields;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Component)]
pub struct Player(bool, PlayerUpdate, PlayerUpdate);

impl Player {
    pub fn new() -> Self {
        Self(true, PlayerUpdate::default(), PlayerUpdate::default())
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
        self.2 = PlayerUpdate::default();
        self.0 = false;
    }
}

impl Deref for Player {
    type Target = PlayerUpdate;

    fn deref(&self) -> &Self::Target {
        &self.2
    }
}

impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0 = true;
        &mut self.2
    }
}

#[derive(Debug, Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: Transform,
    pub health: Health,
}
