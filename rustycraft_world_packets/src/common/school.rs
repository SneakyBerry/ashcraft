use deku::prelude::*;
use std::ops::{Index, IndexMut};
use crate::common::helpers::ArrayWrapped;

#[derive(Debug, Clone, Copy, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u32")]
pub enum DamageSchool {
    Physical = 0,
    Holy = 1,
    Fire = 2,
    Nature = 3,
    Frost = 4,
    Shadow = 5,
    Arcane = 6,
}


pub type SchoolIndexed<T> = ArrayWrapped<T, 7>;

impl<T> Index<DamageSchool> for SchoolIndexed<T>
where
    T: DekuWrite,
{
    type Output = T;

    fn index(&self, index: DamageSchool) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<DamageSchool> for SchoolIndexed<T>
where
    T: DekuWrite,
{
    fn index_mut(&mut self, index: DamageSchool) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
