use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::prelude::*;
use std::collections::BTreeMap;

pub const OBJECT: u32 = 0x0001;
pub const ITEM: u32 = 0x0002;
pub const CONTAINER: u32 = 0x0004;
pub const UNIT: u32 = 0x0008;
pub const PLAYER: u32 = 0x0010;
pub const GAMEOBJECT: u32 = 0x0020;
pub const DYNAMICOBJECT: u32 = 0x0040;
pub const CORPSE: u32 = 0x0080;

#[derive(Debug, DekuRead)]
pub struct GameObjectHeaders {
    #[deku(update = "self.header.len()")]
    pub amount_of_blocks: u8,
    #[deku(count = "amount_of_blocks")]
    pub header: Vec<u32>,
    #[deku(reader = "header_values_reader(&header, deku::rest)")]
    pub values: BTreeMap<u16, u32>,
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

// pub(crate) fn header_set(header: &mut Vec<u32>, bit: u16) {
//     let index = bit / 32;
//     let offset = bit % 32;
//
//     if index >= header.len() as u16 {
//         let extras = index - header.len() as u16;
//         for _ in 0..=extras {
//             header.push(0);
//         }
//     }
//
//     header[index as usize] |= 1 << offset;
// }
//
// pub(crate) fn write_into_vec(
//     v: &mut Vec<u8>,
//     header: &[u32],
//     values: &BTreeMap<u16, u32>,
// ) -> Result<(), std::io::Error> {
//     v.write_all(&[header.len() as u8])?;
//
//     for h in header {
//         v.write_all(h.to_le_bytes().as_slice())?;
//     }
//
//     for value in values.values() {
//         v.write_all(&value.to_le_bytes())?;
//     }
//
//     Ok(())
// }
//
// pub(crate) fn read_inner(
//     r: &mut impl Read,
// ) -> Result<(Vec<u32>, BTreeMap<u16, u32>), std::io::Error> {
//     let amount_of_blocks = crate::util::read_u8_le(r)?;
//
//     let mut header = Vec::new();
//     for _ in 0..amount_of_blocks {
//         header.push(crate::util::read_u32_le(r)?);
//     }
//
//     let mut values = BTreeMap::new();
//     for (index, block) in header.iter().enumerate() {
//         for bit in 0..32 {
//             if (block & 1 << bit) != 0 {
//                 values.insert(index as u16 * 32 + bit, crate::util::read_u32_le(r)?);
//             }
//         }
//     }
//
//     Ok((header, values))
// }
//
// #[derive(Debug, Hash, Clone, PartialEq, Eq)]
// pub enum UpdateMask {
//     Item(UpdateItem),
//     Container(UpdateContainer),
//     Unit(UpdateUnit),
//     Player(UpdatePlayer),
//     GameObject(UpdateGameObject),
//     DynamicObject(UpdateDynamicObject),
//     Corpse(UpdateCorpse),
// }
//
// impl UpdateMask {
//     pub(crate) fn read(r: &mut impl Read) -> Result<Self, io::Error> {
//         let (header, values) = read_inner(r)?;
//
//         let ty = match values.get(&2) {
//             None => {
//                 return Err(std::io::Error::new(
//                     std::io::ErrorKind::Other,
//                     "Missing object TYPE",
//                 ))
//             }
//             Some(ty) => *ty,
//         };
//
//         Ok(if (ty & CONTAINER) != 0 {
//             Self::Container(UpdateContainer::from_inners(header, values))
//         } else if (ty & ITEM) != 0 {
//             Self::Item(UpdateItem::from_inners(header, values))
//         } else if (ty & PLAYER) != 0 {
//             Self::Player(UpdatePlayer::from_inners(header, values))
//         } else if (ty & UNIT) != 0 {
//             Self::Unit(UpdateUnit::from_inners(header, values))
//         } else if (ty & GAMEOBJECT) != 0 {
//             Self::GameObject(UpdateGameObject::from_inners(header, values))
//         } else if (ty & DYNAMICOBJECT) != 0 {
//             Self::DynamicObject(UpdateDynamicObject::from_inners(header, values))
//         } else if (ty & CORPSE) != 0 {
//             Self::Corpse(UpdateCorpse::from_inners(header, values))
//         } else {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 "Object type not valid",
//             ));
//         })
//     }
//
//     pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
//         match self {
//             UpdateMask::Item(i) => i.write_into_vec(v),
//             UpdateMask::Container(i) => i.write_into_vec(v),
//             UpdateMask::Unit(i) => i.write_into_vec(v),
//             UpdateMask::Player(i) => i.write_into_vec(v),
//             UpdateMask::GameObject(i) => i.write_into_vec(v),
//             UpdateMask::DynamicObject(i) => i.write_into_vec(v),
//             UpdateMask::Corpse(i) => i.write_into_vec(v),
//         }
//     }
//
//     pub(crate) fn size(&self) -> usize {
//         let (header_len, values_len) = match self {
//             UpdateMask::Item(i) => (i.header.len(), i.values.len()),
//             UpdateMask::Container(i) => (i.header.len(), i.values.len()),
//             UpdateMask::Unit(i) => (i.header.len(), i.values.len()),
//             UpdateMask::Player(i) => (i.header.len(), i.values.len()),
//             UpdateMask::GameObject(i) => (i.header.len(), i.values.len()),
//             UpdateMask::DynamicObject(i) => (i.header.len(), i.values.len()),
//             UpdateMask::Corpse(i) => (i.header.len(), i.values.len()),
//         };
//
//         1 // amount_of_blocks
//             + header_len * 4
//             + values_len * 4
//     }
// }
//
// #[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
// pub struct UpdateItem {
//     header: Vec<u32>,
//     values: BTreeMap<u16, u32>,
// }
// impl UpdateItem {
//     pub fn new() -> Self {
//         const OBJECT_FIELD_TYPE: u16 = 2;
//
//         let mut header = (::alloc::vec::Vec::new());
//         let mut values = BTreeMap::new();
//
//         ::rustycraft_world_server::helper::update_mask_common::header_set(
//             &mut header,
//             OBJECT_FIELD_TYPE,
//         );
//         values.insert(
//             OBJECT_FIELD_TYPE,
//             ::rustycraft_world_server::helper::update_mask_common::OBJECT | ITEM,
//         );
//
//         Self { header, values }
//     }
//
//     fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
//         Self { header, values }
//     }
//
//     pub(crate) fn header_set(&mut self, bit: u16) {
//         ::rustycraft_world_server::helper::update_mask_common::header_set(&mut self.header, bit);
//     }
//
//     pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
//         ::rustycraft_world_server::helper::update_mask_common::write_into_vec(
//             v,
//             &self.header,
//             &self.values,
//         )
//     }
// }
//
// #[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
// pub struct UpdateContainer {
//     header: Vec<u32>,
//     values: BTreeMap<u16, u32>,
// }
// impl UpdateContainer {
//     pub fn new() -> Self {
//         const OBJECT_FIELD_TYPE: u16 = 2;
//
//         let mut header = (::alloc::vec::Vec::new());
//         let mut values = BTreeMap::new();
//
//         ::rustycraft_world_server::helper::update_mask_common::header_set(
//             &mut header,
//             OBJECT_FIELD_TYPE,
//         );
//         values.insert(
//             OBJECT_FIELD_TYPE,
//             ::rustycraft_world_server::helper::update_mask_common::OBJECT | (ITEM | CONTAINER),
//         );
//
//         Self { header, values }
//     }
//
//     fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
//         Self { header, values }
//     }
//
//     pub(crate) fn header_set(&mut self, bit: u16) {
//         ::rustycraft_world_server::helper::update_mask_common::header_set(&mut self.header, bit);
//     }
//
//     pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
//         ::rustycraft_world_server::helper::update_mask_common::write_into_vec(
//             v,
//             &self.header,
//             &self.values,
//         )
//     }
// }
//
// #[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
// pub struct UpdateUnit {
//     header: Vec<u32>,
//     values: BTreeMap<u16, u32>,
// }
// impl UpdateUnit {
//     pub fn new() -> Self {
//         const OBJECT_FIELD_TYPE: u16 = 2;
//
//         let mut header = (::alloc::vec::Vec::new());
//         let mut values = BTreeMap::new();
//
//         ::rustycraft_world_server::helper::update_mask_common::header_set(
//             &mut header,
//             OBJECT_FIELD_TYPE,
//         );
//         values.insert(
//             OBJECT_FIELD_TYPE,
//             ::rustycraft_world_server::helper::update_mask_common::OBJECT | UNIT,
//         );
//
//         Self { header, values }
//     }
//
//     fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
//         Self { header, values }
//     }
//
//     pub(crate) fn header_set(&mut self, bit: u16) {
//         ::rustycraft_world_server::helper::update_mask_common::header_set(&mut self.header, bit);
//     }
//
//     pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
//         ::rustycraft_world_server::helper::update_mask_common::write_into_vec(
//             v,
//             &self.header,
//             &self.values,
//         )
//     }
// }
//
// #[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
// pub struct UpdatePlayer {
//     header: Vec<u32>,
//     values: BTreeMap<u16, u32>,
// }
// impl UpdatePlayer {
//     pub fn new() -> Self {
//         const OBJECT_FIELD_TYPE: u16 = 2;
//
//         let mut header = (::alloc::vec::Vec::new());
//         let mut values = BTreeMap::new();
//
//         ::rustycraft_world_server::helper::update_mask_common::header_set(
//             &mut header,
//             OBJECT_FIELD_TYPE,
//         );
//         values.insert(
//             OBJECT_FIELD_TYPE,
//             ::rustycraft_world_server::helper::update_mask_common::OBJECT | (UNIT | PLAYER),
//         );
//
//         Self { header, values }
//     }
//
//     fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
//         Self { header, values }
//     }
//
//     pub(crate) fn header_set(&mut self, bit: u16) {
//         ::rustycraft_world_server::helper::update_mask_common::header_set(&mut self.header, bit);
//     }
//
//     pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
//         ::rustycraft_world_server::helper::update_mask_common::write_into_vec(
//             v,
//             &self.header,
//             &self.values,
//         )
//     }
// }
//
// #[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
// pub struct UpdateGameObject {
//     header: Vec<u32>,
//     values: BTreeMap<u16, u32>,
// }
// impl UpdateGameObject {
//     pub fn new() -> Self {
//         const OBJECT_FIELD_TYPE: u16 = 2;
//
//         let mut header = (::alloc::vec::Vec::new());
//         let mut values = BTreeMap::new();
//
//         ::rustycraft_world_server::helper::update_mask_common::header_set(
//             &mut header,
//             OBJECT_FIELD_TYPE,
//         );
//         values.insert(
//             OBJECT_FIELD_TYPE,
//             ::rustycraft_world_server::helper::update_mask_common::OBJECT | GAMEOBJECT,
//         );
//
//         Self { header, values }
//     }
//
//     fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
//         Self { header, values }
//     }
//
//     pub(crate) fn header_set(&mut self, bit: u16) {
//         ::rustycraft_world_server::helper::update_mask_common::header_set(&mut self.header, bit);
//     }
//
//     pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
//         ::rustycraft_world_server::helper::update_mask_common::write_into_vec(
//             v,
//             &self.header,
//             &self.values,
//         )
//     }
// }
//
// #[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
// pub struct UpdateDynamicObject {
//     header: Vec<u32>,
//     values: BTreeMap<u16, u32>,
// }
// impl UpdateDynamicObject {
//     pub fn new() -> Self {
//         const OBJECT_FIELD_TYPE: u16 = 2;
//
//         let mut header = (::alloc::vec::Vec::new());
//         let mut values = BTreeMap::new();
//
//         ::rustycraft_world_server::helper::update_mask_common::header_set(
//             &mut header,
//             OBJECT_FIELD_TYPE,
//         );
//         values.insert(
//             OBJECT_FIELD_TYPE,
//             ::rustycraft_world_server::helper::update_mask_common::OBJECT | DYNAMICOBJECT,
//         );
//
//         Self { header, values }
//     }
//
//     fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
//         Self { header, values }
//     }
//
//     pub(crate) fn header_set(&mut self, bit: u16) {
//         ::rustycraft_world_server::helper::update_mask_common::header_set(&mut self.header, bit);
//     }
//
//     pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
//         ::rustycraft_world_server::helper::update_mask_common::write_into_vec(
//             v,
//             &self.header,
//             &self.values,
//         )
//     }
// }
//
// #[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
// pub struct UpdateCorpse {
//     header: Vec<u32>,
//     values: BTreeMap<u16, u32>,
// }
// impl UpdateCorpse {
//     pub fn new() -> Self {
//         const OBJECT_FIELD_TYPE: u16 = 2;
//
//         let mut header = (::alloc::vec::Vec::new());
//         let mut values = BTreeMap::new();
//
//         ::rustycraft_world_server::helper::update_mask_common::header_set(
//             &mut header,
//             OBJECT_FIELD_TYPE,
//         );
//         values.insert(
//             OBJECT_FIELD_TYPE,
//             ::rustycraft_world_server::helper::update_mask_common::OBJECT | CORPSE,
//         );
//
//         Self { header, values }
//     }
//
//     fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
//         Self { header, values }
//     }
//
//     pub(crate) fn header_set(&mut self, bit: u16) {
//         ::rustycraft_world_server::helper::update_mask_common::header_set(&mut self.header, bit);
//     }
//
//     pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
//         ::rustycraft_world_server::helper::update_mask_common::write_into_vec(
//             v,
//             &self.header,
//             &self.values,
//         )
//     }
// }
