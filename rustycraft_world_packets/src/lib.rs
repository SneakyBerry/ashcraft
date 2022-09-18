pub mod area;
pub mod auth;
pub mod characters;
pub mod class;
pub mod corpse;
pub mod expansion;
pub mod game_object;
pub mod gear;
pub mod gender;
pub mod guid;
pub mod inventory;
pub mod login;
pub mod map;
pub mod movement_block;
pub mod movement_flags;
pub mod object;
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
pub mod update_mask;
pub mod objects;

use std::ffi::CString;
use crate::opcodes::Opcode;
use bytes::Bytes;
use deku::prelude::*;
use std::mem::size_of_val;
use deku::bitvec::{BitSlice, BitVec, Msb0};
use wow_srp::wrath_header::ServerEncrypterHalf;

#[macro_use]
extern crate paste;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate const_format;


#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Encode packet error: {}", .0)]
    EncodingError(#[from] DekuError)
}


pub trait ServerPacket: DekuContainerWrite + DekuUpdate
where
    Self: Sized,
{
    fn get_opcode(&self) -> Opcode;
    fn encode(mut self, encryption: Option<&mut ServerEncrypterHalf>) -> Result<Bytes, PacketError> {
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
        println!("OP: {:?}:\n {:#?}", self.get_opcode(), &vect[4..]);
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

fn read_c_string(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, String), DekuError> {
    let (rest, value) = CString::read(rest, ())?;
    Ok((rest, value.into_string().expect("Is it ever happen?")))
}

pub fn write_c_string(output: &mut BitVec<Msb0, u8>, field: &str) -> Result<(), DekuError> {
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


        impl $struct_name {
            pub const fn new(inner: $inner_type) -> Self { Self { inner } }
            $(
            pub const $const_name: $inner_type = $const_value;
        paste! {
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
                self.inner &= Self::$const_name.reverse_bits();
                self
            }
        }) *
        }
        impl std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&format!("{} {{", std::any::type_name::<Self>()))?;
                $ ( paste! {f.write_str(&format!(" {}: {},", stringify!([<$const_name:lower>]), self.[<is_ $const_name:lower>]()))?;} ) *
                f.write_str(" }")?;
                Ok(())
            }
        }

    }
}

#[macro_export]
macro_rules! setters {
    (
        pub struct $obj_type:expr => $struct_name:ident {
            $ (
                $bit:expr => $field_name:ident: $field_type:ty
            ),*
        }
    ) => {
        #[derive(Debug, Default, Clone, DekuWrite)]
        pub struct $struct_name {
            is_set_flags_count: u8,
            is_set_flags: Vec<u32>,
            object_type: u32,
            $(
            $field_name: Option<$field_type>
            ),*
        }

        impl $struct_name  {
            pub fn new() -> Self {
                const OBJECT_FIELD_TYPE: u16 = 2;

                let mut is_set_flags = vec![];

                Self::is_set_flags_set(&mut is_set_flags, OBJECT_FIELD_TYPE);
                Self {
                    is_set_flags_count: is_set_flags.len() as u8,
                    is_set_flags,
                    object_type: 0x0001 | $obj_type as u32,
                    $ (
                    $field_name: None
                    ), *
                }
            }

            fn is_set_flags_set(is_set_flags: &mut Vec<u32>, bit: u16) {
                let index = bit / 32;
                let offset = bit % 32;

                if index >= is_set_flags.len() as u16 {
                    let extras = index - is_set_flags.len() as u16;
                    for _ in 0..=extras {
                        is_set_flags.push(0);
                    }
                }

                is_set_flags[index as usize] |= 1 << offset;
            }

        $(
            pub fn $field_name(&mut self, val: $field_type) -> &mut Self {
                self.$field_name = Some(val);
                Self::is_set_flags_set(&mut self.is_set_flags, $bit);
                self
            }
        )*
        }
    };
}
