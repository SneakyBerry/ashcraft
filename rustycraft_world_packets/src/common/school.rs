use deku::prelude::*;
use std::ops::{Index, IndexMut};

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

#[derive(Debug, Clone, Copy, PartialEq, DekuWrite)]
pub struct SchoolIndexed<T>(pub [T; 7])
where
    T: DekuWrite;

impl<T> Default for SchoolIndexed<T>
where
    T: DekuWrite + Default + Copy,
{
    fn default() -> Self {
        Self([T::default(); 7])
    }
}

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

impl<T> From<SchoolIndexed<T>> for [T; 7]
where
    T: DekuWrite,
{
    fn from(value: SchoolIndexed<T>) -> Self {
        value.0
    }
}
