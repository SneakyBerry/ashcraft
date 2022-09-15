use crate::packets::guid::Guid;
use crate::packets::map::Map;
use crate::packets::opcodes::Opcode;
use crate::packets::vector3d::Vector3d;
use crate::packets::ServerPacket;
use deku::prelude::*;

#[derive(Debug, DekuRead)]
pub struct CmsgPlayerLogin {
    pub guid: Guid,
}

#[derive(Debug, DekuWrite, DekuRead)]
pub struct SmsgLoginVerifyWorld {
    pub map: Map,
    pub position: Vector3d,
    #[deku(endian = "little")]
    pub orientation: f32,
}

impl ServerPacket for SmsgLoginVerifyWorld {
    fn get_opcode(&self) -> Opcode {
        Opcode::SmsgLoginVerifyWorld
    }
}
