use crate::common::{Guid, Health};
use crate::object::Object;
use crate::unit::Unit;
use bevy_ecs::prelude::*;
use bevy_transform::prelude::Transform;
use rustycraft_world_packets::objects::prelude::{CalcUpdate, PlayerUpdate};
use rustycraft_world_packets::objects::UpdateFields;

#[derive(Debug, Clone, Component)]
pub struct Player(PlayerUpdate, PlayerUpdate);

impl Player {
    pub fn new() -> Self {
        Self(PlayerUpdate::default(), PlayerUpdate::default())
    }

    pub fn get_for_update(&mut self) -> &mut PlayerUpdate {
        &mut self.1
    }

    pub fn get(&self) -> &PlayerUpdate {
        &self.1
    }

    pub fn render_update(&self) -> UpdateFields {
        self.1.get_diff(Some(&self.0))
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.0, &mut self.1);
        self.1 = PlayerUpdate::default();
    }
}


#[derive(Debug, Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: Transform,
    pub health: Health,
}
