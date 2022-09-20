use deku::prelude::*;

use crate::guid::Guid;
use crate::objects::base::{BaseObject, Storage};
use crate::objects::object::ObjectFields;
use crate::objects::{InnerState, UpdateMaskObjectType};

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Container {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: InnerState,
}
impl Container {
    pub fn new(guid: Guid) -> Box<Self> {
        let mut object = Self::default();
        <Self as BaseObject<0x0000>>::set_guid(&mut object, guid);
        <Self as BaseObject<0x0000>>::set_object_type(
            &mut object,
            UpdateMaskObjectType::Container as u32,
        );
        Box::new(object)
    }
}

pub trait ContainerFields: BaseObject<0x0040> {
    fn set_container_num_slots(&mut self, container_num_slots: u32) -> &mut Self {
        self.set_value(container_num_slots, 0x0000)
    }
    fn get_container_num_slots(&self) -> Option<u32> {
        self.get_value(0x0000)
    }
    fn set_container_slot(&mut self, container_slot: Guid, index: usize) -> &mut Self {
        assert!(index < 36, "Index is out of range");
        self.set_value(container_slot, 0x0002 + index * 2)
    }
    fn get_container_slot(&self, index: usize) -> Option<Guid> {
        assert!(index < 36, "Index is out of range");
        self.get_value(0x0002 + index * 2)
    }
}

impl ContainerFields for Container {}
impl Storage for Container {
    fn get_inner(&self) -> &InnerState {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut InnerState {
        &mut self.inner
    }
}
