use bevy_ecs::prelude::*;
use rustycraft_world_packets::objects::prelude::{CalcUpdate, ObjectUpdate};
use rustycraft_world_packets::objects::UpdateFields;

#[derive(Debug, Clone, Component)]
pub struct Object(ObjectUpdate, ObjectUpdate);

impl Object {
    pub fn new() -> Self {
        Self(ObjectUpdate::default(), ObjectUpdate::default())
    }

    pub fn get_for_update(&mut self) -> &mut ObjectUpdate {
        &mut self.1
    }

    pub fn render_update(&mut self) -> UpdateFields {
        let update = self.1.get_diff(Some(&self.0));
        std::mem::swap(&mut self.0, &mut self.1);
        self.1 = ObjectUpdate::default();
        update
    }
}
