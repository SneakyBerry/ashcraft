use deku::bitvec::BitVec;
use deku::prelude::*;

use crate::guid::{Guid, HighGuid};
use crate::objects::InnerState;

impl<T, const N: usize> BaseObject<N> for T where T: Storage + Default {}
pub trait BaseObject<const OFFSET: usize>: Storage + Default {
    fn get_value<T>(&self, offset: usize) -> Option<T>
    where
        T: for<'a> DekuRead<'a>,
    {
        let values = BitVec::from_iter(
            self.get_inner()
                .range(OFFSET + offset..OFFSET + offset + std::mem::size_of::<T>() / 16)
                .map(|(_, v)| v.to_le_bytes())
                .flatten(),
        );
        Some(T::read(&values, ()).expect("Parse value failed").1)
    }

    fn set_value<T>(&mut self, value: T, offset: usize) -> &mut Self
    where
        T: DekuWrite,
    {
        let mut inner = self.get_inner_mut();
        let mut buffer = BitVec::new();
        value.write(&mut buffer, ()).expect("Write failed");
        inner.extend(buffer.as_raw_slice().chunks(4).enumerate().map(|(i, v)| {
            (
                i + offset + OFFSET,
                u32::from_le_bytes(v.try_into().expect("Chunk not equal to 4?")),
            )
        }));
        self
    }

    fn apply_xor(&mut self, offset: usize, mask: u32) -> &mut Self {
        let mut inner = self.get_inner_mut();
        let mut current = *inner.get(&(OFFSET + offset)).unwrap_or(&0);
        inner.insert(OFFSET + offset, current ^ mask);
        self
    }

    fn apply_or(&mut self, offset: usize, mask: u32) -> &mut Self {
        let mut inner = self.get_inner_mut();
        let mut current = *inner.get(&(OFFSET + offset)).unwrap_or(&0);
        inner.insert(OFFSET + offset, current | mask);
        self
    }

    fn apply_and(&mut self, offset: usize, mask: u32) -> &mut Self {
        let mut inner = self.get_inner_mut();
        let mut current = *inner.get(&(OFFSET + offset)).unwrap_or(&0);
        inner.insert(OFFSET + offset, current | mask);
        self
    }

    fn set_guid(&mut self, guid: Guid) -> &mut Self {
        self.set_value(guid, 0x0000)
    }

    fn set_object_type(&mut self, object_type: u32) -> &mut Self {
        self.set_value(object_type, 0x0002)
    }

    fn parse_guid(val: [u32; 2]) -> Guid {
        let u_val = val[0] as u64 | (val[1] as u64 >> 32);
        let high = HighGuid::from_bytes((&u_val.to_le_bytes(), 0))
            .expect(&format!("Guid parse failed: {:X}", u_val))
            .1;
        Guid::new(high, u_val)
    }
}
pub trait Storage {
    fn get_inner(&self) -> &InnerState;
    fn get_inner_mut(&mut self) -> &mut InnerState;
}
