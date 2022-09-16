use crate::object::ObjectType;
use deku::prelude::*;
use std::ops::{BitAnd, Shr};

#[derive(Debug, Clone, DekuWrite, DekuRead)]
pub struct Guid {
    pub high_guid: HighGuid, // 16
    #[deku(bytes = "6", endian = "little")]
    pub guid: u64, // 48
}

#[derive(Debug, Clone, DekuWrite)]
pub struct PackedGuid(Vec<u8>);

impl From<Guid> for PackedGuid {
    fn from(guid: Guid) -> Self {
        let mut v = vec![];
        let placeholder_index = v.len();
        // Placeholder
        v.push(0);

        let guid = guid.as_u64().to_le_bytes();
        let mut bit_pattern = 0;

        for (i, &b) in guid.iter().enumerate() {
            if b != 0 {
                bit_pattern |= 1 << i;
                v.push(b);
            }
        }

        v[placeholder_index] = bit_pattern;
        PackedGuid(v)
    }
}

impl Guid {
    pub fn guid(&self) -> u64 {
        self.guid
    }

    pub fn as_u64(&self) -> u64 {
        u64::MAX.bitand(self.guid << 16) | self.high_guid as u64
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

    pub fn pack_guid(&self) -> [u8; 9] {
        let mut pack_guid = [0; 8 + 1];
        let mut size = 1;
        let mut guid = self.guid;
        let mut i = 0;
        while guid != 0 {
            if guid & 0xFF != 0 {
                pack_guid[0] |= (1 << i) as u8;
                pack_guid[size] = (guid & 0xFF) as u8;
                size += 1;
            }

            guid >>= 8;
            i += 1
        }
        pack_guid
    }
}

#[derive(Debug, Clone, Copy, DekuWrite, DekuRead)]
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

    #[test]
    fn test_as_u64() {
        let guid = Guid {
            high_guid: HighGuid::Item,
            guid: u64::MAX,
        };
        assert_eq!(guid.as_u64(), 0xFFFFFFFFFFFF4000);
        assert_eq!(
            guid.as_u64().to_le_bytes().to_vec(),
            guid.to_bytes().unwrap()
        );
    }

    #[test]
    fn test_packed() {
        let guid = Guid {
            high_guid: HighGuid::Player,
            guid: 4,
        };
        assert_eq!(guid.as_u64(), 0x0000000000040000);
        println!("{:?}", &guid.pack_guid());
        // assert_eq!(PackedGuid::from(guid).to_bytes().unwrap(), vec![4, 4, 4, 21, 0, 0, 0, 0]);
    }
}
