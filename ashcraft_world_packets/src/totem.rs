use deku::prelude::*;

pub mod client {
    use super::*;

    #[derive(Debug, Clone, DekuRead)]
    pub struct TotemDestroyed {
        pub slot: u8,
    }
}

pub mod server {
    use super::*;
    use crate::prelude::{Guid, Opcode};
    use ashcraft_derive::ServerPacket;

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgTotemCreated)]
    pub struct TotemCreated {
        pub slot: u8,
        pub totem: Guid,
        pub duration: u32,
        pub spell_id: u32,
    }
}
