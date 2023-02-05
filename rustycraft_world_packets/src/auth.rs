use crate::expansion::Expansion;
use crate::opcodes::Opcode;
use crate::read_c_string;
use crate::response_code::ResponseCode;
use crate::ServerPacket;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;

#[derive(Debug, DekuWrite)]
pub struct AuthChallengeServer {
    #[deku(pad_bytes_before = "4", pad_bytes_after = "32")]
    pub server_seed: u32,
}

impl ServerPacket for AuthChallengeServer {
    fn get_opcode(&self) -> Opcode {
        Opcode::SmsgAuthChallenge
    }
}

#[derive(Debug, DekuRead)]
pub struct AuthSessionClient {
    #[deku(endian = "little")]
    pub client_build: u32,
    #[deku(endian = "little")]
    pub login_server_id: u32,
    #[deku(reader = "read_c_string(deku::rest)")]
    pub username: String,
    #[deku(endian = "little")]
    pub login_server_type: u32,
    #[deku(endian = "little")]
    pub client_seed: u32,
    #[deku(endian = "little")]
    pub region_id: u32,
    #[deku(endian = "little")]
    pub battleground_id: u32,
    #[deku(endian = "little")]
    pub realm_id: u32,
    /// Purpose and exact meaning of name unknown.
    /// TrinityCore has this name but never uses the variable afterwards.
    ///
    #[deku(endian = "little")]
    pub dos_response: u64,
    pub client_proof: [u8; 20],
    #[deku(endian = "little")]
    pub decompressed_addon_info_size: u32,
    #[deku(reader = "read_compressed_addon_info(deku::rest)")]
    pub compressed_addon_info: Vec<u8>,
}

/// Read the rest of a packet into compressed_addon_info
fn read_compressed_addon_info(
    rest: &BitSlice<u8, Msb0>,
) -> Result<(&BitSlice<u8, Msb0>, Vec<u8>), DekuError> {
    let mut rest = rest;
    let mut res = Vec::with_capacity(rest.len() / 8); // Byte size of bit vector
    while !rest.is_empty() {
        let (curr_rest, value) = u8::read(rest, ())?;
        rest = curr_rest;
        res.push(value)
    }
    Ok((rest, res))
}

#[derive(Debug, DekuWrite)]
pub struct AuthOk {
    #[deku(endian = "little")]
    pub billing_time: u32,
    pub billing_flags: u8,
    #[deku(endian = "little")]
    pub billing_rested: u32,
    pub expansion: Expansion,
}

#[derive(Debug, DekuWrite)]
pub struct AuthWaitQueue {
    #[deku(endian = "little")]
    pub queue_position: u32,
    pub realm_has_free_character_migration: bool,
}

#[derive(Debug, DekuWrite)]
pub struct AuthResponseServer {
    pub result: ResponseCode,
    pub ok: Option<AuthOk>,
    pub wait: Option<AuthWaitQueue>,
}

impl ServerPacket for AuthResponseServer {
    fn get_opcode(&self) -> Opcode {
        Opcode::SmsgAuthResponse
    }
}
