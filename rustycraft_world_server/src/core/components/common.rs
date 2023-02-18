use bevy::prelude::*;
use rustycraft_world_packets::guid::Guid as PacketsGuid;

#[derive(Debug, Clone, Component)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn damage(&mut self, amount: f32) {
        self.current -= amount;
    }

    pub fn heal(&mut self, amount: f32) {
        self.current += amount;
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn max(&self) -> f32 {
        self.max
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Guid(PacketsGuid);

impl Guid {
    pub fn new(guid: PacketsGuid) -> Self {
        Self(guid)
    }

    pub fn guid(&self) -> PacketsGuid {
        self.0
    }
}
