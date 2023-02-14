pub mod auth;
pub mod characters;
mod client_packet_impl_read;
pub mod common;
pub mod gear;
pub mod guid;
pub mod inventory;
pub mod login;
pub mod map;
pub mod movement_block;
pub mod movement_flags;
pub mod object;
pub mod objects;
pub mod opcodes;
pub mod position;
pub mod power;
pub mod race;
pub mod response_code;
pub mod spline;
pub mod time_sync;
pub mod transport;
pub mod tutorial;
pub mod update_flag;
pub mod query;
pub mod templates;

use crate::opcodes::Opcode;
use bytes::Bytes;
use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::prelude::*;
use std::any::Any;
use std::ffi::CString;
use std::fmt::Debug;
use std::mem::size_of_val;
use wow_srp::wrath_header::ServerEncrypterHalf;

#[macro_use]
extern crate paste;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate thiserror;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Encode packet error: {}", .0)]
    EncodingError(#[from] DekuError),
}

pub trait ServerPacket: DekuContainerWrite + DekuUpdate + Send + Sync + Debug {
    fn get_opcode(&self) -> Opcode;
    fn encode(
        &mut self,
        encryption: Option<&mut ServerEncrypterHalf>,
    ) -> Result<Bytes, PacketError> {
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
pub struct ServerHeaders {
    #[deku(endian = "big")]
    size: u16,
    opcode: Opcode,
}

#[derive(Debug)]
pub struct ClientPacket {
    opcode: Opcode,
    data: Box<dyn Any + Send + Sync>,
}

impl ClientPacket {
    pub fn get_opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn data_as<T>(self) -> T
    where
        T: 'static,
    {
        *self.data.downcast().expect("Incorrect generic type")
    }
}

fn read_c_string(rest: &BitSlice<u8, Msb0>) -> Result<(&BitSlice<u8, Msb0>, String), DekuError> {
    let (rest, value) = CString::read(rest, ())?;
    Ok((rest, value.into_string().expect("Is it ever happen?")))
}

pub fn write_c_string(output: &mut BitVec<u8, Msb0>, field: &str) -> Result<(), DekuError> {
    let mut field = field.to_owned();
    field.push('\0');
    field.as_bytes().write(output, ())
}

#[macro_export]
macro_rules! define_flags {
    (   StructName: $struct_name: ident
        InnerType: $inner_type: ident {
        $( $const_name:ident = $const_value: expr),*
    } ) => {
        #[derive(Clone, PartialEq, Default, DekuWrite, DekuRead)]
        pub struct $struct_name {
            inner: $inner_type,
        }

        impl $crate::objects::size_helper::FieldSize for $struct_name {
            const SIZE: usize = 1;
        }

        paste! {
            impl $struct_name {
                pub const fn new(inner: $inner_type) -> Self { Self { inner } }
                $(
                    pub const $const_name: $inner_type = $const_value;
                    pub const fn [<is_ $const_name:lower>](&self) -> bool {
                        (self.inner & Self::$const_name) != 0
                    }

                    pub const fn [<new_ $const_name:lower>]() -> Self {
                        Self { inner: Self::$const_name }
                    }

                    pub fn [<set_ $const_name:lower>](&mut self) -> &mut Self {
                        self.inner |= Self::$const_name;
                        self
                    }

                    pub fn [<clear_ $const_name:lower>](&mut self) -> &mut Self {
                        self.inner &= !Self::$const_name;
                        self
                    }
                ) *
                }
            impl ::std::fmt::Debug for $struct_name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.debug_struct(stringify!($struct_name))
                $ (
                    .field(stringify!([<$const_name:lower>]), &self.[<is_ $const_name:lower>]())
                ) *
                    .finish()?;

                    Ok(())
                }
            }
        }
    }
}
