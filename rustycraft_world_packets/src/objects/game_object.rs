use deku::prelude::*;

use crate::guid::Guid;
use crate::objects::size_helper::FieldSize;
use crate::objects::object::Object;
use crate::objects::UpdateFields;
use crate::position::Vector3d;

use rustycraft_derive::IntoUpdateFields;

#[derive(Debug, Default, Clone, IntoUpdateFields, Builder)]
#[meta(offset = 0x0006, tag = 0x0021)]
pub struct GameObject {
    #[nested]
    pub object: Object,
    pub created_by: Option<Guid>,
    pub display_id: Option<u32>,
    pub flags: Option<u32>,
    pub parent_rotation: Option<Vector3d>,
    pub dynamic: Option<(u16, u16)>,
    pub faction: Option<u32>,
    pub level: Option<u32>,
    pub bytes: Option<GameObjectBytes>,
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
