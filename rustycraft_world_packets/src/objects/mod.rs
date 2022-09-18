use crate::guid::Guid;
use crate::object::ObjectType;
use deku::bitvec::{BitVec, Lsb0};
use deku::prelude::*;
use std::collections::BTreeMap;
use std::marker::PhantomData;

mod container;
mod corpse;
mod dynamic_object;
mod game_object;
mod item;
mod object;
mod player;
mod traits;
mod unit;
mod utils;

type ObjectInner = BTreeMap<u16, [u8; 4]>;

#[macro_use]
mod macroses {
    macro_rules! make_field {
        ( $global_offset:expr => $struct_name:tt => $start:expr => $name:ident(()) ) => {
            // Padding
            paste! {const [<_$global_offset _ $start>]: () = ();}
        };
        ( $global_offset:expr => $struct_name:tt => $start:expr => $name:ident([bool; $size:expr]) ) => {
            impl $struct_name {
                paste! {const [<_$global_offset _ $start>]: () = ();}
                paste! {
                    pub fn [<set_ $name>](&mut self, index: u16) -> &mut Self {
                        use $crate::objects::traits::ObjectField;
                        if index >= $size * 32 {
                            panic!("Index is out of range");
                        }
                        else {
                            let place_index = index / 32;
                            let byte_index = (index % 4) as usize;
                            let bit_index = index % 8;
                            let mut val = self.get_value($global_offset + $start..$global_offset + $start + <[bool; $size]>::SIZE / $size + place_index).unwrap_or(vec![[0u8; 4]])[0];
                            val[byte_index] |= (1u8 << (7 - bit_index));
                            self.set_value(&[val], $global_offset + $start + place_index)
                        }
                    }

                    pub fn [<unset_ $name>](&mut self, index: u16) -> &mut Self {
                        use $crate::objects::traits::ObjectField;
                        if index >= $size * 32 {
                            panic!("Index is out of range");
                        }
                        else {
                            let place_index = index / 32;
                            let byte_index = (index % 4) as usize;
                            let bit_index = index % 8;
                            let mut val = self.get_value($global_offset + $start..$global_offset + $start + <[bool; $size]>::SIZE / $size + place_index).unwrap_or(vec![[0u8; 4]])[0];
                            val[byte_index] &= !(1u8 << (7 - bit_index));
                            self.set_value(&[val], $global_offset + $start + place_index)
                        }
                    }

                    pub fn [<get_ $name>](&self, index: u16) -> Option<bool> {
                        use $crate::objects::traits::ObjectField;
                        if index >= $size * 32 {
                            panic!("Index is out of range");
                        }
                        else {
                            let place_index = index / 32;
                            let byte_index = (index % 4) as usize;
                            let bit_index = index % 8;
                            let val = self.get_value($global_offset + $start..$global_offset + $start + <[bool; $size]>::SIZE / $size + place_index)?[0][byte_index];
                            Some(val & (1u8 << (7 - bit_index)) != 0)
                        }
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
                        if index >= $size {
                            panic!("Index is out of range");
                        }
                        let value = Storable::store($name);
                        self.set_value(value.as_slice(), $global_offset + $start + index)
                    }

                    pub fn [<get_ $name>](&self, index: u16) -> Option<$ty> {
                        use $crate::objects::traits::Storable;
                        use $crate::objects::traits::ObjectField;
                        if index >= $size {
                            panic!("Index is out of range");
                        }
                        let vals = self.get_value($global_offset + $start..$global_offset + $start + <[$ty; $size]>::SIZE / $size + index)?;
                        Some(Storable::load(vals.as_slice().try_into().unwrap()))
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
                        let value = Storable::store($name);
                        self.set_value(value.as_slice(), $global_offset + $start)
                    }

                    pub fn [<get_ $name>](&self) -> Option<$ty> {
                        use $crate::objects::traits::Storable;
                        use $crate::objects::traits::ObjectField;
                        let vals = self.get_value($global_offset + $start..$global_offset + $start + <$ty>::SIZE)?;
                        Some(Storable::load(vals.as_slice().try_into().unwrap()))
                    }
                }
            }
        };
    }

    macro_rules! impl_base {
        ( impl Base for $struct_name:ident ) => {
            impl $struct_name {
                pub fn new() -> Self {
                    Self::default()
                }
                fn get_value(&self, range: std::ops::Range<u16>) -> Option<Vec<[u8; 4]>> {
                    if range.is_empty() {
                        panic!("Incorrect range requested, range: {range:?}");
                    }
                    let res = self
                        .inner
                        .range(range.clone())
                        .map(|(k, v)| *v)
                        .collect::<Vec<_>>();
                    if res.len() < range.len() {
                        None
                    } else {
                        Some(res)
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

            // Static alignment check
            const _: () = {
                use $crate::objects::traits::ObjectField;
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::guid::{Guid, HighGuid};
    use crate::objects::game_object::GameObjectBytes;
    use crate::objects::item::ItemEnchantment;
    use crate::objects::player::*;
    use crate::objects::unit::*;
    use crate::objects::*;
    use crate::position::Vector3d;
    use crate::{
        container_fields, corpse_fields, dynamic_object_fields, game_object_fields, item_fields,
        object_fields, player_fields, unit_fields,
    };
    use deku::prelude::*;
    use std::mem::size_of;

    #[test]
    fn test_object_construct() {
        #[derive(Debug, Clone, Default)]
        struct Test {
            inner: ObjectInner,
        }

        impl_base!(impl Base for Test);
        impl_accessors!(
        Offset: 0x0;
        Size: 0x0007;
        impl Test {
            0x0000 => guid_1: Guid;
            0x0002 => f_32_1: f32;
            0x0003 => u_32_1: u32;
            0x0004 => i_32_1: i32;
            0x0005 => u_64_1: u64;
            }
        );

        impl_accessors!(
        Offset: 0x0007;
        Size: 0x0007;
        impl Test {
            0x0000 => guid_2: Guid;
            0x0002 => f_32_2: f32;
            0x0003 => u_32_2: u32;
            0x0004 => i_32_2: i32;
            0x0005 => u_64_2: u64;
            }
        );

        let mut test = Test::new();

        let guid_1 = Guid::new(HighGuid::Item, 0);
        let f_32_1 = 1.23456;
        let u_32_1 = 1010;
        let u_64_1 = 2020;

        let guid_2 = Guid::new(HighGuid::Corpse, 2);
        let f_32_2 = 2.23456;
        let u_32_2 = 2020;
        let i_32_2 = -2020;
        let u_64_2 = 3030;
        test.set_guid_1(guid_1)
            .set_f_32_1(f_32_1)
            .set_u_32_1(u_32_1)
            .set_u_64_1(u_64_1)
            .set_guid_2(guid_2)
            .set_f_32_2(f_32_2)
            .set_u_32_2(u_32_2)
            .set_i_32_2(i_32_2)
            .set_u_64_2(u_64_2);
        assert_eq!(test.get_guid_1(), Some(guid_1));
        assert_eq!(test.get_f_32_1(), Some(f_32_1));
        assert_eq!(test.get_u_32_1(), Some(u_32_1));
        assert_eq!(test.get_i_32_1(), None);
        assert_eq!(test.get_u_64_1(), Some(u_64_1));
        assert_eq!(test.get_guid_2(), Some(guid_2));
        assert_eq!(test.get_f_32_2(), Some(f_32_2));
        assert_eq!(test.get_u_32_2(), Some(u_32_2));
        assert_eq!(test.get_i_32_2(), Some(i_32_2));
        assert_eq!(test.get_u_64_2(), Some(u_64_2));
    }

    #[derive(Default, Debug)]
    struct Test {
        inner: ObjectInner,
    }

    impl_base!(impl Base for Test);
    impl_accessors!(
        Offset: 0x0;
        Size: 0x0004;
        impl Test {
            0x0000 => mask: [bool;2];
            0x0002 => u32_array: [u32;2];
            }
    );

    #[test]
    fn test_bool_set() {
        let mut test = Test::default();

        assert_eq!(test.get_mask(0), None);
        test.set_mask(0);
        assert_eq!(test.get_mask(0), Some(true));
        assert_eq!(test.get_mask(1), Some(false));
        test.unset_mask(0);
        assert_eq!(test.get_mask(0), Some(false));
        assert_eq!(test.get_mask(31), Some(false));
        assert_eq!(test.get_mask(63), None);
    }

    #[test]
    fn test_get_arr() {
        let mut test = Test::default();
        println!("{:?}", &test.get_u32_array(0));
    }

    #[test]
    #[should_panic(expected = "Index is out of range")]
    fn test_get_out_of_range_bool() {
        let mut test = Test::default();
        test.get_mask(64);
    }
    #[test]
    #[should_panic(expected = "Index is out of range")]
    fn test_get_out_of_range_arr() {
        let mut test = Test::default();
        test.get_u32_array(2);
    }
}
