use deku::bitvec::BitVec;
use deku::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;

mod container;
mod corpse;
mod default;
mod dynamic_object;
mod game_object;
mod item;
mod object;
mod player;
pub(crate) mod size_helper;
mod unit;
mod utils;

pub mod prelude {
    pub use super::container::*;
    pub use super::corpse::*;
    pub use super::dynamic_object::*;
    pub use super::game_object::*;
    pub use super::item::*;
    pub use super::object::*;
    pub use super::player::*;
    pub use super::unit::*;
    pub use super::UpdateFields;
}
#[allow(dead_code)]
enum UpdateMaskObjectType {
    Object = 0x0001,
    Item = 0x0002 | 0x0001,
    Container = 0x0004 | 0x0002 | 0x0001,
    Unit = 0x0008 | 0x0001,
    Player = 0x0010 | 0x0008 | 0x0001,
    GameObject = 0x0020 | 0x0001,
    DynamicObject = 0x0040 | 0x0001,
    Corpse = 0x0080 | 0x0001,
}

type InnerState = BTreeMap<usize, u32>;

#[derive(Debug, Clone, Eq, PartialEq, DekuWrite)]
pub struct UpdateFields {
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: InnerState,
}

impl UpdateFields {
    pub(super) fn new() -> Self {
        Self {
            inner: InnerState::new(),
        }
    }
    pub(super) fn set_value<T, const BASE_OFFSET: usize>(
        &mut self,
        value: T,
        offset: usize,
    ) -> &mut Self
    where
        T: DekuWrite,
    {
        let mut buffer = BitVec::new();
        value.write(&mut buffer, ()).expect("Write failed");
        self.inner
            .extend(buffer.as_raw_slice().chunks(4).enumerate().map(|(i, v)| {
                (
                    BASE_OFFSET + i + offset,
                    u32::from_le_bytes(v.try_into().expect("Chunk not equal to 4?")),
                )
            }));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use crate::common::class::Class;
    use crate::common::gender::Gender;
    use crate::guid::Guid;

    use crate::guid;
    use crate::power::Power;
    use crate::race::Race;
    use deku::prelude::*;

    #[test]
    fn test_basic_player() {
        let player = Player {
            unit: Unit {
                object: Object {
                    guid: Some(Guid::Player(guid::Player::new(0, 4))),
                    scale_x: Some(1.0),
                    ..Default::default()
                },
                data: Some(UnitData {
                    race: Race::Human,
                    class: Class::Warrior,
                    gender: Gender::Female,
                    power: Power::Rage,
                }),
                health: Some(100),
                max_health: Some(100),
                level: Some(1),
                faction_template: Some(1),
                display_id: Some(50),
                native_display_id: Some(50),
                ..Default::default()
            },
            ..Default::default()
        };
        let player_bytes = UpdateFields::from(player).to_bytes().unwrap();
        let expected = [
            3, 23, 0, 128, 1, 1, 0, 192, 0, 24, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0,
            128, 63, 1, 1, 1, 1, 100, 0, 0, 0, 100, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 50, 0, 0, 0,
            50, 0, 0, 0,
        ];
        assert_eq!(player_bytes, expected);
    }
}
