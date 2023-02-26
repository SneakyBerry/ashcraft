use crate::guid::Guid;
use crate::map::Map;
use crate::opcodes::Opcode;
use crate::position::Vector3d;
use deku::prelude::*;
use rustycraft_derive::ServerPacket;

#[derive(Debug, Copy, Clone, DekuRead)]
pub struct CmsgPlayerLogin {
    pub guid: Guid,
}

#[derive(Debug, DekuWrite, ServerPacket)]
#[opcode(Opcode::SmsgLoginVerifyWorld)]
pub struct SmsgLoginVerifyWorld {
    pub map: Map,
    pub position: Vector3d,
}
