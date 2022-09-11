use crate::{IntoServerPacket, ObjectGuid, OpcodeServer};
use deku::prelude::*;

#[derive(Debug, DekuWrite)]
pub struct UpdateObject {
    #[deku(endian = "little")]
    map: u16,
    #[deku(endian = "little")]
    num_obj_updates: u32,
    #[deku(count = "num_obj_updates")]
    data: Vec<UpdateData>,
}

impl UpdateObject {
    pub fn new() -> UpdateObject {
        UpdateObject {
            num_obj_updates: 0,
            map: 1,
            data: vec![],
        }
    }
}

impl IntoServerPacket for UpdateObject {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::UpdateObject
    }
}

#[repr(u8)]
enum UpdateType {
    Values = 0,
    CreateObject1 = 1,
    CreateObject2 = 2,
    DestroyObjects = 3,
}

enum ObjectCreateType {
    Create1 = 0,
    Create2 = 1,
}

enum CreateObjectType {
    InRange = 1,
    Spawn = 2,
}

#[derive(Debug, DekuWrite)]
pub struct UpdateData {
    r#type: u8,
}

pub enum UpdateDataInner {
    Values {
        guid: ObjectGuid,
        mask_size: [u8; 12],
        // it is bit vector
        update_mask: Vec<i32>,
    },
    Create {
        guid: ObjectGuid,
        // if update data type == CreateObject1 then InRange else Spawn
        // create_type: CreateObjectType,
    },
}

pub struct UpdateValueEntry {}

pub enum ObjectType {
    Object = 0,
    Item = 1,
    Container = 2,
    AzeriteEmpoweredItem = 3,
    AzeriteItem = 4,
    Unit = 5,
    Player = 6,
    ActivePlayer = 7,
    GameObject = 8,
    DynamicObject = 9,
    Corpse = 10,
    AreaTrigger = 11,
    SceneObject = 12,
    Conversation = 13,
    Map = 14,
}

// Common object attributes
pub struct WoWObject {
    guid: ObjectGuid,
    r#type: ObjectType,
    movement: u64,
    map: u16,
    area: i16,
    zone: i16,

    // public IObjectData ObjectData;
    // public Dictionary<int, UpdateField> UpdateFields;
    // public Dictionary<int, List<UpdateField>> DynamicUpdateFields;
    phase_mask: u16,
    phases: Vec<u8>,

    difficulty_id: u16,
    force_temp_spawn: bool,
}
