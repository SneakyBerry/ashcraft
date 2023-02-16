use crate::guid::Guid;
use rustycraft_derive::CalcUpdate;

#[derive(Debug, Default, Clone, CalcUpdate, Builder)]
#[meta(offset = 0x0000, tag = 0x0001)]
pub struct ObjectUpdate {
    pub guid: Guid,
    pub type_id: u32,
    pub entry: u32,
    pub scale: f32,
}


