use deku::prelude::*;

pub mod client {}

pub mod server {
    use super::*;
    use crate::prelude::{Area, Map, Opcode};
    use rustycraft_derive::ServerPacket;

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgUpdateWorldState)]
    pub struct WorldState {
        pub variable_id: i32,
        pub value: i32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgInitWorldStates)]
    pub struct InitWorldStates {
        pub map_id: Map,
        pub zone_id: u32, // TODO: Research this
        pub area_id: Area,
        #[deku(update = "self.world_states.len()")]
        pub world_states_size: u16,
        pub world_states: Vec<WorldState>,
    }
}
