use crate::guid::PackedGuid;
use crate::movement_block::MovementBlock;
use crate::objects;
use crate::opcodes::Opcode;
use deku::prelude::*;
use ashcraft_derive::ServerPacket;

#[derive(Debug, Clone, DekuWrite, Builder, ServerPacket)]
#[opcode(Opcode::SmsgUpdateObject)]
pub struct SmsgUpdateObject {
    amount_of_objects: u32, //+
    pub objects: Vec<Object>,
}

impl SmsgUpdateObject {
    pub fn new(objects: Vec<Object>) -> SmsgUpdateObject {
        SmsgUpdateObject {
            amount_of_objects: objects.len() as u32,
            objects,
        }
    }
}

#[derive(Debug, Clone, DekuWrite)]
pub struct Object {
    pub update_type: ObjectUpdateType,
}

#[derive(Debug, Clone, DekuWrite)]
#[deku(type = "u8")]
pub enum ObjectUpdateType {
    #[deku(id = "0x0")]
    Partial {
        guid: PackedGuid,
        update_fields: objects::UpdateFields,
    },
    #[deku(id = "0x1")]
    Movement {
        guid: PackedGuid,
        movement: MovementBlock,
    },
    #[deku(id = "0x2")]
    CreateObject {
        guid: PackedGuid,
        update_fields: objects::UpdateFields,
        movement: MovementBlock,
        object_type: ObjectType,
    },
    #[deku(id = "0x3")]
    CreateObject2 {
        guid: PackedGuid,
        object_type: ObjectType,
        movement: MovementBlock,
        update_fields: objects::UpdateFields,
    },
    #[deku(id = "0x4")]
    OutOfRangeObjects { count: u32, guids: Vec<PackedGuid> },
    #[deku(id = "0x5")]
    NearObjects { count: u32, guids: Vec<PackedGuid> },
}

#[derive(Debug, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ObjectType {
    Object = 0x0,
    Item = 0x1,
    Container = 0x2,
    Unit = 0x3,
    Player = 0x4,
    GameObject = 0x5,
    DynamicObject = 0x6,
    Corpse = 0x7,
}
