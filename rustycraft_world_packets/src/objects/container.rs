use crate::guid::Guid;
use crate::objects::{item, private};

pub trait Container: private::Object<0x0040> {
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
