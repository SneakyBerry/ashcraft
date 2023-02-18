use deku::prelude::*;
use std::ops::{Index, IndexMut};
use crate::common::helpers::ArrayWrapped;

pub enum Stats {
    Strength = 0,
    Agility = 1,
    Stamina = 2,
    Intellect = 3,
    Spirit = 4,
}


pub type StatsIndexed<T> = ArrayWrapped<T, 5>;

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
