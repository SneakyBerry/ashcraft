use crate::object::ObjectType;
use crate::{BitSlice, Msb0};
use deku::prelude::*;
use std::ops::{BitAnd, Shr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, DekuWrite, DekuRead)]
pub struct Guid {
    #[deku(reader = "read_high_guid(deku::rest)")]
    high_guid: HighGuid, // 16
    #[deku(bytes = "6")]
    #[deku(endian = "little")]
    guid: u64, // 48
}

#[derive(Debug, Clone, Eq, PartialEq, DekuWrite, DekuRead)]
pub struct PackedGuid {
    mask: u8,
    #[deku(count = "mask.count_ones()")]
    parts: Vec<u8>,
}


/// Tricky part where we read high_guid from tail
fn read_high_guid(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, HighGuid), DekuError> {
    let res = HighGuid::read(&rest[48..], ())?;
    Ok((rest, res.1))
}

impl Guid {
    pub fn new(high: HighGuid, guid: u64) -> Guid {
        Guid {
            high_guid: high,
            guid: guid & 0x0000FFFFFFFFFFFF,
        }
    }

    pub fn blank() -> Guid {
        Self {
            high_guid: HighGuid::Player,
            guid: 0
        }
    }

    pub fn as_u64(&self) -> u64 {
        self.guid | (self.high_guid as u64) << 48
    }

    pub fn as_u32(&self) -> [u32; 2] {
        [(self.guid >> 32) as u32, self.guid as u32]
    }

    const fn has_entry(&self) -> bool {
        match self.high_guid {
            HighGuid::Item
            | HighGuid::Player
            | HighGuid::DynamicObject
            | HighGuid::Corpse
            | HighGuid::MoTransport
            | HighGuid::Instance
            | HighGuid::Group => true,
            _ => false,
        }
    }

    pub fn get_entry(&self) -> Option<u32> {
        if self.has_entry() {
            Some(self.guid.shr(24u64).bitand(0x0000000000FFFFFF) as u32)
        } else {
            None
        }
    }

    pub fn get_counter(&self) -> u32 {
        self.guid.bitand(if self.has_entry() {
            0x0000000000FFFFFF
        } else {
            0x00000000FFFFFFFF
        }) as u32
    }
}

impl From<Guid> for PackedGuid {
    fn from(guid: Guid) -> Self {
        let mut parts = vec![];
        let mut mask = 0u8;
        for (idx, val) in guid.as_u64().to_le_bytes().into_iter().enumerate() {
            if val != 0 {
                mask |= 1 << idx;
                parts.push(val);
            }
        }
        PackedGuid {
            mask,
            parts
        }
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

        for i in 0..8 {
            if (mask >> i) & 1 == 1 {
                byte_vec[i] = value.parts[arr_ptr];
                arr_ptr += 1;
            }
        }
        let guid = u64::from_le_bytes(byte_vec);
        Ok(Guid::new(
            HighGuid::from_bytes((&guid.to_le_bytes()[6..], 0))?.1,
            guid,
        ))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, DekuWrite, DekuRead)]
#[deku(type = "u16")]
#[deku(endian = "little")]
pub enum HighGuid {
    Item = 0x4000,       // blizz 4000                                   GUID_TRAIT_GLOBAL
    Player = 0x0000,     // blizz 0000                                    GUID_TRAIT_GLOBAL
    GameObject = 0xF110, // blizz F110                                    GUID_TRAIT_MAP_SPECIFIC
    Transport = 0xF120,  // blizz F120 (for GAMEOBJECT_TYPE_TRANSPORT)    GUID_TRAIT_MAP_SPECIFIC
    Unit = 0xF130,       // blizz F130                                    GUID_TRAIT_MAP_SPECIFIC
    Pet = 0xF140,        // blizz F140                                    GUID_TRAIT_MAP_SPECIFIC
    Vehicle = 0xF150,    // blizz F550                                    GUID_TRAIT_MAP_SPECIFIC
    DynamicObject = 0xF100, // blizz F100                                    GUID_TRAIT_MAP_SPECIFIC
    Corpse = 0xF101,     // blizz F100                                    GUID_TRAIT_MAP_SPECIFIC
    MoTransport = 0x1FC0, // blizz 1FC0 (for GAMEOBJECT_TYPE_MO_TRANSPORT) GUID_TRAIT_GLOBAL
    Instance = 0x1F40,   // blizz 1F40                                    GUID_TRAIT_GLOBAL
    Group = 0x1F50,      //                                               GUID_TRAIT_GLOBAL
}

impl From<HighGuid> for ObjectType {
    fn from(guid: HighGuid) -> Self {
        match guid {
            HighGuid::Item => Self::Item,
            HighGuid::Player => Self::Player,
            HighGuid::GameObject | HighGuid::MoTransport => Self::GameObject,
            HighGuid::Unit | HighGuid::Pet | HighGuid::Vehicle => Self::Unit,
            HighGuid::DynamicObject => Self::DynamicObject,
            HighGuid::Corpse => Self::Corpse,
            HighGuid::Instance => todo!(),
            HighGuid::Group => todo!(),
            HighGuid::Transport => Self::Object,
        }
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
    use crate::guid::{Guid, HighGuid, PackedGuid};
    use deku::DekuContainerWrite;
    use std::io::{Cursor, Read};
    use deku::DekuContainerRead;

    #[test]
    fn test_packed() {
        let hex_guid = 0x0000F00B00BAB0BA;

        let guid = Guid::new(HighGuid::Item, hex_guid);
        assert_eq!(guid.guid, hex_guid);
        let packed = PackedGuid::from(guid);
        assert_eq!(packed.mask, 0b10110111);
        assert_eq!(packed.parts, vec![0xBA, 0xB0, 0xBA, 0x0B, 0xF0, 0x40]);
        let guid1 = Guid::try_from(&packed).unwrap();
        assert_eq!(guid, guid1);

        let packed_guid = vec![0b10110111, 0xBA, 0xB0, 0xBA, 0x0B, 0xF0, 0x40];
        let (_, packed1) = PackedGuid::from_bytes((&packed_guid, 0)).unwrap();
        assert_eq!(packed, packed1);
        let (_, guid2) =  Guid::from_bytes((&0x4000F00B00BAB0BAu64.to_le_bytes(), 0)).unwrap();
        assert_eq!(guid1, guid2);
    }
}
