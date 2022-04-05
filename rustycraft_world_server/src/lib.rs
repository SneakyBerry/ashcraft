pub mod opcodes;
pub mod packets;
pub mod world_events;
pub mod world_listener;
pub mod world_server;
pub mod world_session;

#[macro_use]
extern crate log;

use crate::opcodes::OpcodeServer;
use bytes::Bytes;

pub trait ServerPacket {
    const OPCODE: OpcodeServer;
    fn write(&self) -> Bytes;
}

pub trait ClientPacket {
    const OPCODE: OpcodeServer;
    fn read(data: Bytes) -> Option<Self>
    where
        Self: Sized;
}
