use deku::prelude::*;

use crate::guid::Guid;
use crate::objects::base::{BaseObject, Storage};
use crate::objects::{base, InnerState, UpdateMaskObjectType};

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Object {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: InnerState,
}
impl Object {
    pub fn new(guid: Guid) -> Box<Self> {
        let mut object = Self::default();
        <Self as BaseObject<0x0000>>::set_guid(&mut object, guid);
        <Self as BaseObject<0x0000>>::set_object_type(
            &mut object,
            UpdateMaskObjectType::Object as u32,
        );
        Box::new(object)
    }
}

impl<T> ObjectFields for T where T: BaseObject<0x0000> {}
pub trait ObjectFields: BaseObject<0x0000> {
    fn get_guid(&self) -> Option<Guid> {
        self.get_value(0x0000)
    }
    fn get_object_type(&self) -> Option<u32> {
        self.get_value(0x0002)
    }
    fn set_object_entry(&mut self, object_entry: u32) -> &mut Self {
        self.set_value(object_entry, 0x0003)
    }
    fn get_object_entry(&self) -> Option<u32> {
        self.get_value(0x0003)
    }
    fn set_object_scale_x(&mut self, object_scale_x: f32) -> &mut Self {
        self.set_value(object_scale_x, 0x0004)
    }
    fn get_object_scale_x(&self) -> Option<f32> {
        self.get_value(0x0004)
    }
}

impl Storage for Object {
    fn get_inner(&self) -> &InnerState {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut InnerState {
        &mut self.inner
    }
}
