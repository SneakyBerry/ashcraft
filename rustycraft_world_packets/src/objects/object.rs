use crate::guid::Guid;
use crate::objects::size_helper::FieldSize;
use crate::objects::UpdateFields;
use rustycraft_derive::IntoUpdateFields;

#[derive(Debug, Default, Clone, IntoUpdateFields, Builder)]
#[meta(offset = 0x0000, tag = 0x0001)]
pub struct Object {
    pub guid: Option<Guid>,
    pub type_id: Option<u32>,
    pub entry: Option<u32>,
    pub scale_x: Option<f32>,
}
