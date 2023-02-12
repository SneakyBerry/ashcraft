use crate::guid::Guid;
use crate::map::Map;
use crate::opcodes::Opcode;
use crate::position::Vector3d;
use crate::ServerPacket;
use deku::prelude::*;

#[derive(Debug, DekuRead, Valuable)]
pub struct CmsgPlayerLogin {
    pub guid: Guid,
}

#[derive(Debug, DekuWrite, DekuRead, Valuable)]
pub struct SmsgLoginVerifyWorld {
    pub map: Map,
    pub position: Vector3d,
}

impl ServerPacket for SmsgLoginVerifyWorld {
    fn get_opcode(&self) -> Opcode {
        Opcode::SmsgLoginVerifyWorld
    }
}
