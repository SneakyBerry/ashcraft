use crate::packets::area::Area;
use crate::packets::class::Class;
use crate::packets::gear::CharacterGear;
use crate::packets::gender::Gender;
use crate::packets::guid::Guid;
use crate::packets::map::Map;
use crate::packets::opcodes::Opcode;
use crate::packets::race::Race;
use crate::packets::vector3d::Vector3d;
use crate::packets::ServerPacket;
use crate::{read_c_string, write_c_string};
use deku::prelude::*;

#[derive(Debug, DekuWrite)]
pub struct CharacterEnumServer {
    #[deku(update = "self.characters.len() as u8")]
    characters_count: u8,
    pub characters: Vec<Character>,
}

impl CharacterEnumServer {
    pub(crate) fn new(characters: Vec<Character>) -> CharacterEnumServer {
        CharacterEnumServer {
            characters_count: 0,
            characters,
        }
    }
}

impl ServerPacket for CharacterEnumServer {
    fn get_opcode(&self) -> Opcode {
        Opcode::SmsgCharEnum
    }
}

#[derive(Debug, DekuWrite, DekuRead)]
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
    #[deku(endian = "little")]
    pub guild_id: u32,
    #[deku(endian = "little")]
    pub flags: u32,
    #[deku(endian = "little")]
    pub recustomization_flags: u32,
    pub first_login: bool,
    #[deku(endian = "little")]
    pub pet_display_id: u32,
    #[deku(endian = "little")]
    pub pet_level: u32,
    #[deku(endian = "little")]
    pub pet_family: u32,
    pub equipment: [CharacterGear; 23],
}
