use deku::bitvec::{BitVec, Msb0};
use deku::prelude::*;
use sha2::Digest;

pub fn parse_string(input: Vec<u8>) -> Result<String, DekuError> {
    String::from_utf8(input).map_err(|e| DekuError::Parse(e.to_string()))
}

pub fn string_writer(output: &mut BitVec<Msb0, u8>, field: &str) -> Result<(), DekuError> {
    field.as_bytes().write(output, ())
}

fn get_digested<'lt, T>(o1: &'lt [u8], o0: &'lt [u8], o2: &'lt [u8]) -> Vec<u8>
where
    T: Digest,
{
    let mut o0_hasher = T::new();
    o0_hasher.update(o1);
    o0_hasher.update(o0);
    o0_hasher.update(o2);
    o0_hasher.finalize().to_vec()
}

pub fn generate_session_key<T>(seed: &[u8], size: u8) -> Vec<u8>
where
    T: Digest,
{
    let o1 = T::digest(&seed[..seed.len() / 2]).to_vec();
    let o2 = T::digest(&seed[seed.len() / 2..]).to_vec();
    let o0_initial = vec![0; <T as Digest>::output_size()];
    let mut o0 = get_digested::<T>(&o1, &o0_initial, &o2);
    let mut res = Vec::with_capacity(size as usize);
    let mut i = 0;
    while i < size {
        for b in o0.iter() {
            res.push(*b);
            i += 1;
            if i >= size {
                break;
            }
        }
        o0 = get_digested::<T>(&o1, &o0, &o2);
    }

    res
}

pub fn pack_u128(value: u128) -> (u16, Vec<u8>) {
    let mut mask = u16::MIN;
    let mut result = vec![];
    for i in 0..16 {
        let byte = (value >> i * 8) & 0xFF;
        if byte != 0 {
            mask |= (1 << i) as u16;
            result.push(byte as u8);
        }
    }

    (mask, result)
}

#[derive(Debug, Eq, PartialEq)]
pub enum UnpackingError {
    InvalidMask,
    NotEnoughData,
}

pub fn unpack_u128(mask: u16, data: &[u8]) -> Result<u128, UnpackingError> {
    if data.len() > mask.count_ones() as usize {
        return Err(UnpackingError::InvalidMask);
    } else if data.len() < mask.count_ones() as usize {
        return Err(UnpackingError::NotEnoughData);
    }
    let mut result = u128::MIN;
    let mut data_idx = 0;
    for i in 0..16 {
        let not_empty = (mask >> i) & 1 == 1;
        if not_empty {
            result |= (data[data_idx] as u128) << i * 8;
            data_idx += 1;
        }
    }
    Ok(result.to_le())
}

#[cfg(test)]
mod test {
    use crate::utils::{pack_u128, unpack_u128, UnpackingError};

    #[test]
    fn test_pack_unpack_min() {
        let num = u128::MIN;
        let pack_res = pack_u128(num);
        assert_eq!(pack_res.0, u16::MIN);
        assert_eq!(pack_res.1.len(), 0);
        let unpack_res = unpack_u128(pack_res.0, &pack_res.1);
        assert!(unpack_res.is_ok());
        assert_eq!(unpack_res.unwrap(), num)
    }

    #[test]
    fn test_pack_unpack_max() {
        let num = u128::MAX;
        let pack_res = pack_u128(num);
        assert_eq!(pack_res.0, u16::MAX);
        assert_eq!(pack_res.1.len(), 16);
        let unpack_res = unpack_u128(pack_res.0, &pack_res.1);
        assert!(unpack_res.is_ok());
        assert_eq!(unpack_res.unwrap(), num)
    }

    #[test]
    fn test_pack_unpack() {
        let num: u128 = 0x01000000000001100000000000000001;
        let pack_res = pack_u128(num);
        assert_eq!(pack_res.0, 0b1000001100000001);
        assert_eq!(pack_res.1.len(), 4);
        let unpack_res = unpack_u128(pack_res.0, &pack_res.1);
        assert!(unpack_res.is_ok());
        assert_eq!(unpack_res.unwrap(), num)
    }

    #[test]
    fn test_invalid_mask() {
        let unpack_res = unpack_u128(0, &[0, 0]);
        assert!(unpack_res.is_err());
        assert_eq!(unpack_res.unwrap_err(), UnpackingError::InvalidMask);
    }

    #[test]
    fn test_not_enough_data() {
        let unpack_res = unpack_u128(u16::MAX, &[]);
        assert!(unpack_res.is_err());
        assert_eq!(unpack_res.unwrap_err(), UnpackingError::NotEnoughData);
    }
}
