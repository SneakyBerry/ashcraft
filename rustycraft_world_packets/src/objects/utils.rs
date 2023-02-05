use crate::objects::InnerState;
use deku::bitvec::*;
use deku::{DekuError, DekuWrite};

#[inline]
fn aliquot(base: usize, x: usize) -> usize {
    if x % base == 0 {
        x
    } else {
        x + base - (x % base)
    }
}

pub(crate) fn write_object_btree_map(
    output: &mut BitVec<u8, Msb0>,
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
