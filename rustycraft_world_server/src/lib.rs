mod client_session;
mod crypt;
mod objects;
#[allow(dead_code)]
mod packets;
mod server;
mod socket_manager;

use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::{DekuError, DekuRead, DekuWrite};
pub use socket_manager::SocketManager;
use std::ffi::CString;

#[macro_use]
extern crate log;
#[macro_use]
extern crate paste;
#[macro_use]
extern crate derive_builder;

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
            pub(crate) const $const_name: $inner_type = $const_value;
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
                $ ( paste! {f.write_str(&format!("\n\t{}: {},", stringify!([<$const_name:lower>]), self.[<is_ $const_name:lower>]()))?;} ) *
                f.write_str("\n}")?;
                Ok(())
            }
        }

    }
}
