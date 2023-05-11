use deku::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;

pub(crate) mod calc_update;
mod container;
mod corpse;
mod dynamic_object;
mod game_object;
mod item;
mod object;
mod player;
pub(crate) mod size_helper;
mod unit;
mod utils;

pub mod prelude {
    pub use super::calc_update::*;
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
    pub fn new() -> Self {
        Self {
            inner: InnerState::new(),
        }
    }
    pub(super) fn set_value<const BASE_OFFSET: usize>(
        &mut self,
        value: &[u8],
        offset: usize,
    ) -> &mut Self {
        self.inner.extend(value.chunks(4).enumerate().map(|(i, v)| {
            (
                BASE_OFFSET + i + offset,
                u32::from_le_bytes(v.try_into().expect("Chunk not equal to 4?")),
            )
        }));
        self
    }

    pub(crate) fn update(&mut self, new: UpdateFields) {
        self.inner.extend(new.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use crate::common::class::Class;
    use crate::common::gender::Gender;
    use crate::guid::Guid;

    use crate::guid;
    use crate::objects::calc_update::CalcUpdate;
    use crate::power::Power;
    use crate::race::Race;
    use deku::prelude::*;

    #[test]
    fn test_basic_player() {
        let player = PlayerUpdate {
            unit: UnitUpdate {
                object: ObjectUpdate {
                    guid: Guid::Player(guid::PlayerGuid::new(0, 4)),
                    scale: 1.0,
                    ..Default::default()
                },
                data: UnitData {
                    race: Race::Human,
                    class: Class::Warrior,
                    gender: Gender::Female,
                    power: Power::Rage,
                },
                health: 100,
                max_health: 100,
                level: 1,
                faction_template: 1,
                display_id: 50,
                native_display_id: 50,
                ..Default::default()
            },
            ..Default::default()
        };
        let player_bytes = player.get_diff(None).to_bytes().unwrap();
        let expected = [
            3, 23, 0, 128, 1, 1, 0, 192, 0, 24, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0,
            128, 63, 1, 1, 1, 1, 100, 0, 0, 0, 100, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 50, 0, 0, 0,
            50, 0, 0, 0,
        ];
        assert_eq!(player_bytes, expected);
    }
}
