use deku::prelude::*;

use crate::guid::Guid;
use crate::objects::base::{BaseObject, Storage};
use crate::objects::object::ObjectFields;
use crate::objects::{InnerState, UpdateMaskObjectType};

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct DynamicObject {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: InnerState,
}
impl DynamicObject {
    pub fn new(guid: Guid) -> Box<Self> {
        let mut object = Self::default();
        <Self as BaseObject<0x0000>>::set_guid(&mut object, guid);
        <Self as BaseObject<0x0000>>::set_object_type(
            &mut object,
            UpdateMaskObjectType::DynamicObject as u32,
        );
        Box::new(object)
    }
}

enum DynamicObjectType {
    Portal = 0x0, // unused
    AreaSpell = 0x1,
    FarSightFocus = 0x2,
}

pub trait DynamicObjectFields: BaseObject<0x0006> {
    fn set_dynamic_object_caster(&mut self, dynamic_object_caster: Guid) -> &mut Self {
        self.set_value(dynamic_object_caster, 0x0000)
    }
    fn get_dynamic_object_caster(&self) -> Option<Guid> {
        self.get_value(0x0000)
    }
    fn set_dynamic_object_bytes(&mut self, dynamic_object_bytes: [u8; 4]) -> &mut Self {
        self.set_value(dynamic_object_bytes, 0x0002)
    }
    fn get_dynamic_object_bytes(&self) -> Option<u32> {
        self.get_value(0x0002)
    }
    fn set_dynamic_object_spell_id(&mut self, dynamic_object_spell_id: u32) -> &mut Self {
        self.set_value(dynamic_object_spell_id, 0x0003)
    }
    fn get_dynamic_object_spell_id(&self) -> Option<u32> {
        self.get_value(0x0003)
    }
    fn set_dynamic_object_radius(&mut self, dynamic_object_radius: f32) -> &mut Self {
        self.set_value(dynamic_object_radius, 0x0004)
    }
    fn get_dynamic_object_radius(&self) -> Option<f32> {
        self.get_value(0x0004)
    }
    fn set_dynamic_object_cast_time(&mut self, dynamic_object_cast_time: u32) -> &mut Self {
        self.set_value(dynamic_object_cast_time, 0x0005)
    }
    fn get_dynamic_object_cast_time(&self) -> Option<u32> {
        self.get_value(0x0005)
    }
}

impl DynamicObjectFields for DynamicObject {}
impl Storage for DynamicObject {
    fn get_inner(&self) -> &InnerState {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut InnerState {
        &mut self.inner
    }
}
