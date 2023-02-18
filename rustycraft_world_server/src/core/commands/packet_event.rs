use crate::core::events::packets::ClientPacketReceived;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use std::fmt::Debug;

pub struct EmitClientPacketEvent<T>(pub Entity, pub T);
impl<T> Command for EmitClientPacketEvent<T>
where
    T: Send + Sync + 'static,
{
    fn write(mut self, world: &mut World) {
        let mut events = world.resource_mut::<Events<ClientPacketReceived<T>>>();
        events.send(ClientPacketReceived(self.0, self.1));
    }
}
