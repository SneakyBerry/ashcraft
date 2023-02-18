use bevy::prelude::*;
use rustycraft_world_packets::objects::prelude::{CalcUpdate, ObjectUpdate};
use rustycraft_world_packets::objects::UpdateFields;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Component)]
pub struct Object(bool, ObjectUpdate, ObjectUpdate);

impl Object {
    pub fn new() -> Self {
        Self(true, ObjectUpdate::default(), ObjectUpdate::default())
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
        self.2 = ObjectUpdate::default();
        self.0 = false;
    }
}

impl Deref for Object {
    type Target = ObjectUpdate;

    fn deref(&self) -> &Self::Target {
        &self.2
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0 = true;
        &mut self.2
    }
}
