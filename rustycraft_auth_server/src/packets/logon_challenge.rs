use crate::packets::{parse_reverse, parse_string, Opcode, DekuWriteDebug, RequestResult, Version};
use deku::prelude::*;
use wow_srp::GENERATOR_LENGTH;

#[derive(Debug, DekuRead)]
pub(crate) struct LogonChallengeRequest {
    pub(crate) cmd: Opcode,
    pub(crate) protocol_version: u8,
    #[deku(endian = "little")]
    pub(crate) size: u16,
    #[deku(count = "4", map = "parse_reverse")]
    pub(crate) game_name: String,
    pub(crate) version: Version,
    #[deku(count = "4", map = "parse_reverse")]
    pub(crate) platform_data: String,
    #[deku(count = "4", map = "parse_reverse")]
    pub(crate) os: String,
    #[deku(count = "4", map = "parse_reverse")]
    pub(crate) country: String,
    #[deku(endian = "little")]
    pub(crate) utc_timezone_offset: u32,
    pub(crate) ip: [u8; 4],
    pub(crate) username_size: u8,
    #[deku(count = "username_size", map = "parse_string")]
    pub(crate) username: String,
}

#[derive(Debug, DekuWrite)]
pub(crate) struct LogonChallengeResponse {
    pub(crate) result: RequestResult,
    pub(crate) server_public_key: [u8; 32],
    pub(crate) generator_length: u8,
    pub(crate) generator: [u8; GENERATOR_LENGTH as usize],
    pub(crate) large_safe_prime_length: u8,
    pub(crate) large_safe_prime: [u8; 32],
    pub(crate) salt: [u8; 32],
    pub(crate) crc_salt: [u8; 16],
    pub(crate) security_flags: u8,
    // if (securityFlags & 0x01)               // PIN input
    // {
    // pkt << uint32(0);
    // pkt << uint64(0) << uint64(0);      // 16 bytes hash?
    // }
    //
    // if (securityFlags & 0x02)               // Matrix input
    // {
    // pkt << uint8(0);
    // pkt << uint8(0);
    // pkt << uint8(0);
    // pkt << uint8(0);
    // pkt << uint64(0);
    // }
    //
    // if (securityFlags & 0x04)               // Security token input
    // pkt << uint8(1);
}

impl DekuWriteDebug for LogonChallengeResponse {}
