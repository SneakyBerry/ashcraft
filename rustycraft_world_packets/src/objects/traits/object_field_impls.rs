use crate::guid::Guid;
use crate::object::ObjectType;
use crate::objects::game_object::GameObjectBytes;
use crate::objects::item::ItemEnchantment;
use crate::objects::player::{
    Bytes1, Bytes2, Bytes3, Bytes4, PlayerEnchantment, PlayerFieldBytes2Offsets, QuestLogItem,
    Rune, VisibleItem,
};
use crate::objects::traits::ObjectField;
use crate::objects::unit::{AttackPower, ClassSpecific, UnitData};
use crate::position::Vector3d;

macro_rules! object_field_array {
    ( $ ( $n:expr ), + ) => {
        $ (
            impl<T> ObjectField for [T; $n]
            where
                T: ObjectField,
            {
                const SIZE: u16 = $n * T::SIZE;
            }
        ) *
    };
}

object_field_array!(2, 3, 5, 6, 7, 12, 16, 18, 19, 21, 23, 24, 25, 29, 32, 36, 64, 128, 384);

impl ObjectField for u32 {
    const SIZE: u16 = 1;
}

impl ObjectField for i32 {
    const SIZE: u16 = 1;
}

impl ObjectField for bool {
    const SIZE: u16 = 1;
}

impl ObjectField for () {
    const SIZE: u16 = 1;
}

impl ObjectField for f32 {
    const SIZE: u16 = 1;
}

impl ObjectField for u64 {
    const SIZE: u16 = 2;
}

impl ObjectField for (u8, u8, u8, u8) {
    const SIZE: u16 = 1;
}

impl ObjectField for (u8, u8, u16) {
    const SIZE: u16 = 1;
}

impl ObjectField for (u16, u8, u8) {
    const SIZE: u16 = 1;
}

impl ObjectField for (u16, u16) {
    const SIZE: u16 = 1;
}

impl ObjectField for Guid {
    const SIZE: u16 = 2;
}

impl ObjectField for ObjectType {
    const SIZE: u16 = 1;
}

impl ObjectField for Vector3d {
    const SIZE: u16 = 4;
}

impl ObjectField for GameObjectBytes {
    const SIZE: u16 = 1;
}

impl ObjectField for ItemEnchantment {
    const SIZE: u16 = 3;
}

impl ObjectField for UnitData {
    const SIZE: u16 = 1;
}

impl ObjectField for ClassSpecific {
    const SIZE: u16 = 1;
}

impl ObjectField for AttackPower {
    const SIZE: u16 = 3;
}

// impl ObjectField for Bytes1 {
//     const SIZE: u16 = 1;
// }
//
// impl ObjectField for Bytes2 {
//     const SIZE: u16 = 1;
// }
//
// impl ObjectField for Bytes3 {
//     const SIZE: u16 = 1;
// }
//
// impl ObjectField for Bytes4 {
//     const SIZE: u16 = 1;
// }
//
// impl ObjectField for QuestLogItem {
//     const SIZE: u16 = 4;
// }
//
// impl ObjectField for VisibleItem {
//     const SIZE: u16 = 3;
// }
//
// impl ObjectField for PlayerFieldBytes2Offsets {
//     const SIZE: u16 = 1;
// }
//
// impl ObjectField for Rune {
//     const SIZE: u16 = 4;
// }
//
// impl ObjectField for PlayerEnchantment {
//     const SIZE: u16 = 2;
// }
//
// impl<T, TT> ObjectField for (T, TT)
// where
//     T: ObjectField,
//     TT: ObjectField,
// {
//     const SIZE: u16 = T::SIZE + TT::SIZE;
// }
