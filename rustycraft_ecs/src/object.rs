use crate::impl_update;
use bevy_ecs::prelude::*;
use rustycraft_world_packets::guid::Guid;
use rustycraft_world_packets::objects::prelude::ObjectUpdate;

#[derive(Debug, Clone, Component)]
pub struct Object {
    pub guid: Guid,
    pub scale: f32,
}

impl Object {
    pub fn update_object(&self, old: Option<&Object>) -> ObjectUpdate {
        let mut update = ObjectUpdate::default();
        // impl_update!(self, old, update { scale => scale });
        update
    }
}
