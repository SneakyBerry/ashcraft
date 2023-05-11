use deku::prelude::*;

pub mod client {
    use super::*;
    use crate::prelude::Emote;

    #[derive(Debug, Clone, DekuRead)]
    pub struct ClientEmote {
        pub emote: Emote,
    }
}

pub mod server {
    use super::*;
    use crate::prelude::{Emote, Guid, Opcode};
    use ashcraft_derive::ServerPacket;

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgEmote)]
    pub struct ServerEmote {
        pub emote: Emote,
        pub guid: Guid,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgEmote)]
    pub struct ChatServerMessage {
        pub message_id: u32,
        #[deku(writer = "crate::write_c_string(deku::output, &self.message)")]
        pub message: String,
    }
}
