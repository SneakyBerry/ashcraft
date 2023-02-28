use bevy::prelude::*;
use rustycraft_world_packets::prelude::Opcode;

#[derive(Debug)]
pub struct ClientPacketReceived<T>(pub Entity, pub Opcode, pub T);
