use crate::packets::IntoServerPacket;
use crate::OpcodeServer;
use deku::prelude::*;

#[derive(Debug, DekuWrite)]
#[deku(endian = "little")]
pub struct ClientCacheVersion {
    version: u32,
}

impl ClientCacheVersion {
    pub fn new() -> ClientCacheVersion {
        ClientCacheVersion { version: 0 }
    }
}

impl IntoServerPacket for ClientCacheVersion {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::CacheVersion
    }
}
