use crate::guid::PackedGuid;
use crate::movement_flags::{ExtraMovementFlags, MovementFlags};
use crate::transport::TransportInfo;
use bevy_math::{Quat, Vec3};
use deku::prelude::*;

#[derive(Debug, Clone, PartialEq, DekuWrite, DekuRead, Valuable, Builder)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, DekuWrite, DekuRead, Valuable, Builder)]
pub struct FallingInfo {
    pub z_speed: f32,
    pub sin_angle: f32,
    pub cos_angle: f32,
    pub xy_speed: f32,
}

#[derive(Debug, Clone, PartialEq, DekuWrite, DekuRead, Valuable, Builder)]
pub struct CMovementData {
    pub guid: PackedGuid,
    pub movement_info: MovementInfo,
}

#[derive(Debug, Clone, PartialEq, DekuWrite, DekuRead, Valuable, Builder)]
pub struct MovementInfo {
    pub flags: MovementFlags,
    pub flags_extra: ExtraMovementFlags,
    pub time: u32,
    pub pos: Vector3d,
    // TODO: CTX for TransportInfo
    #[deku(cond = "!flags.is_on_transport()", skip)]
    pub transport_info: Option<TransportInfo>,
    #[deku(cond = "!flags.is_swimming() && !flags.is_flying()", skip)]
    pub pitch: Option<f32>,
    pub fall_time: u32,
    #[deku(cond = "!flags.is_falling()", skip)]
    pub falling: Option<FallingInfo>,
    #[deku(cond = "!flags.is_spline_elevation()", skip)]
    pub spline_elevation: Option<f32>,
}

impl From<&Vector3d> for Quat {
    fn from(value: &Vector3d) -> Self {
        Quat::from_rotation_x(value.rotation.unwrap_or(0.0))
    }
}

impl From<&Vector3d> for Vec3 {
    fn from(value: &Vector3d) -> Self {
        Vec3::new(value.x, value.y, value.z)
    }
}

impl From<&Vector3d> for (Vec3, Quat) {
    fn from(value: &Vector3d) -> Self {
        (Vec3::from(value), Quat::from(value))
    }
}
