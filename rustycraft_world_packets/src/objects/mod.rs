use crate::guid::Guid;

use crate::objects::base::BaseObject;
use deku::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;
mod base;
mod container;
mod corpse;
mod dynamic_object;
mod game_object;
mod item;
mod object;
mod player;
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
pub trait UpdateFields:
    DekuUpdate + DekuContainerRead<'static> + DekuContainerWrite + Debug + Sync + Send
{
}
impl<T> UpdateFields for T where
    T: BaseObject<0x0000>
        + DekuUpdate
        + DekuContainerRead<'static>
        + DekuContainerWrite
        + Debug
        + Sync
        + Send
{
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use crate::class::Class;
    use crate::gender::Gender;
    use crate::guid::{Guid, HighGuid};
    use crate::objects::UpdateMaskObjectType;
    use crate::power::Power;
    use crate::race::Race;
    use deku::prelude::*;

    #[test]
    fn test_basic_player() {
        let mut object = Object::new(Default::default());
        object.set_object_entry(100);
        let mut player = Player::new(Guid::new(HighGuid::Player, 4));
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
        let loaded = Box::new(Player::from_bytes((&player_bytes, 0)).unwrap().1);
        assert_eq!(loaded, player)
    }
}
