#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameObjectBytes {
    pub state: GameObjectState,
    pub r#type: GameObjectTypes,
    pub art_kit: u8,
    pub anim_progress: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GameObjectState {
    Active = 0,    // show in world as used and not reset (closed door open)
    Ready = 1,     // show in world as ready (closed door close)
    Destroyed = 2, // show the object in-game as already used and not yet reset (e.g. door opened by a cannon blast)
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

macro_rules! game_object_fields {
    (
        impl for $struct_name:ident
    ) => {
        impl_accessors!(
            Offset: 0x0006;
            Size: 0x000C;
            impl $struct_name {
                0x0000 => game_object_created_by: Guid;
                0x0002 => game_object_display_id: u32;
                0x0003 => game_object_flags: [bool; 1];
                0x0004 => game_object_parent_rotation: Vector3d;
                0x0008 => game_object_dynamic: (u16, u16);
                0x0009 => game_object_faction: u32;
                0x000A => game_object_level: u32;
                0x000B => game_object_bytes: GameObjectBytes;
            }
        );
    };
}
