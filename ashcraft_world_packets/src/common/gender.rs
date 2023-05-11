use deku::prelude::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuWrite, DekuRead)]
#[deku(type = "u8")]
pub enum Gender {
    #[default]
    Male = 0x00,
    Female = 0x01,
    /// Apparently used by hunter and warlock pets.
    ///
    None = 0x02,
}
