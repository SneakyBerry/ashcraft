use deku::prelude::*;

pub mod client {}

pub mod server {
    use super::*;
    use crate::prelude::Opcode;
    use rustycraft_derive::ServerPacket;

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u8")]
    pub enum ComplaintStatus {
        Disabled = 0,
        EnabledWithoutAutoIgnore = 1,
        EnabledWithAutoIgnore = 2,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgFeatureSystemStatus)]
    pub struct FeatureSystemStatus {
        pub complaint_status: ComplaintStatus,
        pub voice_enabled: bool,
    }
}
