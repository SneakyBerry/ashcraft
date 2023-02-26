use crate::common::helpers::ArrayWrapped;
use crate::guid::Guid;
use crate::objects::prelude::ObjectUpdate;
use rustycraft_derive::CalcUpdate;

#[derive(Debug, Default, Clone, CalcUpdate, Builder)]
#[meta(offset = 0x0040, tag = 0x0007)]
pub struct ContainerUpdate {
    #[nested]
    pub object: ObjectUpdate,
    pub num_slots: u32,
    pub slot: ArrayWrapped<Guid, 36>,
}
