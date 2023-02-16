use deku::bitvec::{AsBits, BitSlice, BitVec, Msb0};
use deku::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
// 4 bits
#[repr(u8)]
pub enum MapSpecific {
    DynamicObject {
        // 28 bits
        unk: u32,
        // 24 bits
        counter: u32,
    } = 0x0,
    // #[deku(id = "0x01")]
    // Corpse {
    //     #[deku(bits = "28")]
    //     unk: u32,
    //     #[deku(bits = "24")]
    //     counter: u32,
    // },
    GameObject {
        // 28 bits
        object_id: u32,
        // 24 bits
        counter: u32,
    } = 0x1,
    Transport {
        // 28 bits
        unk: u32,
        // 24 bits
        counter: u32,
    } = 0x2,
    Unit {
        // 12 bits
        unk: u16,
        // 16 bits
        npc_id: u16,
        // 24 bits
        counter: u32,
    } = 0x3,
    Pet {
        // 28 bits
        pet_id: u32,
        // 24 bits
        counter: u32,
    } = 0x4,
    Vehicle {
        // 28 bits
        entry: u32,
        // 24 bits
        counter: u32,
    } = 0x5,
}

impl MapSpecific {
    fn parse(input: u64) -> Result<MapSpecific, DekuError> {
        let id = (input & 0x00F0000000000000) >> 52;
        match id {
            0x0 => Ok(MapSpecific::DynamicObject {
                unk: ((input & 0x000FFFFFFF000000) >> 24) as u32,
                counter: (input & 0x0000000000FFFFFF) as u32,
            }),
            0x1 => Ok(MapSpecific::GameObject {
                object_id: ((input & 0x000FFFFFFF000000) >> 24) as u32,
                counter: (input & 0x0000000000FFFFFF) as u32,
            }),
            0x2 => Ok(MapSpecific::Transport {
                unk: ((input & 0x000FFFFFFF000000) >> 24) as u32,
                counter: (input & 0x0000000000FFFFFF) as u32,
            }),
            0x3 => Ok(MapSpecific::Unit {
                unk: ((input & 0x000FFF0000000000) >> 40) as u16,
                npc_id: ((input & 0x000000FFFF000000) >> 24) as u16,
                counter: (input & 0x0000000000FFFFFF) as u32,
            }),
            0x4 => Ok(MapSpecific::Pet {
                pet_id: ((input & 0x000FFFFFFF000000) >> 24) as u32,
                counter: (input & 0x0000000000FFFFFF) as u32,
            }),
            0x5 => Ok(MapSpecific::Vehicle {
                entry: ((input & 0x000FFFFFFF000000) >> 24) as u32,
                counter: (input & 0x0000000000FFFFFF) as u32,
            }),
            _ => Err(DekuError::Parse(format!("Invalid MapSpecific id: {}", id))),
        }
    }

    fn to_int(&self) -> u64 {
        match self {
            MapSpecific::DynamicObject { unk, counter } => {
                0x0000000000000000
                    | (*unk as u64 & 0x000FFFFFFF) << 24
                    | *counter as u64 & 0x0000000000FFFFFF
            }
            MapSpecific::GameObject { object_id, counter } => {
                0x0010000000000000
                    | (*object_id as u64 & 0x000FFFFFFF) << 24
                    | *counter as u64 & 0x0000000000FFFFFF
            }
            MapSpecific::Transport { unk, counter } => {
                0x0020000000000000
                    | (*unk as u64 & 0x000FFFFFFF) << 24
                    | *counter as u64 & 0x0000000000FFFFFF
            }
            MapSpecific::Unit {
                unk,
                npc_id,
                counter,
            } => {
                0x0030000000000000
                    | (*unk as u64 & 0x000FFF) << 40
                    | (*npc_id as u64 & 0x000FFFF) << 24
                    | *counter as u64 & 0x0000000000FFFFFF
            }
            MapSpecific::Pet { pet_id, counter } => {
                0x0040000000000000
                    | (*pet_id as u64 & 0x000FFFFFFF) << 24
                    | *counter as u64 & 0x0000000000FFFFFF
            }
            MapSpecific::Vehicle { entry, counter } => {
                0x0050000000000000
                    | (*entry as u64 & 0x000FFFFFFF) << 24
                    | *counter as u64 & 0x0000000000FFFFFF
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
// 4 bits
#[repr(u8)]
pub enum Global {
    MoTransport {
        // 52 bits
        unk: u64,
    } = 0xC,
    Instance {
        // 52 bits
        unk: u64,
    } = 0x4,
    Group {
        // 52 bits
        unk: u64,
    } = 0x5,
}

impl Global {
    fn parse(input: u64) -> Result<Global, DekuError> {
        let id = (input & 0x00F0000000000000) >> 52;
        match id {
            0xC => Ok(Global::MoTransport {
                unk: input & 0x000FFFFFFFFFFFFF,
            }),
            0x4 => Ok(Global::Instance {
                unk: input & 0x000FFFFFFFFFFFFF,
            }),
            0x5 => Ok(Global::Group {
                unk: input & 0x000FFFFFFFFFFFFF,
            }),
            _ => Err(DekuError::Parse(format!("Invalid Global id: {}", id))),
        }
    }

    fn to_int(&self) -> u64 {
        match self {
            Global::MoTransport { unk } => 0x00C0000000000000 | unk & 0x000FFFFFFFFFFFFF,
            Global::Instance { unk } => 0x0040000000000000 | unk & 0x000FFFFFFFFFFFFF,
            Global::Group { unk } => 0x0050000000000000 | unk & 0x000FFFFFFFFFFFFF,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Player {
    // 4 bits
    unk: u8,
    // 28 bits
    entry: u32,
    // 24 bits
    counter: u32,
}

impl Player {
    pub fn new(entry: u32, counter: u32) -> Player {
        Player {
            unk: 0x0,
            entry,
            counter,
        }
    }
    fn parse(input: u64) -> Result<Player, DekuError> {
        Ok(Player {
            unk: ((input & 0x00F0000000000000) >> 52) as u8,
            entry: ((input & 0x000FFFFFFF000000) >> 24) as u32,
            counter: (input & 0x0000000000FFFFFF) as u32,
        })
    }
    fn to_int(&self) -> u64 {
        0x0000000000000000
            | (self.unk as u64) << 52
            | (self.entry as u64) << 24
            | self.counter as u64
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Item {
    // 56 bits
    pub unk: u64,
}

impl Item {
    fn parse(input: u64) -> Result<Item, DekuError> {
        Ok(Item {
            unk: input & 0x00FFFFFFFFFFFFFF,
        })
    }

    fn to_int(&self) -> u64 {
        self.unk & 0x00FFFFFFFFFFFFFF
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Guid {
    #[default]
    Empty,
    Player(Player),
    MapSpecific(MapSpecific),
    Item(Item),
    Global(Global),
}

impl TryFrom<u64> for Guid {
    type Error = DekuError;

    fn try_from(guid: u64) -> Result<Self, Self::Error> {
        if guid == 0 {
            Ok(Self::Empty)
        } else {
            match guid >> 56 {
                0x00 => Ok(Guid::Player(Player::parse(guid)?)),
                0xF1 => Ok(Guid::MapSpecific(MapSpecific::parse(guid)?)),
                0x40 => Ok(Guid::Item(Item::parse(guid)?)),
                0x1F => Ok(Guid::Global(Global::parse(guid)?)),
                _ => Err(DekuError::Parse(format!(
                    "Invalid guid type: {}",
                    guid >> 56
                ))),
            }
        }
    }
}

impl From<Guid> for u64 {
    fn from(value: Guid) -> Self {
        match value {
            Guid::Player(player) => 0x0000000000000000 | player.to_int(),
            Guid::MapSpecific(map_specific) => 0xF100000000000000 | map_specific.to_int(),
            Guid::Item(item) => 0x4000000000000000 | item.to_int(),
            Guid::Global(global) => 0x1F00000000000000 | global.to_int(),
            Guid::Empty => 0x0000000000000000,
        }
    }
}

impl<'a> DekuRead<'a> for Guid {
    fn read(
        input: &'a BitSlice<u8, Msb0>,
        _: (),
    ) -> Result<(&'a BitSlice<u8, Msb0>, Self), DekuError>
    where
        Self: Sized,
    {
        let (rest, guid) = u64::read(input, ())?;
        let res = Guid::try_from(guid)?;
        Ok((rest, res))
    }
}

impl<'a> DekuContainerRead<'a> for Guid {
    fn from_bytes(
        (input, offset): (&'a [u8], usize),
    ) -> Result<((&'a [u8], usize), Self), DekuError>
    where
        Self: Sized,
    {
        let (_, guid) = u64::read(&input.as_bits()[offset..], ())?;
        let res = Guid::try_from(guid)?;
        Ok(((&input[4..], offset), res))
    }
}

impl DekuWrite for Guid {
    fn write(&self, output: &mut BitVec<u8, Msb0>, _: ()) -> Result<(), DekuError> {
        let guid = u64::from(*self);
        guid.write(output, ())?;
        Ok(())
    }
}

impl DekuContainerWrite for Guid {
    fn to_bytes(&self) -> Result<Vec<u8>, DekuError> {
        let guid = u64::from(*self);
        Ok(guid.to_le_bytes().to_vec())
    }

    fn to_bits(&self) -> Result<BitVec<u8, Msb0>, DekuError> {
        let guid = u64::from(*self);
        let mut res = BitVec::new();
        guid.write(&mut res, ())?;
        Ok(res)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, DekuWrite, DekuRead)]
pub struct PackedGuid {
    mask: u8,
    #[deku(count = "mask.count_ones()")]
    parts: Vec<u8>,
}

impl From<Guid> for PackedGuid {
    fn from(guid: Guid) -> Self {
        let mut parts = vec![];
        let mut mask = 0u8;
        for (idx, val) in u64::from(guid).to_le_bytes().into_iter().enumerate() {
            if val != 0 {
                mask |= 1 << idx;
                parts.push(val);
            }
        }
        PackedGuid { mask, parts }
    }
}

impl TryFrom<&PackedGuid> for Guid {
    type Error = DekuError;

    fn try_from(value: &PackedGuid) -> Result<Self, Self::Error> {
        let mask = value.mask;

        // Just to ensure that PackedGuid is consistent
        // Because of PackedGuid have no external constructors
        // it should be an issue in parsing of PackedGuid
        assert_eq!(value.mask.count_ones() as usize, value.parts.len());

        let mut arr_ptr = 0;
        let mut byte_vec = [0u8; 8];

        for (i, byte) in byte_vec.iter_mut().enumerate() {
            if (mask >> i) & 1 == 1 {
                *byte = value.parts[arr_ptr];
                arr_ptr += 1;
            }
        }
        let guid = u64::from_le_bytes(byte_vec);
        Ok(Guid::try_from(guid)?)
    }
}

pub enum TypeMask {
    TypemaskObject = 0x0001,
    TypemaskItem = 0x0002,
    TypemaskContainer = 0x0006, // TypemaskItem | 0x0004
    TypemaskUnit = 0x0008,      // creature
    TypemaskPlayer = 0x0010,
    TypemaskGameobject = 0x0020,
    TypemaskDynamicobject = 0x0040,
    TypemaskCorpse = 0x0080,
    TypemaskSeer = (Self::TypemaskPlayer as u32
        | Self::TypemaskUnit as u32
        | Self::TypemaskDynamicobject as u32) as isize,
}

#[cfg(test)]
mod test {
    use crate::guid::{Guid, Item, PackedGuid};
    use deku::DekuContainerRead;

    #[test]
    fn test_packed() {
        let hex_guid = 0x0000F00B00BAB0BA;

        let guid = Guid::Item(Item { unk: hex_guid });
        let packed = PackedGuid::from(guid);
        assert_eq!(packed.mask, 0b10110111);
        assert_eq!(packed.parts, vec![0xBA, 0xB0, 0xBA, 0x0B, 0xF0, 0x40]);
        let guid1 = Guid::try_from(&packed).unwrap();
        assert_eq!(guid, guid1);

        let packed_guid = vec![0b10110111, 0xBA, 0xB0, 0xBA, 0x0B, 0xF0, 0x40];
        let (_, packed1) = PackedGuid::from_bytes((&packed_guid, 0)).unwrap();
        assert_eq!(packed, packed1);
        let (_, guid2) = Guid::from_bytes((&0x4000F00B00BAB0BAu64.to_le_bytes(), 0)).unwrap();
        assert_eq!(guid1, guid2);
    }

    #[test]
    fn test_guid_enum() {
        let input1 = 0xF13DEADBEEFB00B2u64;
        let res = input1.to_le_bytes();
        let guid = Guid::from_bytes((&res, 0)).unwrap().1;
        assert_eq!(u64::from(guid), input1);
    }
}
