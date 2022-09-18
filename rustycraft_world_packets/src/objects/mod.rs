//! Possible object types:
//!
//! pub enum ObjectType {
//!     Object = 0x0,
//!     Item = 0x1,
//!     Container = 0x2,
//!     Unit = 0x3,
//!     Player = 0x4,
//!     GameObject = 0x5,
//!     DynamicObject = 0x6,
//!     Corpse = 0x7,
//! }

use crate::guid::Guid;

use deku::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;

#[macro_use]
mod container;
#[macro_use]
mod corpse;
#[macro_use]
mod dynamic_object;
#[macro_use]
mod game_object;
#[macro_use]
mod item;
#[macro_use]
mod player;
#[macro_use]
mod unit;
mod utils;

#[macro_use]
mod macros;

use crate::object::ObjectType;
use crate::position::Vector3d;

use game_object::*;
use item::*;

pub use player::*;
pub use unit::*;

type ObjectInner = BTreeMap<usize, u32>;
pub trait UpdateFields:
    DekuUpdate + DekuContainerRead<'static> + DekuContainerWrite + Debug
{
}
mod private {
    use crate::guid::{Guid, HighGuid};
    use crate::object::ObjectType;
    use crate::objects::ObjectInner;
    use deku::bitvec::{BitVec, Msb0};
    use deku::ctx::Endian;
    use deku::{DekuContainerRead, DekuContainerWrite, DekuRead, DekuWrite};
    use std::any::type_name;

    pub trait Storage {
        fn get_inner(&self) -> &ObjectInner;
        fn get_inner_mut(&mut self) -> &mut ObjectInner;
    }
    pub trait Object<const OFFSET: usize>: Storage + Default {
        fn get_value<T>(&self, offset: usize) -> Option<T>
        where
            T: for<'a> DekuRead<'a>,
        {
            let values = BitVec::from_iter(
                self.get_inner()
                    .range(OFFSET + offset..OFFSET + offset + std::mem::size_of::<T>() / 16)
                    .map(|(_, v)| v.to_le_bytes())
                    .flatten(),
            );
            Some(T::read(&values, ()).expect("Parse value failed").1)
        }

        fn set_value<T>(&mut self, value: T, offset: usize) -> &mut Self
        where
            T: DekuWrite,
        {
            let mut inner = self.get_inner_mut();
            let mut buffer = BitVec::new();
            value.write(&mut buffer, ()).expect("Write failed");
            inner.extend(buffer.as_raw_slice().chunks(4).enumerate().map(|(i, v)| {
                (
                    i + offset,
                    u32::from_le_bytes(v.try_into().expect("Chunk not equal to 4?")),
                )
            }));
            self
        }

        fn apply_xor(&mut self, offset: usize, mask: u32) -> &mut Self {
            let mut inner = self.get_inner_mut();
            let mut current = *inner.get(&(OFFSET + offset)).unwrap_or(&0);
            inner.insert(OFFSET + offset, current ^ mask);
            self
        }

        fn apply_or(&mut self, offset: usize, mask: u32) -> &mut Self {
            let mut inner = self.get_inner_mut();
            let mut current = *inner.get(&(OFFSET + offset)).unwrap_or(&0);
            inner.insert(OFFSET + offset, current | mask);
            self
        }

        fn apply_and(&mut self, offset: usize, mask: u32) -> &mut Self {
            let mut inner = self.get_inner_mut();
            let mut current = *inner.get(&(OFFSET + offset)).unwrap_or(&0);
            inner.insert(OFFSET + offset, current | mask);
            self
        }

        fn set_guid(&mut self, guid: Guid) -> &mut Self {
            self.set_value(guid.as_u32(), 0x0000)
        }

        fn set_object_type(&mut self, object_type: ObjectType) -> &mut Self {
            self.set_value([object_type as u32], 0x0002)
        }

        fn parse_guid(val: [u32; 2]) -> Guid {
            let u_val = val[0] as u64 | (val[1] as u64 >> 32);
            let high = HighGuid::from_bytes((&u_val.to_le_bytes(), 0))
                .expect(&format!("Guid parse failed: {:X}", u_val))
                .1;
            Guid::new(high, u_val)
        }
    }
}

pub trait Object: private::Object<0x0000> {
    fn new(guid: Guid) -> Box<Self> {
        let mut object = Self::default();
        object.set_guid(guid).set_object_type(ObjectType::Object);
        Box::new(object)
    }

    fn get_guid(&self) -> Option<Guid> {
        self.get_value(0x0000)
    }
    fn get_object_type(&self) -> Option<ObjectType> {
        self.get_value(0x0002)
    }
    fn set_object_entry(&mut self, object_entry: u32) -> &mut Self {
        self.set_value(object_entry, 0x0003)
    }
    fn get_object_entry(&self) -> Option<u32> {
        self.get_value(0x0003)
    }
    fn set_object_scale_x(&mut self, object_scale_x: f32) -> &mut Self {
        self.set_value(object_scale_x, 0x0004)
    }
    fn get_object_scale_x(&self) -> Option<f32> {
        self.get_value(0x0004)
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Item {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
// item_fields!(impl for Item);

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Container {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
// object_fields!(ObjectType::Container => Container);
// container_fields!(impl for Container);

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Unit {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
// object_fields!(ObjectType::Unit => Unit);
// unit_fields!(impl for Unit);

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Player {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
// object_fields!(ObjectType::Player => Player);
// unit_fields!(impl for Player);
// player_fields!(impl for Player);

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct GameObject {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
// object_fields!(ObjectType::GameObject => GameObject);
// game_object_fields!(impl for GameObject);

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct DynamicObject {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
// object_fields!(ObjectType::DynamicObject => DynamicObject);
// dynamic_object_fields!(impl for DynamicObject);

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Corpse {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
// object_fields!(ObjectType::Corpse => Corpse);
// corpse_fields!(impl for Corpse);

#[cfg(test)]
mod test {
    use super::*;
    use crate::guid::{Guid, HighGuid};
    use crate::object::ObjectType;
    use crate::objects::item::Item;

    #[test]
    fn test_object_construct() {
        #[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
        struct Test {
            #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
            #[deku(
                writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)"
            )]
            inner: ObjectInner,
        }
        impl private::Storage for Test {
            fn get_inner(&self) -> &ObjectInner {
                &self.inner
            }

            fn get_inner_mut(&mut self) -> &mut ObjectInner {
                &mut self.inner
            }
        }
        impl Object for Test {};
        impl private::Object<0x0000> for Test {}

        impl Item for Test {};
        impl private::Object<0x0006> for Test {}

        let mut test = <Test as Item>::new(Guid::new(HighGuid::Corpse, 0xFFFF0000000000FF));
        test.set_flags(0xFF00FF00)
            .set_enchantment(
                ItemEnchantment {
                    id: 0xFFFEFFEF,
                    duration: u32::MAX,
                    charges: u32::MAX / 2,
                },
                1,
            )
            .set_enchantment(
                ItemEnchantment {
                    id: 0xFF0000EF,
                    duration: u32::MAX,
                    charges: u32::MAX / 2,
                },
                0,
            );
        debug_print_btree(&test.inner);
        let inn = test.inner.clone();
        let prom = test.to_bytes().unwrap();
        let restored = Test::from_bytes((&prom, 0)).unwrap().1;
        println!("{:?}", &prom);
        println!("{:?}", &test);
        println!("{:?}", &restored);
        debug_print_btree(&restored.inner);
    }
}

pub(crate) fn debug_print_btree(inner: &ObjectInner) {
    for (k, v) in inner {
        println!("{}: {:0>8X}", k, v);
    }
}
