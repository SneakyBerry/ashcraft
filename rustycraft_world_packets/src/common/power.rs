use deku::prelude::*;
use std::ops::{Index, IndexMut};
use crate::common::helpers::ArrayWrapped;
use crate::power::Power;

pub type PowerIndexed<T> = ArrayWrapped<T, 7>;

impl<T> Index<Power> for PowerIndexed<T>
where
    T: DekuWrite,
{
    type Output = T;

    fn index(&self, index: Power) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<Power> for PowerIndexed<T>
where
    T: DekuWrite,
{
    fn index_mut(&mut self, index: Power) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
