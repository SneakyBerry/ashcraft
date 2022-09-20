use deku::prelude::*;

use crate::guid::Guid;
use crate::objects::base::{BaseObject, Storage};
use crate::objects::{InnerState, UpdateMaskObjectType};
use crate::position::Vector3d;

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct GameObject {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: InnerState,
}
impl GameObject {
    pub fn new(guid: Guid) -> Box<Self> {
        let mut object = Self::default();
        <Self as BaseObject<0x0000>>::set_guid(&mut object, guid);
        <Self as BaseObject<0x0000>>::set_object_type(
            &mut object,
            UpdateMaskObjectType::GameObject as u32,
        );
        Box::new(object)
    }
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct GameObjectBytes {
    pub state: GameObjectState,
    pub r#type: GameObjectTypes,
    pub art_kit: u8,
    pub anim_progress: u8,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum GameObjectState {
    Active = 0,    // show in world as used and not reset (closed door open)
    Ready = 1,     // show in world as ready (closed door close)
    Destroyed = 2, // show the object in-game as already used and not yet reset (e.g. door opened by a cannon blast)
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum GameObjectTypes {
    Door = 0,
    Button = 1,
    QuestGiver = 2,
    Chest = 3,
    Binder = 4,
    Generic = 5,
    Trap = 6,
    Chair = 7,
    SpellFocus = 8,
    Text = 9,
    Goober = 10,
    Transport = 11,
    AreaDamage = 12,
    Camera = 13,
    MapObject = 14,
    MoTransport = 15,
    DuelArbiter = 16,
    FishingNode = 17,
    SummoningRitual = 18,
    Mailbox = 19,
    DoNotUse = 20,
    Guardpost = 21,
    SpellCaster = 22,
    MeetingStone = 23,
    FlagStand = 24,
    FishingHole = 25,
    FlagDrop = 26,
    MiniGame = 27,
    DoNotUse2 = 28,
    CapturePoint = 29,
    AuraGenerator = 30,
    DungeonDifficulty = 31,
    BarberChair = 32,
    DestructibleBuilding = 33,
    GuildBank = 34,
    Trapdoor = 35,
}

pub trait GameObjectFields: BaseObject<0x0006> {
    fn set_game_object_created_by(&mut self, game_object_created_by: Guid) -> &mut Self {
        self.set_value(game_object_created_by, 0x0000)
    }
    fn get_game_object_created_by(&self) -> Option<Guid> {
        self.get_value(0x0000)
    }

    fn set_game_object_display_id(&mut self, game_object_display_id: u32) -> &mut Self {
        self.set_value(game_object_display_id, 0x0002)
    }
    fn get_game_object_display_id(&self) -> Option<u32> {
        self.get_value(0x0002)
    }
    fn set_game_object_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x0003, mask)
    }
    fn unset_game_object_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x0003, !(mask))
    }
    fn get_game_object_flags(&mut self) -> Option<u32> {
        self.get_value(0x0003)
    }
    fn set_game_object_parent_rotation(
        &mut self,
        game_object_parent_rotation: Vector3d,
    ) -> &mut Self {
        self.set_value(game_object_parent_rotation, 0x0004)
    }
    fn get_game_object_parent_rotation(&self) -> Option<Vector3d> {
        self.get_value(0x0004)
    }
    fn set_game_object_dynamic(&mut self, game_object_dynamic: (u16, u16)) -> &mut Self {
        self.set_value(game_object_dynamic, 0x0008)
    }
    fn get_game_object_dynamic(&self) -> Option<(u16, u16)> {
        self.get_value(0x0008)
    }
    fn set_game_object_faction(&mut self, game_object_faction: u32) -> &mut Self {
        self.set_value(game_object_faction, 0x0009)
    }
    fn get_game_object_faction(&self) -> Option<u32> {
        self.get_value(0x0009)
    }
    fn set_game_object_level(&mut self, game_object_level: u32) -> &mut Self {
        self.set_value(game_object_level, 0x000A)
    }
    fn get_game_object_level(&self) -> Option<u32> {
        self.get_value(0x000A)
    }
    fn set_game_object_bytes(&mut self, game_object_bytes: GameObjectBytes) -> &mut Self {
        self.set_value(game_object_bytes, 0x000B)
    }
    fn get_game_object_bytes(&self) -> Option<GameObjectBytes> {
        self.get_value(0x000B)
    }
}

impl GameObjectFields for GameObject {}
impl Storage for GameObject {
    fn get_inner(&self) -> &InnerState {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut InnerState {
        &mut self.inner
    }
}
