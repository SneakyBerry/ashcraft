use crate::objects::ObjectInner;
use deku::bitvec::*;
use deku::error::NeedSize;
use deku::{DekuError, DekuWrite};
use std::collections::BTreeMap;
use std::io::Write;
use std::ops::{Add, Rem, Shr, Sub};
use std::process::id;

pub(crate) fn read_object_btree_map(
    rest: &BitSlice<Msb0, u8>,
) -> Result<(&BitSlice<Msb0, u8>, ObjectInner), DekuError> {
    if rest.len() < 8 {
        return Err(DekuError::Incomplete(NeedSize::new(8)));
    }
    let (num_mask_blocks, rest) = rest.split_at(8);
    let num_masks = num_mask_blocks.load::<u8>();
    let masks_end = num_masks as usize * 32;

    if rest.len() < masks_end {
        return Err(DekuError::Incomplete(NeedSize::new(masks_end - rest.len())));
    }

    let (masks, rest) = rest.split_at(masks_end);
    let blocks_count = masks.count_ones() * 32;
    let masks = masks
        .chunks(32)
        .map(BitSlice::load_le)
        .collect::<Vec<u32>>();

    if rest.len() < blocks_count {
        return Err(DekuError::Incomplete(NeedSize::new(
            blocks_count - rest.len(),
        )));
    }
    let (values, rest) = rest.split_at(blocks_count);

    let mut values = values
        .chunks(32)
        .map(|x| {
            x.chunks(8)
                .map(BitSlice::load::<u8>)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .rev()
        .collect::<Vec<_>>();

    let mut res = BTreeMap::new();
    for (idx, mask) in masks.iter().enumerate() {
        for pos in (0..32).rev() {
            if mask & (1 << pos) == (1 << pos) {
                let key = (idx * 32 + 31 - pos) as u16;
                res.insert(key, values.pop().unwrap()); // Unwrap is ok, length was checked
            }
        }
    }

    Ok((rest, res))
}

#[inline]
fn aliquot(base: u16, x: u16) -> u16 {
    if x % base == 0 {
        x
    } else {
        x + base - (x % base)
    }
}

pub(crate) fn write_object_btree_map(
    output: &mut BitVec<Msb0, u8>,
    field: &ObjectInner,
) -> Result<(), DekuError> {
    if field.len() > 0x578 {
        panic!("Max fields count: 1 400, given: {}", field.len());
    }
    let keys = field.keys();
    let bit_mask_size = aliquot(32, *keys.max().unwrap_or(&0));
    let mut masks = vec![0_u32; bit_mask_size as usize / 32];
    for bit in 0..bit_mask_size as usize * 8 {
        let mask_idx = bit / 32;
        let bit_pos = bit % 32;
        if field.contains_key(&(bit as u16)) {
            masks[mask_idx as usize] |= 1_u32.wrapping_shl(31_u32 - bit_pos as u32);
        }
    }
    (masks.len() as u8).to_le_bytes().write(output, ())?; // write bitmask size
    masks
        .iter()
        .flat_map(|x| x.to_le_bytes())
        .collect::<Vec<_>>()
        .write(output, ())?;
    field.values().collect::<Vec<_>>().write(output, ())?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::objects::utils::{aliquot, read_object_btree_map, write_object_btree_map};
    use crate::objects::ObjectInner;
    use deku::bitvec::BitVec;

    #[test]
    fn test_read_write() {
        let mut map = ObjectInner::new();
        let mut out = BitVec::new();

        for idx in 0..0x578 {
            map.insert(idx, (idx as u32).to_le_bytes());
        }

        write_object_btree_map(&mut out, &map).unwrap();

        let (rest, res) = read_object_btree_map(&out).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_read_write_err_len() {
        let mut map = ObjectInner::new();
        let mut out = BitVec::new();

        for idx in 0..0x579 {
            map.insert(idx, (idx as u32).to_le_bytes());
        }

        let _ = write_object_btree_map(&mut out, &map);
    }

    #[test]
    fn test_aliquot() {
        assert_eq!(aliquot(8, 16), 16);
    }
}
