use crate::define_flags;

pub struct UpdateFlag {
    inner: u16,
}

define_flags!(
    StructName: UpdateFlag
    InnerType: u16 {
        SELF = 0x01,
        TRANSPORT = 0x02,
        HAS_ATTACKING_TARGET = 0x04,
        LOW_GUID = 0x08,
        HIGH_GUID = 0x10,
        LIVING = 0x20,
        HAS_POSITION = 0x40,
        VEHICLE = 0x80,
        POSITION = 0x100,
        ROTATION = 0x200
});

#[cfg(test)]
mod test {
    use crate::packets::update_flag::UpdateFlag;

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
rustycraft_world_server::packets::update_flag::UpdateFlag {
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
}".to_owned();
        assert_eq!(res, expect)
    }
}
