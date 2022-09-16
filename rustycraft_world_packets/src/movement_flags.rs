use crate::define_flags;
use deku::prelude::*;

#[derive(Clone, DekuWrite)]
pub struct ExtraMovementFlags {
    #[deku(endian = "little")]
    inner: u16,
}

define_flags!(
    StructName: ExtraMovementFlags
    InnerType: u16 {
        NO_STRAFE = 0x01,
        NO_JUMPING = 0x02,
        UNK3 = 0x04,
        FULL_SPEED_TURNING = 0x08,
        FULL_SPEED_PITCHING = 0x10,
        ALWAYS_ALLOW_PITCHING = 0x20,
        UNK7 = 0x40,
        UNK8 = 0x80,
        UNK9 = 0x100,
        UNK10 = 0x200,
        INTERPOLATED_MOVEMENT = 0x400,
        INTERPOLATED_TURNING = 0x800,
        INTERPOLATED_PITCHING = 0x1000,
        UNK14 = 0x2000,
        UNK15 = 0x4000,
        UNK16 = 0x8000
});

#[derive(Clone, DekuWrite)]
pub struct MovementFlags {
    #[deku(endian = "little")]
    inner: u32,
}

define_flags!(
    StructName: MovementFlags
    InnerType: u32 {
        FORWARD = 0x01,
        BACKWARD = 0x02,
        STRAFE_LEFT = 0x04,
        STRAFE_RIGHT = 0x08,
        LEFT = 0x10,
        RIGHT = 0x20,
        PITCH_UP = 0x40,
        PITCH_DOWN = 0x80,
        WALKING = 0x100,
        ON_TRANSPORT = 0x200,
        DISABLE_GRAVITY = 0x400,
        ROOT = 0x800,
        FALLING = 0x1000,
        FALLING_FAR = 0x2000,
        PENDING_STOP = 0x4000,
        PENDING_STRAFE_STOP = 0x8000,
        PENDING_FORWARD = 0x10000,
        PENDING_BACKWARD = 0x20000,
        PENDING_STRAFE_LEFT = 0x40000,
        PENDING_STRAFE_RIGHT = 0x80000,
        PENDING_ROOT = 0x100000,
        SWIMMING = 0x200000,
        ASCENDING = 0x400000,
        DESCENDING = 0x800000,
        CAN_FLY = 0x1000000,
        FLYING = 0x2000000,
        SPLINE_ELEVATION = 0x4000000,
        SPLINE_ENABLED = 0x8000000,
        WATERWALKING = 0x10000000,
        FALLING_SLOW = 0x20000000,
        HOVER = 0x40000000
});
