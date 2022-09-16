use crate::guid::PackedGuid;
use crate::position::Vector3d;
use deku::prelude::*;

#[derive(Debug, Clone, DekuWrite)]
pub struct TransportInfo {
    pub guid: PackedGuid, // SHOULD BE PACKED
    pub position: Vector3d,
    #[deku(endian = "little")]
    pub orientation: f32,
    #[deku(endian = "little")]
    pub timestamp: u32,
    pub seat: u8,
    // if (movementInfo.HasExtraMovementFlag(MOVEMENTFLAG2_INTERPOLATED_MOVEMENT))
    #[deku(endian = "little")]
    pub time_2: Option<u32>,
}
