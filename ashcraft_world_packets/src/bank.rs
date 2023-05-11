use crate::opcodes::Opcode;
use crate::prelude::Guid;
use deku::prelude::*;
use ashcraft_derive::ServerPacket;

#[derive(Debug, Clone, DekuRead)]
pub struct AutoBankItem {
    pub bag: u8,
    pub slot: u8,
}

#[derive(Debug, Clone, DekuRead)]
pub struct AutoStoreBankItem {
    pub bag: u8,
    pub slot: u8,
}

#[derive(Debug, Clone, DekuRead)]
pub struct BuyBankSlot {
    pub banker: Guid,
}

#[derive(Debug, Clone, DekuWrite, ServerPacket)]
#[opcode(Opcode::SmsgBuyBankSlotResult)]
pub struct BuyBankSlotResult {
    pub result: u32,
}

#[derive(Debug, Clone, DekuWrite, ServerPacket)]
#[opcode(Opcode::SmsgShowBank)]
pub struct ShowBank {
    pub banker: Guid,
}
