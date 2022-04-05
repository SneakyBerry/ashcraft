use rustycraft_proc_macro::ServerPacket;
use crate::opcodes::OpcodeServer;
use crate::ServerPacket;


#[derive(Debug, ServerPacket)]
pub struct AuthChallenge {
    pub challenge: [u8; 16],
    pub dos_challenge: [u32; 8],
    pub dos_zero_bits: u8,
}
