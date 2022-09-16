use crate::guid::PackedGuid;
use crate::movement_block::MovementBlock;
use crate::opcodes::Opcode;
use crate::update_mask::UpdateMask;
use crate::ServerPacket;
use deku::prelude::*;

#[derive(Debug, DekuWrite)]
pub struct SmsgUpdateObject {
    #[deku(endian = "little")]
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

#[derive(Debug, DekuWrite)]
pub struct Object {
    pub update_type: ObjectUpdateType, //+
}

#[derive(Debug, DekuWrite)]
#[deku(type = "u8")]
pub enum ObjectUpdateType {
    #[deku(id = "0x0")]
    Values {
        guid1: PackedGuid,
        mask1: UpdateMask,
    },
    #[deku(id = "0x1")]
    Movement {
        guid2: PackedGuid,
        movement1: MovementBlock,
    },
    #[deku(id = "0x2")]
    CreateObject {
        guid3: PackedGuid,
        mask2: UpdateMask,
        movement2: MovementBlock,
        object_type: ObjectType,
    },
    #[deku(id = "0x3")]
    CreateObject2 {
        guid3: PackedGuid,       // SHOULD BE PACKED 12
        object_type: ObjectType, // 13
        movement2: MovementBlock,
        mask2: UpdateMask,
    },
    #[deku(id = "0x4")]
    OutOfRangeObjects {
        #[deku(endian = "little")]
        count: u32,
        guids: Vec<PackedGuid>,
    },
    #[deku(id = "0x5")]
    NearObjects {
        #[deku(endian = "little")]
        count: u32,
        guids: Vec<PackedGuid>,
    },
}

#[cfg(test)]
mod test {
    use crate::object::ObjectUpdateType;
    use deku::prelude::*;

    #[test]
    fn test() {
        // println!("{:?}", &ObjectUpdateType::None { a: 10, b: 20 }.to_bytes());
    }
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
