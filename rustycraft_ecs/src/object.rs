use bevy_ecs::prelude::*;
use rustycraft_world_packets::guid::Guid;

#[derive(Component)]
pub struct Object {
    guid: Guid,
    scale: f32,
}
