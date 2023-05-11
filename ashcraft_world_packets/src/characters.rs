use crate::common::area::Area;
use crate::common::class::Class;
use crate::common::gender::Gender;
use crate::gear::CharacterGear;
use crate::guid::Guid;
use crate::map::Map;
use crate::opcodes::Opcode;
use crate::position::Vector3d;
use crate::race::Race;
use crate::{read_c_string, write_c_string};
use deku::prelude::*;
use ashcraft_derive::ServerPacket;

#[derive(Debug, DekuRead)]
pub struct CharEnum;

#[deku_derive(DekuWrite)]
#[derive(Debug, ServerPacket)]
#[opcode(Opcode::SmsgCharEnum)]
pub struct CharacterEnumServer {
    #[deku(temp, temp_value = "characters.len() as u8")]
    characters_count: u8,
    pub characters: Vec<Character>,
}

impl CharacterEnumServer {
    pub fn new(characters: Vec<Character>) -> CharacterEnumServer {
        CharacterEnumServer {
            characters,
        }
    }
}

#[derive(Debug, Clone, DekuWrite, DekuRead, Builder)]
pub struct Character {
    pub guid: Guid,
    #[deku(
        writer = "write_c_string(deku::output, &self.name)",
        reader = "read_c_string(deku::rest)"
    )]
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub gender: Gender,
    pub skin: u8,
    pub face: u8,
    pub hair_style: u8,
    pub hair_color: u8,
    pub facial_hair: u8,
    pub level: u8,
    pub area: Area,
    pub map: Map,
    pub position: Vector3d,
    pub guild_id: u32,
    pub flags: u32,
    pub recustomization_flags: u32,
    pub first_login: bool,
    pub pet_display_id: u32,
    pub pet_level: u32,
    pub pet_family: u32,
    pub equipment: [CharacterGear; 23],
}
