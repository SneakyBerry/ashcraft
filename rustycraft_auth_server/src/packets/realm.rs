use crate::packets::{c_string_writer, DekuWriteWithDebug, Opcode};
use deku::prelude::*;

#[derive(Debug, DekuWrite)]
#[deku(type = "u32", endian = "little")]
pub(crate) enum Population {
    GreenRecommended = 0x43480000,
    RedFull = 0x43c80000,
    BlueRecommended = 0x44160000,
}

#[derive(Debug, DekuWrite)]
#[deku(type = "u8")]
pub(crate) enum RealmFlag {
    None = 0x00,
    Invalid = 0x01,
    Offline = 0x02,
    ForceBlueRecommended = 0x20,
    ForceGreenRecommended = 0x40,
    ForceRedFull = 0x80,
}

#[derive(Debug, DekuWrite)]
#[deku(type = "u8")]
pub(crate) enum RealmType {
    PvE = 0x0,
    PvP = 0x1,
    RP = 0x6,
    RPPvP = 0x8,
}

#[derive(Debug, DekuWrite)]
#[deku(type = "u8")]
pub(crate) enum RealmCategory {
    Default = 0x0,
    One = 0x1,
    Two = 0x2,
    Three = 0x3,
    Five = 0x5,
}

#[derive(Debug, DekuWrite)]
pub struct Realm {
    /// vmangos: this is the second column in Cfg_Configs.dbc
    ///
    pub(crate) realm_type: RealmType,
    pub(crate) locked: u8,
    pub(crate) flag: RealmFlag,
    #[deku(writer = "c_string_writer(deku::output, &self.name)")]
    pub(crate) name: String,
    #[deku(writer = "c_string_writer(deku::output, &self.address)")]
    pub(crate) address: String,
    pub(crate) population: Population,
    pub(crate) number_of_characters_on_realm: u8,
    pub(crate) category: RealmCategory,
    pub(crate) realm_id: u8,
}

#[derive(Debug, DekuWrite)]
pub(crate) struct RealmListResponse {
    pub(crate) opcode: Opcode,
    #[deku(endian = "little", pad_bytes_after = "4")]
    pub(crate) size: u16,
    pub(crate) realms_count: u16,
    #[deku(pad_bytes_after = "2")]
    pub(crate) realms: Vec<Realm>,
}

impl DekuWriteWithDebug for RealmListResponse {}
