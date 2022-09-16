use deku::prelude::*;

use crate::define_flags;

#[derive(Clone, Default, DekuWrite)]
pub struct UpdateFlag {
    #[deku(endian = "little")]
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

#[cfg(test)]
mod test {
    use crate::update_flag::UpdateFlag;

    #[test]
    fn test() {
        let mut a = UpdateFlag::new(0);
        a.set_self();
        a.set_has_attacking_target();
        a.set_high_guid();
        a.set_has_position();
        a.set_vehicle();
        a.set_rotation();
        let res = format!("{:?}", &a);
        let expect = "\
rustycraft_world_server::update_flag::UpdateFlag {
	self: true,
	transport: false,
	has_attacking_target: true,
	low_guid: false,
	high_guid: true,
	living: false,
	has_position: true,
	vehicle: true,
	position: false,
	rotation: true,
}"
        .to_owned();
        assert_eq!(res, expect)
    }
}
