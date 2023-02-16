use crate::objects::size_helper::FieldSize;
use array_init::array_init;
use deku::bitvec::{BitVec, Msb0};
use deku::prelude::*;
use deku::DekuWrite;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
pub struct ArrayWrapped<T, const N: usize>([T; N]);
impl<T, const N: usize> From<[T; N]> for ArrayWrapped<T, N> {
    fn from(value: [T; N]) -> Self {
        ArrayWrapped(value)
    }
}

impl<T, const N: usize> DekuWrite for ArrayWrapped<T, N>
where
    [T; N]: DekuWrite,
{
    fn write(&self, output: &mut BitVec<u8, Msb0>, ctx: ()) -> Result<(), DekuError> {
        self.0.write(output, ctx)
    }
}
impl<T, const N: usize> Default for ArrayWrapped<T, N>
where
    T: Default,
{
    fn default() -> Self {
        ArrayWrapped(array_init(|_| T::default()))
    }
}

impl<T, const N: usize> Debug for ArrayWrapped<T, N>
where
    [T; N]: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ArrayWrapped").field(&self.0).finish()
    }
}
impl<T, const N: usize> Clone for ArrayWrapped<T, N>
where
    [T; N]: Clone,
{
    fn clone(&self) -> Self {
        ArrayWrapped(self.0.clone())
    }
}
impl<T, const N: usize> Copy for ArrayWrapped<T, N> where [T; N]: Copy {}
impl<T, const N: usize> FieldSize for ArrayWrapped<T, N>
where
    [T; N]: FieldSize,
{
    const SIZE: usize = <[T; N]>::SIZE;
    const EL_SIZE: usize = <[T; N]>::EL_SIZE;
}

impl<T, const N: usize> Deref for ArrayWrapped<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for ArrayWrapped<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
