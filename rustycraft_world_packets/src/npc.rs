use crate::guid::Guid;
use deku::prelude::*;

use rustycraft_derive::ServerPacket;

pub mod client {
    use super::*;

    #[derive(Debug, Clone, DekuRead)]
    pub struct Hello {
        pub unit: Guid,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct TrainerBuySpell {
        pub unit: Guid,
        pub spell_id: u32,
    }
}

pub mod server {
    use crate::prelude::Opcode;

    use super::*;

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "i32")]
    pub enum TrainerType {
        Class = 0,
        Mount = 1,
        TradeSkill = 2,
        Pet = 3,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct TrainerSpell {
        pub spell_id: i32,
        pub usable: bool,
        pub money_cost: i32,
        pub point_cost: [i32; 2],
        pub req_level: u8,
        pub req_skill_line: i32,
        pub req_skill_rank: i32,
        pub req_ability: [i32; 3],
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone, ServerPacket)]
    #[opcode(Opcode::SmsgTrainerList)]
    pub struct TrainerList {
        pub unit: Guid,
        pub trainer_type: TrainerType,
        #[deku(temp, temp_value = "spells.len() as i32")]
        pub spells_size: i32,
        pub spells: Vec<TrainerSpell>,
        #[deku(writer = "crate::write_c_string(deku::output, &self.greeting)")]
        pub greeting: String,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "i32")]
    pub enum BuyFailReasons {
        Unavailable = 0,
        NotEnoughMoney = 1,
        NotEnoughSkill = 2,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgTrainerBuyFailed)]
    pub struct TrainerBuyFailed {
        pub unit: Guid,
        pub spell_id: i32,
        pub reason: BuyFailReasons,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgTrainerBuySucceeded)]
    pub struct TrainerBuySucceeded {
        pub unit: Guid,
        pub spell_id: i32,
    }
}
