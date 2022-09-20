use crate::guid::Guid;

use deku::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;

pub mod container;
pub mod corpse;
pub mod dynamic_object;
pub mod game_object;
pub mod item;
pub mod player;
pub mod unit;
mod utils;

use crate::position::Vector3d;

pub mod traits {
    pub use super::container::*;
    pub use super::corpse::*;
    pub use super::dynamic_object::*;
    pub use super::game_object::*;
    pub use super::item::*;
    pub use super::player::*;
    pub use super::unit::*;
    pub use super::ObjectTrait;
}

use crate::objects::private::Storage;

type ObjectInner = BTreeMap<usize, u32>;
pub trait UpdateFields:
    DekuUpdate + DekuContainerRead<'static> + DekuContainerWrite + Debug + Sync + Send
{
}

impl UpdateFields for Object {}
impl UpdateFields for Container {}
impl UpdateFields for Corpse {}
impl UpdateFields for GameObject {}
impl UpdateFields for Item {}
impl UpdateFields for Player {}
impl UpdateFields for Unit {}

pub enum UpdateMaskObjectType {
    Object = 0x0001,
    Item = 0x0002 | 0x0001,
    Container = 0x0004 | 0x0002 | 0x0001,
    Unit = 0x0008 | 0x0001,
    Player = 0x0010 | 0x0008 | 0x0001,
    GameObject = 0x0020 | 0x0001,
    DynamicObject = 0x0040 | 0x0001,
    Corpse = 0x0080 | 0x0001,
}

mod private {
    use crate::guid::{Guid, HighGuid};
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
                    i + offset + OFFSET,
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
            self.set_value(guid, 0x0000)
        }

        fn set_object_type(&mut self, object_type: u32) -> &mut Self {
            self.set_value(object_type, 0x0002)
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

pub trait ObjectTrait: private::Object<0x0000> {
    fn new(guid: Guid, object_type: UpdateMaskObjectType) -> Box<Self> {
        let mut object = Self::default();
        object.set_guid(guid).set_object_type(object_type as u32);
        Box::new(object)
    }

    fn get_guid(&self) -> Option<Guid> {
        self.get_value(0x0000)
    }
    fn get_object_type(&self) -> Option<u32> {
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
pub struct Object {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
impl Storage for Object {
    fn get_inner(&self) -> &ObjectInner {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut ObjectInner {
        &mut self.inner
    }
}
impl private::Object<0x0000> for Object {}
impl ObjectTrait for Object {}

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Item {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
impl Storage for Item {
    fn get_inner(&self) -> &ObjectInner {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut ObjectInner {
        &mut self.inner
    }
}
impl private::Object<0x0000> for Item {}
impl private::Object<0x0006> for Item {}
impl ObjectTrait for Item {}
impl item::Item for Item {}

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Container {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
impl Storage for Container {
    fn get_inner(&self) -> &ObjectInner {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut ObjectInner {
        &mut self.inner
    }
}
impl private::Object<0x0000> for Container {}
impl private::Object<0x0006> for Container {}
impl private::Object<0x0040> for Container {}
impl ObjectTrait for Container {}
impl item::Item for Container {}
impl container::Container for Container {}

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Unit {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
impl Storage for Unit {
    fn get_inner(&self) -> &ObjectInner {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut ObjectInner {
        &mut self.inner
    }
}
impl private::Object<0x0000> for Unit {}
impl private::Object<0x0006> for Unit {}
impl ObjectTrait for Unit {}
impl unit::Unit for Unit {}

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Player {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
impl Storage for Player {
    fn get_inner(&self) -> &ObjectInner {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut ObjectInner {
        &mut self.inner
    }
}
impl private::Object<0x0000> for Player {}
impl private::Object<0x0006> for Player {}
impl private::Object<0x0094> for Player {}
impl ObjectTrait for Player {}
impl unit::Unit for Player {}
impl player::Player for Player {}

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct GameObject {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
impl Storage for GameObject {
    fn get_inner(&self) -> &ObjectInner {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut ObjectInner {
        &mut self.inner
    }
}
impl private::Object<0x0000> for GameObject {}
impl private::Object<0x0006> for GameObject {}
impl ObjectTrait for GameObject {}
impl game_object::GameObject for GameObject {}

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct DynamicObject {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
impl Storage for DynamicObject {
    fn get_inner(&self) -> &ObjectInner {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut ObjectInner {
        &mut self.inner
    }
}
impl private::Object<0x0000> for DynamicObject {}
impl private::Object<0x0006> for DynamicObject {}
impl ObjectTrait for DynamicObject {}
impl dynamic_object::DynamicObject for DynamicObject {}

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Corpse {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: ObjectInner,
}
impl Storage for Corpse {
    fn get_inner(&self) -> &ObjectInner {
        &self.inner
    }
    fn get_inner_mut(&mut self) -> &mut ObjectInner {
        &mut self.inner
    }
}
impl private::Object<0x0000> for Corpse {}
impl private::Object<0x0006> for Corpse {}
impl ObjectTrait for Corpse {}
impl corpse::Corpse for Corpse {}

#[cfg(test)]
mod tests {
    use super::traits::*;
    use crate::class::Class;
    use crate::gender::Gender;
    use crate::guid::{Guid, HighGuid};
    use crate::objects::UpdateMaskObjectType;
    use crate::objects::UpdateMaskObjectType::Player;
    use crate::power::Power;
    use crate::race::Race;
    use deku::prelude::*;

    #[test]
    fn test_basic_player() {
        let mut player =
            super::Player::new(Guid::new(HighGuid::Player, 4), UpdateMaskObjectType::Player);
        player
            .set_unit_unit_data(UnitData {
                race: Race::Human,
                class: Class::Warrior,
                gender: Gender::Female,
                power: Power::Rage,
            })
            .set_object_scale_x(1.0)
            .set_unit_health(100)
            .set_unit_max_health(100)
            .set_unit_level(1)
            .set_unit_faction_template(1)
            .set_unit_display_id(50)
            .set_unit_native_display_id(50);

        let player_bytes = player.to_bytes().unwrap();
        let expected = [
            3, 23, 0, 128, 1, 1, 0, 192, 0, 24, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0,
            128, 63, 1, 1, 1, 1, 100, 0, 0, 0, 100, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 50, 0, 0, 0,
            50, 0, 0, 0,
        ];
        assert_eq!(player_bytes, expected);
        let loaded = Box::new(super::Player::from_bytes((&player_bytes, 0)).unwrap().1);
        assert_eq!(loaded, player)
    }
}
