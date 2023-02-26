use crate::guid::Guid;
use crate::objects::object::ObjectUpdate;
use rustycraft_derive::CalcUpdate;

#[derive(Debug, Default, Clone, CalcUpdate, Builder)]
#[meta(offset = 0x0006, tag = 0x0041)]
pub struct DynamicObjectUpdate {
    #[nested]
    pub object: ObjectUpdate,
    pub caster: Guid,
    pub bytes: [u8; 4],
    pub spell_id: u32,
    pub radius: f32,
    pub cast_time: u32,
}

pub enum DynamicObjectType {
    Portal = 0x0, // unused
    AreaSpell = 0x1,
    FarSightFocus = 0x2,
}
