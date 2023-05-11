pub mod auth;
pub mod bank;
pub mod character;
pub mod characters;
pub mod chat;
mod client_packet_impl_read;
pub mod combat;
pub mod combat_log;
pub mod common;
pub mod gear;
pub mod guid;
pub mod guild;
pub mod inventory;
pub mod lfg;
pub mod login;
pub mod map;
pub mod misc;
pub mod movement_block;
pub mod movement_flags;
pub mod npc;
pub mod object;
pub mod objects;
pub mod opcodes;
pub mod position;
pub mod power;
pub mod query;
pub mod quest;
pub mod race;
pub mod response_code;
pub mod spell;
pub mod spline;
pub mod system;
pub mod templates;
pub mod time_sync;
pub mod totem;
pub mod transport;
pub mod tutorial;
pub mod update_flag;
pub mod world_state;

use crate::opcodes::Opcode;
use bytes::Bytes;
use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::prelude::*;
use std::any::Any;
use std::ffi::CString;
use std::fmt::Debug;
use std::mem::size_of_val;
use wow_srp::wrath_header::ServerEncrypterHalf;

pub mod prelude {
    pub mod client {
        pub use super::super::character::client::*;
        pub use super::super::chat::client::*;
        pub use super::super::combat::client::*;
        pub use super::super::guild::client::*;
        pub use super::super::lfg::client::*;
        pub use super::super::misc::client::*;
        pub use super::super::npc::client::*;
        pub use super::super::query::client::*;
        pub use super::super::quest::client::*;
        pub use super::super::spell::client::*;
        pub use super::super::system::client::*;
        pub use super::super::totem::client::*;
        pub use super::super::world_state::client::*;
    }
    pub mod server {
        pub use super::super::character::server::*;
        pub use super::super::chat::server::*;
        pub use super::super::combat::server::*;
        pub use super::super::combat_log::server::*;
        pub use super::super::guild::server::*;
        pub use super::super::misc::server::*;
        pub use super::super::npc::server::*;
        pub use super::super::query::server::*;
        pub use super::super::quest::server::*;
        pub use super::super::spell::server::*;
        pub use super::super::system::server::*;
        pub use super::super::totem::server::*;
        pub use super::super::world_state::server::*;
    }
    pub use super::auth::*;
    pub use super::bank::*;
    pub use super::characters::*;
    pub use super::common::prelude::*;
    pub use super::gear::*;
    pub use super::guid::*;
    pub use super::inventory::*;
    pub use super::login::*;
    pub use super::map::*;
    pub use super::movement_block::*;
    pub use super::movement_flags::*;
    pub use super::object::*;
    pub use super::objects::prelude::*;
    pub use super::opcodes::*;
    pub use super::position::*;
    pub use super::power::*;
    pub use super::query::*;
    pub use super::race::*;
    pub use super::response_code::*;
    pub use super::spline::*;
    pub use super::templates::*;
    pub use super::time_sync::*;
    pub use super::transport::*;
    pub use super::tutorial::*;
    pub use super::update_flag::*;
}

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
    (  size: $size:expr,  $struct_name:tt: $inner_type:tt {
        $( $const_name:tt = $const_value:tt),*
    } ) => {
        define_flags!($struct_name: $inner_type {
            $( $const_name = $const_value),*
        });
        impl $crate::objects::size_helper::FieldSize for $struct_name {
            const SIZE: usize = $size;
        }
    };

    (   $struct_name: ident: $inner_type: ident {
        $( $const_name:ident = $const_value: expr),*
    } ) => {
        #[derive(Clone, Eq, PartialEq, Default, DekuWrite, DekuRead)]
        pub struct $struct_name {
            inner: $inner_type,
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
