use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::prelude::*;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct ObjectGuid {
    // 24 bits
    pub low: u64,
    // 40 bits
    pub server_id: u64,

    // 6 bits
    pub high: HighGuid,
    // 3 bits?
    pub unk: u8,
    // 13 bits
    pub realm_id: u16,
    // 13 bits
    pub map_id: u16,
    // 23 bits
    pub entry: u32,
    // 6 bits
    pub sub_type: u8,
}

impl ObjectGuid {
    pub fn as_raw(&self) -> (u64, u64) {
        let mut low = 0u64;
        let mut high = 0u64;
        low |= self.low as u64 & 0xFFFFFFFFFF;
        low |= ((self.server_id) as u64) << 40;

        high |= (self.high as u64) << 58;
        high |= (self.unk as u64 & 0x7) << 45;
        high |= (self.realm_id as u64 & 0x1FFF) << 42;
        high |= (self.map_id as u64 & 0x1FFF) << 29;
        high |= (self.entry as u64 & 0x7FFFFF) << 6;
        high |= self.sub_type as u64 & 0x3F;
        (low, high)
    }
}

// we don't actually need it but DekuContainerRead have DekuRead in supertraits.
impl<'a> DekuRead<'a, ()> for ObjectGuid {
    fn read(_input: &BitSlice<Msb0, u8>, _ctx: ()) -> Result<(&BitSlice<Msb0, u8>, Self), DekuError>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl<'a> DekuContainerRead<'a> for ObjectGuid {
    fn from_bytes(input: (&'a [u8], usize)) -> Result<((&'a [u8], usize), Self), DekuError>
    where
        Self: Sized,
    {
        let low = u64::from_le_bytes((&input.0[0..8]).try_into()?);
        let high_bytes = &input.0[8..16];
        let high = u64::from_le_bytes(high_bytes.try_into()?);
        Ok((
            (&[], 0),
            ObjectGuid {
                low: low & 0xFFFFFFFFFF,
                server_id: (low >> 40) & 0xFFFFFF,
                high: HighGuid::from_bytes((&[high_bytes[7] & 0xFC], 0))?.1,
                unk: ((high >> 45) & 0x7) as u8,
                realm_id: ((high >> 42) & 0x1FFF) as u16,
                map_id: ((high >> 29) & 0x1FFF) as u16,
                entry: ((high >> 6) & 0x7FFFFF) as u32,
                sub_type: (high & 0x3F) as u8,
            },
        ))
    }
}

impl DekuWrite for ObjectGuid {
    fn write(&self, output: &mut BitVec<Msb0, u8>, ctx: ()) -> Result<(), DekuError> {
        todo!()
    }
}

impl DekuContainerWrite for ObjectGuid {
    fn to_bytes(&self) -> Result<Vec<u8>, DekuError> {
        let mut low = 0u64;
        let mut high = 0u64;
        low |= self.low as u64 & 0xFFFFFFFFFF;
        low |= ((self.server_id) as u64) << 40;

        high |= (self.high as u64) << 58;
        high |= (self.unk as u64 & 0x7) << 45;
        high |= (self.realm_id as u64 & 0x1FFF) << 42;
        high |= (self.map_id as u64 & 0x1FFF) << 29;
        high |= (self.entry as u64 & 0x7FFFFF) << 6;
        high |= self.sub_type as u64 & 0x3F;

        let mut res = Vec::with_capacity(16);
        res.extend(low.to_le_bytes());
        res.extend(high.to_le_bytes());

        Ok(res)
    }
    fn to_bits(&self) -> Result<BitVec<Msb0, u8>, DekuError> {
        todo!()
    }
}

impl ObjectGuid {
    pub fn new() -> ObjectGuid {
        ObjectGuid {
            low: 0,
            server_id: 0,
            high: HighGuid::Null,
            unk: 0,
            realm_id: 0,
            map_id: 0,
            entry: 0,
            sub_type: 0,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8")]
#[deku(bits = "6")]
#[repr(u8)]
pub enum HighGuid {
    Null = 0x00,
    Uniq = 0x01,
    Player = 0x02,
    Item = 0x03,
    WorldTransaction = 0x04,
    StaticDoor = 0x05, //NYI
    Transport = 0x06,
    Conversation = 0x07,
    Creature = 0x08,
    Vehicle = 0x09,
    Pet = 0x0A,
    GameObject = 0x0B,
    DynamicObject = 0x0C,
    AreaTrigger = 0x0D,
    Corpse = 0x0E,
    LootObject = 0x0F,
    SceneObject = 0x10,
    Scenario = 0x11,
    AIGroup = 0x12,
    DynamicDoor = 0x13,
    ClientActor = 0x14, //NYI
    Vignette = 0x15,
    CallForHelp = 0x16,
    AIResource = 0x17,
    AILock = 0x18,
    AILockTicket = 0x19,
    ChatChannel = 0x1A,
    Party = 0x1B,
    Guild = 0x1C,
    WowAccount = 0x1D,
    BNetAccount = 0x1E,
    GMTask = 0x1F,
    MobileSession = 0x20, //NYI
    RaidGroup = 0x21,
    Spell = 0x22,
    Mail = 0x23,
    WebObj = 0x24,    //NYI
    LFGObject = 0x25, //NYI
    LFGList = 0x26,   //NYI
    UserRouter = 0x27,
    PVPQueueGroup = 0x28,
    UserClient = 0x29,
    PetBattle = 0x2A, //NYI
    UniqUserClient = 0x2B,
    BattlePet = 0x2C,
    CommerceObj = 0x2D,
    ClientSession = 0x2E,
    Cast = 0x2F,
    ClientConnection = 0x30,
    ClubFinder = 0x31,
    ToolsClient = 0x32,
    WorldLayer = 0x33,
    ArenaTeam = 0x34,
}
