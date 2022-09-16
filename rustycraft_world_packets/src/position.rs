use deku::prelude::*;
use std::ops::Sub;

#[derive(Debug, Clone, DekuWrite, DekuRead, Builder)]
#[deku(endian = "little")]
pub struct Vector3d {
    #[deku(endian = "little")]
    pub x: f32,
    #[deku(endian = "little")]
    pub y: f32,
    #[deku(endian = "little")]
    pub z: f32,
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

#[cfg(test)]
mod test {
    use crate::position::Vector3d;

    #[test]
    fn test_distance_zero() {
        let vec1 = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let vec2 = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(vec1.distance(&vec2), 0.)
    }

    #[test]
    fn test_distance_10x() {
        let vec1 = Vector3d {
            x: 5.,
            y: 0.,
            z: 0.,
        };
        let vec2 = Vector3d {
            x: -5.,
            y: 0.,
            z: 0.,
        };
        assert_eq!(vec1.distance(&vec2), 10.)
    }

    #[test]
    fn test_distance_10y() {
        let vec1 = Vector3d {
            x: 0.,
            y: 5.,
            z: 0.,
        };
        let vec2 = Vector3d {
            x: 0.,
            y: -5.,
            z: 0.,
        };
        assert_eq!(vec1.distance(&vec2), 10.)
    }

    #[test]
    fn test_distance_10z() {
        let vec1 = Vector3d {
            x: 0.,
            y: 0.,
            z: 5.,
        };
        let vec2 = Vector3d {
            x: 0.,
            y: 0.,
            z: -5.,
        };
        assert_eq!(vec1.distance(&vec2), 10.)
    }
}
