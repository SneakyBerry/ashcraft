use crate::packets::Opcode;
use deku::prelude::*;

#[derive(Debug, DekuWrite)]
pub(crate) struct ReconnectProofResponse {
    pub(crate) command: Opcode,
    pub(crate) r1: [u8; 16],
    pub(crate) r2: [u8; 20],
    pub(crate) r3: [u8; 20],
    pub(crate) number_of_keys: u8,
}
