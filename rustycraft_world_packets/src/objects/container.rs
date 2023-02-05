use crate::guid::Guid;
use crate::objects::prelude::Object;
use crate::objects::UpdateFields;
use crate::objects::size_helper::FieldSize;
use rustycraft_derive::IntoUpdateFields;

#[derive(Debug, Clone, IntoUpdateFields)]
#[meta(offset = 0x0040, tag = 0x0007)]
pub struct Container {
    #[nested]
    pub object: Object,
    pub num_slots: Option<u32>,
    pub slot: [Option<Guid>; 36],
}
