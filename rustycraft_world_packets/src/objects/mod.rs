use crate::guid::Guid;
use crate::object::ObjectType;
use deku::bitvec::{BitVec, Lsb0};
use deku::prelude::*;
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub(crate) mod container;
pub(crate) mod corpse;
pub(crate) mod dynamic_object;
pub(crate) mod game_object;
pub(crate) mod item;
pub(crate) mod object;
pub(crate) mod player;
pub(crate) mod traits;
pub(crate) mod unit;
pub(crate) mod utils;

pub(crate) type ObjectInner = BTreeMap<u16, [u8; 4]>;

macro_rules! make_field {
    ( $global_offset:expr => $struct_name:tt => $start:expr => $name:ident(()) ) => {
        // Padding
        paste! {const [<_$global_offset _ $start>]: () = ();}
    };
    ( $global_offset:expr => $struct_name:tt => $start:expr => $name:ident(bool) ) => {
        impl $struct_name {
            paste! {const [<_$global_offset _ $start>]: () = ();}
            paste! {
                pub fn [<set_ $name>](&mut self, bit: u8) -> &mut Self {
                    todo!()
                    // use $crate::objects::traits::Storable;
                    // let value = <$ty as Storable>::store($name);
                    // self.set_value(value.as_slice(), $global_offset + $start)
                }

                pub fn [<unset_ $name>](&mut self, bit: u8) -> &mut Self {
                    todo!()
                    // use $crate::objects::traits::Storable;
                    // let value = <$ty as Storable>::store($name);
                    // self.set_value(value.as_slice(), $global_offset + $start)
                }

                pub fn [<apply_mask_ $name>](&mut self, mask: u32) -> &mut Self {
                    todo!()
                    // use $crate::objects::traits::Storable;
                    // let value = <$ty as Storable>::store($name);
                    // self.set_value(value.as_slice(), $global_offset + $start)
                }

                pub fn [<get_ $name>](&self, bit: u8) -> Option<bool> {
                    todo!()
                    // use $crate::objects::traits::Storable;
                    // let vals = self.get_value($global_offset + $start, $end - $start)?;
                    // Some(<$ty as Storable>::load(vals.as_slice().try_into().unwrap()))
                }
            }
        }
    };
    ( $global_offset:expr => $struct_name:tt => $start:expr => $name:ident([$ty:ty; $size:expr]) ) => {
        impl $struct_name {
            paste! {const [<_$global_offset _ $start>]: () = ();}
            paste! {
                pub fn [<set_ $name>](&mut self, $name: $ty, index: u16) -> &mut Self {
                    use $crate::objects::traits::Storable;
                    let value = <$ty as Storable>::store($name);
                    self.set_value(value.as_slice(), $global_offset + $start + index)
                }

                pub fn [<get_ $name>](&self, index: u16) -> Option<$ty> {
                    use $crate::objects::traits::Storable;
                    use crate::objects::traits::ObjectField;
                    let vals = self.get_value($global_offset + $start, <[$ty; $size]>::SIZE / $size + index)?;
                    Some(<$ty as Storable>::load(vals.as_slice().try_into().unwrap()))
                }
            }
        }
    };
    ( $global_offset:expr => $struct_name:tt => $start:expr => $name:ident($ty:ty) ) => {
        impl $struct_name {
            paste! {const [<_$global_offset _ $start>]: () = ();}
            paste! {
                pub fn [<set_ $name>](&mut self, $name: $ty) -> &mut Self {
                    use $crate::objects::traits::Storable;
                    let value = <$ty as Storable>::store($name);
                    self.set_value(value.as_slice(), $global_offset + $start)
                }

                pub fn [<get_ $name>](&self) -> Option<$ty> {
                    use $crate::objects::traits::Storable;
                    use crate::objects::traits::ObjectField;
                    let vals = self.get_value($global_offset + $start, <$ty>::SIZE)?;
                    Some(<$ty as Storable>::load(vals.as_slice().try_into().unwrap()))
                }
            }
        }
    };
}
macro_rules! impl_base {
    ( impl Base for $struct_name:ident; ) => {
        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }
            fn get_value(&self, offset: u16, size: u16) -> Option<Vec<[u8; 4]>> {
                let range = self
                    .inner
                    .range(offset..offset + size)
                    .map(|(k, v)| *v)
                    .collect::<Vec<_>>();
                if range.len() < size as usize {
                    None
                } else {
                    Some(range)
                }
            }

            fn set_value(&mut self, value: &[[u8; 4]], offset: u16) -> &mut Self {
                for (value_offset, value) in value.iter().enumerate() {
                    self.inner.insert(offset + value_offset as u16, *value);
                }
                self
            }
        }
    };
}

macro_rules! impl_accessors {
    (
        Offset: $offset:expr;
        Size: $size:expr;
        impl $struct_name:path {
            $ ( $start:expr => $name:ident: $type:tt; ) *
        }
    ) => {
        $ (
            make_field!($offset => $struct_name => $start => $name($type));
        ) *

        // Static assert alignment
        const _: () = {
            use crate::objects::traits::ObjectField;
            let starts = [$( $start, )* $size];
            let i = 1;
            $(
            if starts[i] != $start + <$type>::SIZE {
               panic!("Bad fields mapping")
            };
            let i = i + 1;
            )*;
        };
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::guid::{Guid, HighGuid};
    use crate::objects::game_object::GameObjectBytes;
    use crate::objects::item::ItemEnchantment;
    use crate::objects::player::*;
    use crate::objects::unit::*;
    use crate::position::Vector3d;
    use crate::{
        container_fields, corpse_fields, dynamic_object_fields, game_object_fields, item_fields,
        object_fields, player_fields, unit_fields,
    };
    use deku::prelude::*;
    use std::mem::size_of;

    const OBJECT_BIT_VEC_SIZE: usize = 0x0006;
    const ITEM_BIT_VEC_SIZE: usize = OBJECT_BIT_VEC_SIZE + 0x003A;
    const CONTAINER_BIT_VEC_SIZE: usize = ITEM_BIT_VEC_SIZE + 0x004A;
    const UNIT_BIT_VEC_SIZE: usize = OBJECT_BIT_VEC_SIZE + 0x008E;
    const PLAYER_BIT_VEC_SIZE: usize = UNIT_BIT_VEC_SIZE + 0x049A;
    const GAME_OBJECT_BIT_VEC_SIZE: usize = OBJECT_BIT_VEC_SIZE + 0x000C;
    const DYNAMIC_OBJECT_BIT_VEC_SIZE: usize = OBJECT_BIT_VEC_SIZE + 0x0006;
    const CORPSE_BIT_VEC_SIZE: usize = OBJECT_BIT_VEC_SIZE + 0x001E;

    #[derive(Debug, Clone, Default, deku::prelude::DekuRead, deku::prelude::DekuWrite)]
    struct TestStruct {
        #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
        #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, inner)")]
        inner: crate::objects::ObjectInner,
    }

    impl_base!(impl Base for TestStruct;);

    object_fields!(
        Offset: 0x0000;
        impl Object for TestStruct
    );

    container_fields!(
        Offset: 0x1000;
        impl Container for TestStruct
    );

    corpse_fields!(
        Offset: 0x2000;
        impl Corpse for TestStruct
    );

    dynamic_object_fields!(
        Offset: 0x3000;
        impl DynamicObject for TestStruct
    );

    game_object_fields!(
        Offset: 0x4000;
        impl GameObject for TestStruct
    );

    item_fields!(
        Offset: 0x5000;
        impl Item for TestStruct
    );

    unit_fields!(
        Offset: 0x6000;
        impl Unit for TestStruct
    );

    // player_fields!(
    //     Offset: 0x7000;
    //     impl Player for TestStruct
    // );

    impl_accessors!(
        Offset: 0x0;
        Size: 0x0006;
        impl TestStruct {
            0x0000 => guid_1: Guid;
            0x0002 => f_32_1: f32;
            0x0003 => u_32_1: u32;
            0x0004 => u_64_1: u64;
        }
    );

    impl_accessors!(
        Offset: 0x0006;
        Size: 0x0006;
        impl TestStruct {
            0x0000 => guid_2: Guid;
            0x0002 => f_32_2: f32;
            0x0003 => u_32_2: u32;
            0x0004 => u_64_2: u64;
        }
    );

    #[test]
    fn test_object_construct() {
        let mut test = TestStruct::new();

        let guid_1 = Guid::new(HighGuid::Item, 0);
        let f_32_1 = 1.23456;
        let u_32_1 = 1010;
        let u_64_1 = 2020;

        let guid_2 = Guid::new(HighGuid::Corpse, 2);
        let f_32_2 = 2.23456;
        let u_32_2 = 2020;
        let u_64_2 = 3030;
        test.set_guid_1(guid_1)
            .set_f_32_1(f_32_1)
            .set_u_32_1(u_32_1)
            .set_u_64_1(u_64_1)
            .set_guid_2(guid_2)
            .set_f_32_2(f_32_2)
            .set_u_32_2(u_32_2)
            .set_u_64_2(u_64_2);
        println!("{:?}", &test);
        assert_eq!(test.get_guid_1(), Some(guid_1));
        assert_eq!(test.get_f_32_1(), Some(f_32_1));
        assert_eq!(test.get_u_32_1(), Some(u_32_1));
        assert_eq!(test.get_u_64_1(), Some(u_64_1));
        assert_eq!(test.get_guid_2(), Some(guid_2));
        assert_eq!(test.get_f_32_2(), Some(f_32_2));
        assert_eq!(test.get_u_32_2(), Some(u_32_2));
        assert_eq!(test.get_u_64_2(), Some(u_64_2));
    }

    // #[test]
    // fn test_object_size() {
    //     let obj = super::Object::default().get_bit_mask();
    //     assert_eq!(obj.len(), OBJECT_BIT_VEC_SIZE)
    // }
    // #[test]
    // fn test_item_size() {
    //     let obj = super::Item::default().get_bit_mask();
    //     assert_eq!(obj.len(), ITEM_BIT_VEC_SIZE)
    // }
    // #[test]
    // fn test_container_size() {
    //     let obj = super::Container::default().get_bit_mask();
    //     assert_eq!(obj.len(), CONTAINER_BIT_VEC_SIZE)
    // }
    // #[test]
    // fn test_unit_size() {
    //     let obj = super::Unit::default().get_bit_mask();
    //     assert_eq!(obj.len(), UNIT_BIT_VEC_SIZE)
    // }
    // #[test]
    // fn test_player_size() {
    //     let obj = super::Player::default().get_bit_mask();
    //     assert_eq!(obj.len(), PLAYER_BIT_VEC_SIZE)
    // }
    // #[test]
    // fn test_game_object_size() {
    //     let obj = super::GameObject::default().get_bit_mask();
    //     assert_eq!(obj.len(), GAME_OBJECT_BIT_VEC_SIZE)
    // }
    // #[test]
    // fn test_dynamic_object_size() {
    //     let obj = super::DynamicObject::default().get_bit_mask();
    //     assert_eq!(obj.len(), DYNAMIC_OBJECT_BIT_VEC_SIZE)
    // }
    //
    // #[test]
    // fn test_corpse_size() {
    //     let obj = super::Corpse::default().get_bit_mask();
    //     assert_eq!(obj.len(), CORPSE_BIT_VEC_SIZE)
    // }
}
