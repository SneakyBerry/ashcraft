use deku::prelude::*;
use std::fmt::Debug;
use deku::bitvec::{BitVec, Msb0};

pub mod logon_challenge;
pub mod logon_proof;
pub mod realm;
pub mod reconnect_challenge;
pub mod reconnect_proof;

pub trait DekuWriteDebug: DekuContainerWrite + Debug + Send {}

fn parse_reverse(mut input: Vec<u8>) -> Result<String, DekuError> {
    input.reverse();
    parse_string(input)
}

pub fn parse_string(input: Vec<u8>) -> Result<String, DekuError> {
    String::from_utf8(input).map_err(|e| DekuError::Parse(e.to_string()))
}

pub fn c_string_writer(output: &mut BitVec<Msb0, u8>, field: &str) -> Result<(), DekuError> {
    let mut field = field.to_owned();
    field.push('\0');
    field.as_bytes().write(output, ())
}

#[derive(Debug, Hash, Eq, PartialEq, DekuWrite, DekuRead)]
#[deku(type = "u8")]
pub(crate) enum Opcode {
    AuthLogonChallenge = 0x00,
    AuthLogonProof = 0x01,
    AuthReconnectChallenge = 0x02,
    AuthReconnectProof = 0x03,
    RealmList = 0x10,
    XferInitiate = 0x30,
    XferData = 0x31,
    XferAccept = 0x32,
    XferResume = 0x33,
    XferCancel = 0x34,
}

#[derive(Debug, Hash, Eq, PartialEq, DekuWrite, DekuRead)]
#[deku(type = "u8")]
pub(crate) enum AuthResult {
    WowSuccess = 0x00,
    WowFailBanned = 0x03,
    WowFailUnknownAccount = 0x04,
    WowFailIncorrectPassword = 0x05,
    WowFailAlreadyOnline = 0x06,
    WowFailNoTime = 0x07,
    WowFailDbBusy = 0x08,
    WowFailVersionInvalid = 0x09,
    WowFailVersionUpdate = 0x0A,
    WowFailInvalidServer = 0x0B,
    WowFailSuspended = 0x0C,
    WowFailFailNoaccess = 0x0D,
    WowSuccessSurvey = 0x0E,
    WowFailParentcontrol = 0x0F,
    WowFailLockedEnforced = 0x10,
    WowFailTrialEnded = 0x11,
    WowFailUseBattlenet = 0x12,
    WowFailAntiIndulgence = 0x13,
    WowFailExpired = 0x14,
    WowFailNoGameAccount = 0x15,
    WowFailChargeback = 0x16,
    WowFailInternetGameRoomWithoutBnet = 0x17,
    WowFailGameAccountLocked = 0x18,
    WowFailUnlockableLock = 0x19,
    WowFailConversionRequired = 0x20,
    WowFailDisconnected = 0xFF,
}

impl TryFrom<u8> for Opcode {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::AuthLogonChallenge),
            0x01 => Ok(Self::AuthLogonProof),
            0x02 => Ok(Self::AuthReconnectChallenge),
            0x03 => Ok(Self::AuthReconnectProof),
            0x10 => Ok(Self::RealmList),
            0x30 => Ok(Self::XferInitiate),
            0x31 => Ok(Self::XferData),
            0x32 => Ok(Self::XferAccept),
            0x33 => Ok(Self::XferResume),
            0x34 => Ok(Self::XferCancel),
            _ => Err(anyhow::anyhow!("Unexpected command")),
        }
    }
}

#[derive(Debug, DekuWrite)]
pub struct RequestResult {
    pub(crate) cmd: Opcode,
    pub(crate) protocol_version: Option<u8>,
    pub(crate) result: AuthResult,
}

#[derive(Debug, DekuRead)]
pub struct Version(u8, u8, u8, u16);

pub(crate) const VERSION_CHALLENGE: [u8; 16] = [
    0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1,
];
