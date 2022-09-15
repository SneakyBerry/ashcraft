pub(crate) mod area;
pub(crate) mod auth;
pub(crate) mod characters;
pub(crate) mod class;
pub(crate) mod expansion;
pub(crate) mod gear;
pub(crate) mod gender;
pub(crate) mod guid;
pub(crate) mod inventory;
pub(crate) mod login;
pub(crate) mod map;
pub(crate) mod movement_block;
pub(crate) mod movement_flags;
pub(crate) mod object;
pub(crate) mod opcodes;
pub(crate) mod race;
pub(crate) mod response_code;
pub(crate) mod transport;
pub(crate) mod tutorial;
pub(crate) mod update_flag;
pub(crate) mod vector3d;
pub(crate) mod spline;
pub(crate) mod update_mask;

use crate::packets::opcodes::Opcode;
use bytes::Bytes;
use deku::prelude::*;
use std::mem::size_of_val;
use wow_srp::wrath_header::ServerEncrypterHalf;

pub(crate) trait ServerPacket: DekuContainerWrite + DekuUpdate
where
    Self: Sized,
{
    fn get_opcode(&self) -> Opcode;
    fn encode(mut self, encryption: Option<&mut ServerEncrypterHalf>) -> anyhow::Result<Bytes> {
        self.update()?;
        let mut vect = Vec::with_capacity(size_of_val(&self) + 4);
        let body = self.to_bytes()?;

        let size = (body.len() + 2) as u16; // Magic constant 2 is Opcode size
        let opcode = self.get_opcode();
        let mut headers = ServerHeaders { size, opcode }.to_bytes()?;

        if let Some(encryption) = encryption {
            encryption.encrypt(&mut headers);
        }

        vect.extend(headers);
        vect.extend(body);
        let res = Bytes::from(vect);
        Ok(res)
    }
}

#[derive(Debug, Copy, Clone, DekuWrite)]
pub(crate) struct ServerHeaders {
    #[deku(endian = "big")]
    size: u16,
    opcode: Opcode,
}
