use crate::packets::guid::Guid;
use crate::packets::movement_block::MovementBlock;
use crate::packets::opcodes::Opcode;
use crate::packets::ServerPacket;
use deku::prelude::*;

#[derive(Debug, DekuWrite)]
pub struct SmsgUpdateObject {
    #[deku(endian = "little")]
    amount_of_objects: u32,
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

#[derive(Debug, DekuWrite)]
pub struct Object {
    pub update_type: ObjectUpdateType,
}

#[derive(Debug, DekuWrite)]
#[deku(type = "u8")]
pub enum ObjectUpdateType {
    #[deku(id = "0x0")]
    Values(Values),
    #[deku(id = "0x1")]
    Movement(Movement),
    #[deku(id = "0x2")]
    CreateObject(CreateObject),
    #[deku(id = "0x3")]
    CreateObject2(CreateObject2),
    #[deku(id = "0x4")]
    OutOfRangeObjects(OutOfRangeObjects),
    #[deku(id = "0x5")]
    NearObjects(NearObjects),
}

#[derive(Debug, DekuWrite)]
pub struct Values {
    guid1: Guid,
    // mask1: UpdateMask,
}

#[derive(Debug, DekuWrite)]
pub struct Movement {
    guid2: Guid,
    movement1: MovementBlock,
}

#[derive(Debug, DekuWrite)]
pub struct CreateObject {
    guid3: Guid,
    // mask2: UpdateMask,
    movement2: MovementBlock,
    object_type: ObjectType,
}

#[derive(Debug, DekuWrite)]
pub struct CreateObject2 {
    guid3: Guid,
    // mask2: UpdateMask,
    movement2: MovementBlock,
    object_type: ObjectType,
}

#[derive(Debug, DekuWrite)]
pub struct OutOfRangeObjects {
    count: u32,
    guids: Vec<Guid>,
}

#[derive(Debug, DekuWrite)]
pub struct NearObjects {
    count: u32,
    guids: Vec<Guid>,
}

impl ServerPacket for SmsgUpdateObject {
    fn get_opcode(&self) -> Opcode {
        Opcode::SmsgUpdateObject
    }
}

#[derive(Debug, DekuWrite)]
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
