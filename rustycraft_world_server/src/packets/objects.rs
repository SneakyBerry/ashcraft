use bytes::Bytes;
use deku::prelude::*;
use crate::{IntoServerPacket, ObjectGuid, OpcodeServer};


#[derive(Debug, DekuWrite)]
pub struct UpdateObject {
    #[deku(endian = "little")]
    map: u16,
    #[deku(endian = "little")]
    num_obj_updates: u32,
    #[deku(count = "num_obj_updates")]
    data: Vec<UpdateData>
}

impl UpdateObject {
    pub fn new() -> UpdateObject {
        UpdateObject {
            num_obj_updates: 0,
            map: 1,
            data: vec![]
        }
    }
}

impl IntoServerPacket for UpdateObject {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::UpdateObject
    }
}


#[repr(u8)]
enum UpdateType
{
    Values         = 0,
    CreateObject1  = 1,
    CreateObject2  = 2,
    DestroyObjects = 3
}


enum ObjectCreateType
{
    Create1 = 0,
    Create2 = 1
}


#[derive(Debug, DekuWrite)]
pub struct UpdateData {
    r#type: u8,

}

pub enum UpdateDataInner {
   Values {
       guid: ObjectGuid,

   }
}


pub struct UpdateValueEntry {

}