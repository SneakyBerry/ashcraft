use deku::prelude::*;

#[derive(Debug, DekuWrite, DekuRead)]
pub struct Guid {
    pub(crate) guid: u64,
}
