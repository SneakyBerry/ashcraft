use crate::define_flags;
use deku::prelude::*;

define_flags!(
    StructName: SplineFlag
    InnerType: u32 {
        DONE = 0x01,
        FALLING = 0x02,
        UNKNOWN3 = 0x04,
        UNKNOWN4 = 0x08,
        UNKNOWN5 = 0x10,
        UNKNOWN6 = 0x20,
        UNKNOWN7 = 0x40,
        UNKNOWN8 = 0x80,
        RUNMODE = 0x100,
        FLYING = 0x200,
        NO_SPLINE = 0x400,
        UNKNOWN12 = 0x800,
        UNKNOWN13 = 0x1000,
        UNKNOWN14 = 0x2000,
        UNKNOWN15 = 0x4000,
        UNKNOWN16 = 0x8000,
        FINAL_POINT = 0x10000,
        FINAL_TARGET = 0x20000,
        FINAL_ANGLE = 0x40000,
        UNKNOWN19 = 0x80000,
        CYCLIC = 0x100000,
        ENTER_CYCLE = 0x200000,
        FROZEN = 0x400000,
        UNKNOWN23 = 0x800000,
        UNKNOWN24 = 0x1000000,
        UNKNOWN25 = 0x2000000,
        UNKNOWN26 = 0x4000000,
        UNKNOWN27 = 0x8000000,
        UNKNOWN28 = 0x10000000,
        UNKNOWN29 = 0x20000000,
        UNKNOWN30 = 0x40000000,
        UNKNOWN31 = 0x80000000
    }
);
