use crate::opcodes::Opcode;
use deku::prelude::*;
use rustycraft_derive::ServerPacket;

#[derive(Debug, DekuWrite, Builder, ServerPacket)]
#[opcode(Opcode::SmsgTimeSyncReq)]
pub struct SmsgTimeSyncReq {
    pub time_sync: u32,
}
