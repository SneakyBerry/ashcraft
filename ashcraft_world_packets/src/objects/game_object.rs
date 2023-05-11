use crate::guid::Guid;
use crate::objects::object::ObjectUpdate;
use crate::position::Vector3d;
use deku::prelude::*;
use ashcraft_derive::CalcUpdate;

#[derive(Debug, Default, Clone, CalcUpdate, Builder)]
#[meta(offset = 0x0006, tag = 0x0021)]
pub struct GameObjectUpdate {
    #[nested]
    pub object: ObjectUpdate,
    pub created_by: Guid,
    pub display_id: u32,
    pub flags: u32,
    pub parent_rotation: Vector3d,
    pub dynamic: (u16, u16),
    pub faction: u32,
    pub level: u32,
    pub bytes: GameObjectBytes,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
pub struct GameObjectBytes {
    pub state: GameObjectState,
    pub r#type: GameObjectTypes,
    pub art_kit: u8,
    pub anim_progress: u8,
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum GameObjectState {
    #[default]
    Active = 0, // show in world as used and not reset (closed door open)
    Ready = 1,     // show in world as ready (closed door close)
    Destroyed = 2, // show the object in-game as already used and not yet reset (e.g. door opened by a cannon blast)
}

#[derive(Debug, Default, Clone, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum GameObjectTypes {
    #[default]
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
