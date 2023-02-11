use deku::prelude::*;

use crate::define_flags;

#[derive(Clone, Default, DekuWrite, Builder)]
pub struct UpdateFlag {
    inner: u16,
}

define_flags!(
    StructName: UpdateFlag
    InnerType: u16 {
        SELF = 0x0001,
        TRANSPORT = 0x0002,
        HAS_ATTACKING_TARGET = 0x0004,
        LOW_GUID = 0x0008,
        HIGH_GUID = 0x0010,
        LIVING = 0x0020,
        HAS_POSITION = 0x0040,
        VEHICLE = 0x0080,
        POSITION = 0x0100,
        ROTATION = 0x0200
});
