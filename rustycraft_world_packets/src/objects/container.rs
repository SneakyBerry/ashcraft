macro_rules! container_fields {
    (
    impl for $struct_name:ident
    ) => {
        impl_accessors!(
            Offset: 0x0040;
            Size: 0x004A;
            impl $struct_name {
                0x0000 => container_num_slots: u32;
                0x0001 => container_padding: ();
                0x0002 => container_slot: [Guid; 36];
            }
        );
    };
}
