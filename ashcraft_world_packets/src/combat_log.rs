use deku::prelude::*;

pub mod client {}

pub mod server {
    use super::*;
    use crate::prelude::{Guid, Opcode};
    use ashcraft_derive::ServerPacket;

    #[derive(Debug, DekuWrite)]
    #[deku(type = "u8")]
    enum EnviromentalDamage {
        Exhausted = 0,
        Drowning = 1,
        Fall = 2,
        Lava = 3,
        Slime = 4,
        Fire = 5,
        FallToVoid = 6, // custom case for fall without durability loss
    }

    #[derive(Debug, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgEnvironmentalDamageLog)]
    struct EnvironmentalDamageLog {
        pub victim: Guid,
        pub damage_type: EnviromentalDamage,
        pub amount: u32,
        pub resisted: u32,
        pub absorbed: u32,
    }
}
