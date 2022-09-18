enum DynamicObjectType {
    Portal = 0x0, // unused
    AreaSpell = 0x1,
    FarSightFocus = 0x2,
}

#[macro_export]
macro_rules! dynamic_object_fields {
    (
        Offset: $offset:tt;
        impl DynamicObject for $struct_name:ident
    ) => {
        impl_accessors!(
            Offset: $offset;
            Size: 0x0006;
            impl $struct_name {
                0x0000 => dynamic_object_caster: Guid;
                0x0002 => dynamic_object_bytes: u32;
                0x0003 => dynamic_object_spell_id: u32;
                0x0004 => dynamic_object_radius: f32;
                0x0005 => dynamic_object_cast_time: u32;
            }
        );
    };
}
