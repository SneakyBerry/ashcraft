use crate::packets::{DekuWriteWithDebug, Opcode, RequestResult};
use deku::prelude::*;

// We need to keep packet alignment
#[allow(dead_code)]
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

// We need to keep packet alignment
#[allow(dead_code)]
#[derive(Debug, DekuRead)]
pub struct TelemetryKey {
    pub(crate) unknown1: u16,
    pub(crate) unknown2: u32,
    pub(crate) unknown3: [u8; 4],
    pub(crate) unknown4: [u8; 20],
}

#[derive(Debug, DekuWrite)]
pub(crate) struct LogonProofResponse {
    pub(crate) result: RequestResult,
    pub(crate) server_proof: [u8; 20],
    pub(crate) account_flags: u32,
    pub(crate) hardware_survey_id: u32,
    pub(crate) login_flags: u16,
}

impl DekuWriteWithDebug for LogonProofResponse {}
