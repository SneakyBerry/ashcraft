use crate::objects::InnerState;
use deku::bitvec::*;
use deku::error::NeedSize;
use deku::{DekuError, DekuWrite};
use std::collections::BTreeMap;

pub(crate) fn read_object_btree_map(
    rest: &BitSlice<Msb0, u8>,
) -> Result<(&BitSlice<Msb0, u8>, InnerState), DekuError> {
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
        .map(BitSlice::as_raw_slice)
        .map(|x| {
            x.try_into()
                .expect("It should be 32 bits according to chunk size")
        })
        .map(u32::from_le_bytes)
        .rev()
        .collect::<Vec<_>>();

    let mut res = BTreeMap::new();
    for (idx, mask) in masks.iter().enumerate() {
        for pos in 0..32 {
            if mask & (1 << pos) == (1 << pos) {
                let key = idx * 32 + pos;
                res.insert(
                    key,
                    values
                        .pop()
                        .expect("Length can not be less then needed because of previous check."),
                );
            }
        }
    }

    Ok((rest, res))
}

#[inline]
fn aliquot(base: usize, x: usize) -> usize {
    if x % base == 0 {
        x
    } else {
        x + base - (x % base)
    }
}

pub(crate) fn write_object_btree_map(
    output: &mut BitVec<Msb0, u8>,
    field: &InnerState,
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
        if field.contains_key(&bit) {
            masks[mask_idx as usize] |= 1_u32.wrapping_shl(bit_pos as u32);
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
