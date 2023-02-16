use rustycraft_derive::CalcUpdate;
use crate::guid::Guid;
use crate::objects::helpers::ArrayWrapped;
use crate::objects::prelude::ObjectUpdate;

#[derive(Debug, Default, Clone, CalcUpdate, Builder)]
#[meta(offset = 0x0040, tag = 0x0007)]
pub struct ContainerUpdate {
    #[nested]
    pub object: ObjectUpdate,
    pub num_slots: u32,
    pub slot: ArrayWrapped<Guid, 36>,
}
