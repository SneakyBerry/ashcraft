use bevy::prelude::*;
use rustycraft_world_packets::ServerPacket;
use std::fmt::Debug;

#[derive(Debug, Component)]
pub struct ClientPacketReceived<T>(pub Entity, pub T);

#[derive(Debug, Component)]
pub struct ServerPacketSend(pub Entity, pub Box<dyn ServerPacket>);
