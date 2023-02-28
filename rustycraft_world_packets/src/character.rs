use deku::prelude::*;

pub mod client {
    use super::*;

    #[derive(Debug, Clone, DekuRead)]
    pub struct ShowingCloak {
        pub show_cloak: bool,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct ShowingHelm {
        pub show_helm: bool,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct PlayedTimeClient {
        pub trigger_event: bool,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct LogoutRequest;

    #[derive(Debug, Clone, DekuRead)]
    pub struct PlayerLogout;

    #[derive(Debug, Clone, DekuRead)]
    pub struct LogoutCancel;
}

pub mod server {
    use super::*;

    use crate::opcodes::Opcode;
    use rustycraft_derive::ServerPacket;

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgLogoutComplete)]
    pub struct LogoutComplete();

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgLogoutCancelAck)]
    pub struct LogoutCancelAck();

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgLogoutResponse)]
    pub struct LogoutResponse {
        pub result: u32,
        pub instant: bool,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgPlayedTime)]
    pub struct PlayedTime {
        pub total: u32,
        pub level: u32,
        pub trigger_event: bool,
    }
}
