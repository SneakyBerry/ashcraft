use crate::objects::size_helper::FieldSize;
use crate::objects::UpdateFields;
use deku::prelude::*;
use std::ops::{Index, IndexMut};
use crate::objects::calc_update::CalcUpdate;

pub enum Stats {
    Strength = 0,
    Agility = 1,
    Stamina = 2,
    Intellect = 3,
    Spirit = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, DekuWrite)]
pub struct StatsIndexed<T>(pub [T; 5])
where
    T: DekuWrite;

impl<T> Default for StatsIndexed<T>
where
    T: DekuWrite + Default + Copy,
{
    fn default() -> Self {
        Self([T::default(); 5])
    }
}

impl<T> Index<Stats> for StatsIndexed<T>
where
    T: DekuWrite,
{
    type Output = T;

    fn index(&self, index: Stats) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<Stats> for StatsIndexed<T>
where
    T: DekuWrite,
{
    fn index_mut(&mut self, index: Stats) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl<T, const BASE_OFFSET: usize> CalcUpdate<BASE_OFFSET> for StatsIndexed<T>
where
    T: PartialEq + DekuWrite + FieldSize + Default + Copy,
{
    fn get_diff(&self, old: Option<&Self>) -> UpdateFields {
        <[T; 5] as CalcUpdate<BASE_OFFSET>>::get_diff(&self.0, old.map(|o| &o.0))
    }
}
