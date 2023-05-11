use bevy::prelude::*;
use ashcraft_world_packets::prelude::Opcode;

#[derive(Debug)]
pub struct ClientPacketReceived<T>(pub Entity, pub Opcode, pub T);
