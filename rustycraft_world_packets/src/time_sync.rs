use crate::opcodes::Opcode;
use crate::ServerPacket;
use deku::prelude::*;

#[derive(Debug, DekuWrite, Builder)]
pub struct SmsgTimeSyncReq {
    pub time_sync: u32,
}

impl ServerPacket for SmsgTimeSyncReq {
    fn get_opcode(&self) -> Opcode {
        Opcode::SmsgTimeSyncReq
    }
}
