use bevy::prelude::*;

#[derive(Debug)]
pub struct ClientPacketReceived<T>(pub Entity, pub T);
