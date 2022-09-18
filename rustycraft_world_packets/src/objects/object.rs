#[macro_export]
macro_rules! object_fields {
    (
        Offset: $offset:tt;
        impl Object for $struct_name:ident
    ) => {
        impl_accessors!(
            Offset: $offset;
            Size: 0x0006;
            impl $struct_name {
                0x0000 => object_guid: Guid;
                0x0002 => object_type: ObjectType;
                0x0003 => object_entry: u32;
                0x0004 => object_scale_x: f32;
                0x0005 => object_padding: ();
            }
        );
    };
}
