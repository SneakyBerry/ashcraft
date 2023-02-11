use crate::opcodes::Opcode;
use crate::ServerPacket;
use deku::prelude::*;

#[derive(Debug, DekuWrite, Builder)]

pub struct SmsgTutorialFlags {
    pub tutorial_data0: u32,
    pub tutorial_data1: u32,
    pub tutorial_data2: u32,
    pub tutorial_data3: u32,
    pub tutorial_data4: u32,
    pub tutorial_data5: u32,
    pub tutorial_data6: u32,
    pub tutorial_data7: u32,
}

impl Default for SmsgTutorialFlags {
    fn default() -> Self {
        Self {
            tutorial_data0: u32::MAX,
            tutorial_data1: u32::MAX,
            tutorial_data2: u32::MAX,
            tutorial_data3: u32::MAX,
            tutorial_data4: u32::MAX,
            tutorial_data5: u32::MAX,
            tutorial_data6: u32::MAX,
            tutorial_data7: u32::MAX,
        }
    }
}

impl ServerPacket for SmsgTutorialFlags {
    fn get_opcode(&self) -> Opcode {
        Opcode::SmsgTutorialFlags
    }
}
