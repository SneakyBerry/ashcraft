use crate::guid::Guid;
use crate::objects::size_helper::FieldSize;
use crate::objects::object::Object;
use crate::objects::UpdateFields;

use rustycraft_derive::IntoUpdateFields;

#[derive(Debug, Default, Clone, IntoUpdateFields)]
#[meta(offset = 0x0006, tag = 0x0041)]
pub struct DynamicObject {
    #[nested]
    pub object: Object,
    pub caster: Option<Guid>,
    pub bytes: Option<[u8; 4]>,
    pub spell_id: Option<u32>,
    pub radius: Option<f32>,
    pub cast_time: Option<u32>,
}

pub enum DynamicObjectType {
    Portal = 0x0, // unused
    AreaSpell = 0x1,
    FarSightFocus = 0x2,
}
