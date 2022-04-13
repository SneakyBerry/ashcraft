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
                break
            }
        }
        o0 = get_digested::<T>(&o1, &o0, &o2);
    }

    res
}
