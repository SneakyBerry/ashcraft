use crate::guid::Guid;
use crate::objects::game_object::GameObjectBytes;
use crate::objects::item::ItemEnchantment;
use crate::objects::{player, unit};
use crate::position::Vector3d;
use std::mem::size_of;

pub(super) trait FieldSize {
    const SIZE: usize;
}

macro_rules! impl_field_size {
    ($($ty:path),*) => {
        $(
            impl FieldSize for $ty {
                const SIZE: usize = size_of::<Self>() / 4;
            }
        )*
    };
    ($ty:path: $size:expr) => {
        impl FieldSize for $ty {
            const SIZE: usize = $size;
        }
    };
    (($($ty:ident),*)) => {
        impl<$($ty),*> FieldSize for ($($ty),*)
            where
            $(
                $ty: FieldSize,
            )*
        {
            const SIZE: usize = 0 $(+ $ty::SIZE) *;
        }
    };
}

impl FieldSize for [u8; 4] {
    const SIZE: usize = 1;
}

impl FieldSize for (u16, u16) {
    const SIZE: usize = 1;
}

impl FieldSize for (u8, u8, u16) {
    const SIZE: usize = 1;
}

impl FieldSize for (u16, u8, u8) {
    const SIZE: usize = 1;
}
impl_field_size!(GameObjectBytes: 1);
impl_field_size!(player::Bytes1: 1);
impl_field_size!(player::Bytes2: 1);
impl_field_size!(player::Bytes3: 1);
impl_field_size!(player::Bytes4: 1);
impl_field_size!(unit::UnitData: 1);
impl_field_size!(unit::ClassSpecific: 1);
impl_field_size!(player::PlayerFieldBytes2Offsets: 1);
impl_field_size!(ItemEnchantment: 2);
impl_field_size!(player::PlayerEnchantment: 2);
impl_field_size!(unit::AttackPower: 3);
impl_field_size!(Vector3d: 4);
impl_field_size!(player::Rune: 4);
impl_field_size!(player::QuestLogItem: 5);
impl_field_size!(u32, i32, f32, u64, Guid);
impl_field_size!((A, B));
impl_field_size!((A, B, C));
impl_field_size!((A, B, C, D));

impl<T> FieldSize for Option<T>
where
    T: FieldSize,
{
    const SIZE: usize = T::SIZE;
}

impl<T, const N: usize> FieldSize for [T; N]
where
    T: FieldSize,
{
    const SIZE: usize = T::SIZE * N;
}
