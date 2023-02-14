use crate::guid::PackedGuid;
use crate::position::Vector3d;
use deku::prelude::*;

#[derive(Debug, Clone, PartialEq, DekuWrite, DekuRead)]
pub struct TransportInfo {
    pub guid: PackedGuid,
    pub position: Vector3d,
    pub timestamp: u32,
    pub seat: u8,
    // if (movementInfo.HasExtraMovementFlag(MOVEMENTFLAG2_INTERPOLATED_MOVEMENT))
    pub time_2: Option<u32>,
}
