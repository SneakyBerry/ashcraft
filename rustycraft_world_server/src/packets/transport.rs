use crate::packets::guid::Guid;
use crate::packets::vector3d::Vector3d;
use deku::prelude::*;

#[derive(Debug, DekuWrite)]
pub struct TransportInfo {
    pub guid: Guid,
    pub position: Vector3d,
    pub orientation: f32,
    pub timestamp: u32,
    pub seat: u8,
}
