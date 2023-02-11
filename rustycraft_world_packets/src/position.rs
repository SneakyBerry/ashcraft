use crate::guid::PackedGuid;
use crate::movement_flags::{ExtraMovementFlags, MovementFlags};
use crate::transport::TransportInfo;
use deku::prelude::*;
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq, DekuWrite, DekuRead, Builder)]

pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: Option<f32>,
}

impl Vector3d {
    pub fn distance(&self, other: &Self) -> f32 {
        self - other
    }
}

impl Sub for &Vector3d {
    type Output = f32;

    fn sub(self, rhs: Self) -> Self::Output {
        ((rhs.x - self.x).powf(2.) + (rhs.y - self.y).powf(2.) + (rhs.z - self.z).powf(2.))
            .powf(0.5)
    }
}

#[derive(Debug, Clone, PartialEq, DekuWrite, DekuRead, Builder)]
pub struct VectorXYZO {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
}

#[derive(Debug, Clone, PartialEq, DekuWrite, DekuRead, Builder)]
pub struct FallingInfo {
    pub z_speed: f32,
    pub sin_angle: f32,
    pub cos_angle: f32,
    pub xy_speed: f32,
}

#[derive(Debug, Clone, PartialEq, DekuWrite, DekuRead, Builder)]
pub struct MovementInfo {
    pub guid: PackedGuid,
    pub flags: MovementFlags,
    pub flags_extra: ExtraMovementFlags,
    pub time: u32,
    pub pos: VectorXYZO,
    // TODO: CTX for TransportInfo
    #[deku(skip, cond = "!flags.is_on_transport()")]
    pub transport_info: Option<TransportInfo>,
    #[deku(cond = "!flags.is_swimming() || !flags.is_flying()", skip)]
    pub pitch: Option<f32>,
    pub fall_time: u32,
    #[deku(cond = "!flags.is_falling()", skip)]
    pub falling: Option<FallingInfo>,
    #[deku(cond = "!flags.is_spline_elevation()", skip)]
    pub spline_elevation: Option<f32>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::position::{MovementInfo, Vector3d};

    const DATA: &[u8; 32] = b"\x01\x04\x01\0\0\0\0\0\x08\x91.\x1f~\xe6\x0b\xc6\xd8\xb2\x03\xc3\xab|\xa6Bp=\xad@\0\0\0\0";

    #[test]
    fn test_deserialize() {
        let res = MovementInfo::from_bytes((DATA, 0)).unwrap().1;
        println!("{:#?}", &res);
    }

    #[test]
    fn test_distance_zero() {
        let vec1 = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rotation: Some(0.0),
        };
        let vec2 = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rotation: Some(0.0),
        };
        assert_eq!(vec1.distance(&vec2), 0.)
    }

    #[test]
    fn test_distance_10x() {
        let vec1 = Vector3d {
            x: 5.,
            y: 0.,
            z: 0.,
            rotation: Some(0.0),
        };
        let vec2 = Vector3d {
            x: -5.,
            y: 0.,
            z: 0.,
            rotation: Some(0.0),
        };
        assert_eq!(vec1.distance(&vec2), 10.)
    }

    #[test]
    fn test_distance_10y() {
        let vec1 = Vector3d {
            x: 0.,
            y: 5.,
            z: 0.,
            rotation: Some(0.0),
        };
        let vec2 = Vector3d {
            x: 0.,
            y: -5.,
            z: 0.,
            rotation: Some(0.0),
        };
        assert_eq!(vec1.distance(&vec2), 10.)
    }

    #[test]
    fn test_distance_10z() {
        let vec1 = Vector3d {
            x: 0.,
            y: 0.,
            z: 5.,
            rotation: Some(0.0),
        };
        let vec2 = Vector3d {
            x: 0.,
            y: 0.,
            z: -5.,
            rotation: Some(0.0),
        };
        assert_eq!(vec1.distance(&vec2), 10.)
    }
}
