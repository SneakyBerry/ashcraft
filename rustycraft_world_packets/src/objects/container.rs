// use crate::guid::Guid;
// use crate::BitVec;
// use deku::prelude::*;
// use crate::objects::{ObjectInner, UpdateObject};
// use crate::objects::traits::sealed::Sealed;

// #[derive(Debug, Clone, Default, DekuRead)]
// pub struct Container {
//     #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
//     inner: ObjectInner,
//     // inner: [Option<u32>; 74],
//     // #[deku(endian = "little", pad_bytes_after = "4")]
//     // num_slots: Option<u32>,
//     // slots: [Option<Guid>; 36],
// }

#[macro_export]
macro_rules! container_fields {
    (
        Offset: $offset:tt;
        impl Container for $struct_name:ident
    ) => {
        impl_accessors!(
            Offset: $offset;
            Size: 0x004A;
            impl $struct_name {
                0x0000 => container_num_slots: u32;
                0x0001 => container_padding: ();
                0x0002 => container_slot: [Guid; 36];
            }
        );
    };
}
