use deku::prelude::*;
use crate::packets::{Opcode, DekuWriteDebug, RequestResult};

#[derive(Debug, DekuRead)]
pub(crate) struct LogonProofRequest {
    pub(crate) cmd: Opcode,
    pub(crate) client_public_key: [u8; 32],
    pub(crate) client_proof: [u8; 20],
    pub(crate) crc_hash: [u8; 20],
    // pub(crate) telemetry_keys_size: u8,
    // #[deku(count = "telemetry_keys_size")]
    // pub(crate) telemetry_keys: Vec<TelemetryKey>,
    // pub(crate) security_flags: u8,
}

#[derive(Debug, DekuRead)]
pub struct TelemetryKey {
    #[deku(endian = "little")]
    pub(crate) unknown1: u16,
    #[deku(endian = "little")]
    pub(crate) unknown2: u32,
    pub(crate) unknown3: [u8; 4],
    pub(crate) unknown4: [u8; 20],
}

#[derive(Debug, DekuWrite)]
pub(crate) struct LogonProofResponse {
    pub(crate) result: RequestResult,
    pub(crate) server_proof: [u8; 20],
    #[deku(endian = "little")]
    pub(crate) account_flags: u32,
    #[deku(endian = "little")]
    pub(crate) hardware_survey_id: u32,
    #[deku(endian = "little")]
    pub(crate) login_flags: u16,
}

impl DekuWriteDebug for LogonProofResponse {}
