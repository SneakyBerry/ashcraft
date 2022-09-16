use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::prelude::*;
use std::collections::BTreeMap;

const OBJECT: u32 = 0x0001;

pub enum UpdateType {
    Item = 0x0002,
    Container = 0x0004,
    Unit = 0x0008,
    Player = 0x0010,
    GameObject = 0x0020,
    DynamicObject = 0x0040,
    Corpse = 0x0080,
}

pub(crate) fn header_set(header: &mut Vec<u32>, bit: u16) {
    let index = bit / 32;
    let offset = bit % 32;

    if index >= header.len() as u16 {
        let extras = index - header.len() as u16;
        for _ in 0..=extras {
            header.push(0);
        }
    }

    header[index as usize] |= 1 << offset;
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct UpdateMask {
    #[deku(update = "self.mask.len()")]
    amount_of_blocks: u8, // 74
    #[deku(count = "amount_of_blocks")]
    #[deku(endian = "little")]
    mask: Vec<u32>, // 78
    #[deku(reader = "header_values_reader(&mask, deku::rest)")]
    #[deku(writer = "header_values_writer(deku::output, &self.values)")]
    values: BTreeMap<u16, u32>,
}

impl UpdateMask {
    pub fn new(object: UpdateType) -> UpdateMask {
        const OBJECT_FIELD_TYPE: u16 = 2;

        let mut header = vec![];
        let mut values = BTreeMap::new();

        header_set(&mut header, OBJECT_FIELD_TYPE);
        values.insert(OBJECT_FIELD_TYPE, OBJECT | object as u32);
        UpdateMask {
            amount_of_blocks: header.len() as u8,
            mask: header,
            values
        }
    }

    pub fn header_set(&mut self, bit: u16) {
        header_set(&mut self.mask, bit);
    }

    pub fn set_value(&mut self, bit: u16, v: u32) -> &mut Self {
        self.header_set(bit);
        self.values.insert(bit, v);
        self
    }
}

fn header_values_reader<'a, 'b>(
    headers: &'b Vec<u32>,
    rest: &'a BitSlice<Msb0, u8>,
) -> Result<(&'a BitSlice<Msb0, u8>, BTreeMap<u16, u32>), DekuError> {
    let mut values = BTreeMap::new();
    let mut rest = rest;
    for (index, block) in headers.iter().enumerate() {
        for bit in 0..32 {
            if (block & 1 << bit) != 0 {
                let (rest_after, value) = u32::read(rest, ())?;
                rest = rest_after;
                values.insert(index as u16 * 32 + bit, value.to_le());
            }
        }
    }
    Ok((rest, values))
}
pub fn header_values_writer(
    output: &mut BitVec<Msb0, u8>,
    field: &BTreeMap<u16, u32>,
) -> Result<(), DekuError> {
    field
        .values()
        .into_iter()
        .flat_map(|v| v.to_le_bytes())
        .collect::<Vec<_>>()
        .write(output, ())
}

// Item = 0x0002,
// Container = 0x0004,
// Unit = 0x0008,
// Player = 0x0010,
// GameObject = 0x0020,
// DynamicObject = 0x0040,
// Corpse = 0x0080,
