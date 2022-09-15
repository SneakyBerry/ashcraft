use deku::prelude::*;

#[derive(Debug, DekuWrite, DekuRead)]
#[deku(type = "u8")]
pub enum Gender {
    Male = 0x01,
    Female = 0x02,
    /// Apparently used by hunter and warlock pets.
    ///
    None = 0x03,
}
