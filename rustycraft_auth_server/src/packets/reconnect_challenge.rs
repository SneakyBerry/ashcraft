use crate::packets::{parse_reverse, parse_string, Opcode, Version};
use deku::prelude::*;

// We need to keep packet alignment
#[allow(dead_code)]
#[derive(Debug, DekuRead)]
pub(crate) struct ReconnectChallengeRequest {
    pub(crate) command: Opcode,
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
