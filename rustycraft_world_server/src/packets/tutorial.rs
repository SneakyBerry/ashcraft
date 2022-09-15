use crate::packets::opcodes::Opcode;
use crate::packets::ServerPacket;
use deku::prelude::*;

#[derive(Debug, Default, DekuWrite)]
#[deku(endian = "little")]
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

impl ServerPacket for SmsgTutorialFlags {
    fn get_opcode(&self) -> Opcode {
        Opcode::SmsgTutorialFlags
    }
}
