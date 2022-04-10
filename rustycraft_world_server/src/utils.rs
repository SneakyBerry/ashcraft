use deku::bitvec::{BitVec, Msb0};
use deku::prelude::*;
use std::io::Write;

pub fn parse_string(input: Vec<u8>) -> Result<String, DekuError> {
    String::from_utf8(input).map_err(|e| DekuError::Parse(e.to_string()))
}

pub fn string_writer(output: &mut BitVec<Msb0, u8>, field: &str) -> Result<(), DekuError> {
    field.as_bytes().write(output, ())
}
