macro_rules! make_field {
    ( $global_offset:expr => $struct_name:tt => $start:expr => $name:ident(()) ) => {
        // Padding
        impl $struct_name {
            paste! {const [<_$global_offset:upper _ $start:upper>]: () = ();}
        }
    };
    ( $global_offset:expr => $struct_name:tt => $start:expr => object_guid($ty:ty) ) => {
        impl $struct_name {
            paste! {const [<_$global_offset:upper _ $start:upper>]: () = ();}
            fn set_guid(&mut self, guid: $ty) -> &mut Self {
                use $crate::objects::traits::Storable;
                let value = Storable::store(guid);
                self.set_value(value.as_slice(), $global_offset + $start)
            }

            fn get_guid(&self) -> Option<$ty> {
                use $crate::objects::traits::Storable;
                use $crate::objects::traits::ObjectField;
                let vals = self.get_value($global_offset + $start..$global_offset + $start + <$ty>::SIZE)?;
                Some(Storable::load(vals.as_slice().try_into().unwrap()))
            }
        }
    };
    ( $global_offset:expr => $struct_name:tt => $start:expr => object_type($ty:ty) ) => {
        impl $struct_name {
            paste! {const [<_$global_offset:upper _ $start:upper>]: () = ();}
            fn set_object_type(&mut self, object_type: $ty) -> &mut Self {
                use $crate::objects::traits::Storable;
                let value = Storable::store(object_type);
                self.set_value(value.as_slice(), $global_offset + $start)
            }

            fn get_object_type(&self) -> Option<$ty> {
                use $crate::objects::traits::Storable;
                use $crate::objects::traits::ObjectField;
                let vals = self.get_value($global_offset + $start..$global_offset + $start + <$ty>::SIZE)?;
                Some(Storable::load(vals.as_slice().try_into().unwrap()))
            }
        }
    };

    ( $global_offset:expr => $struct_name:tt => $start:expr => $name:ident([bool; $size:expr]) ) => {
        impl $struct_name {
            paste! {const [<_$global_offset:upper _ $start:upper>]: () = ();}
            paste! {
                pub fn [<set_ $name>](&mut self, index: u16) -> &mut Self {
                    use $crate::objects::traits::ObjectField;
                    if index >= $size * 32 {
                        panic!("Index is out of range");
                    }
                    else {
                        let place_index = index / 32;
                        let byte_index = (index % 4) as usize;
                        let bit_index = index % 8;
                        let mut val = self.get_value($global_offset + $start..$global_offset + $start + <[bool; $size]>::SIZE / $size + place_index).unwrap_or(vec![[0u8; 4]])[0];
                        val[byte_index] |= (1u8 << (7 - bit_index));
                        self.set_value(&[val], $global_offset + $start + place_index)
                    }
                }

                pub fn [<unset_ $name>](&mut self, index: u16) -> &mut Self {
                    use $crate::objects::traits::ObjectField;
                    if index >= $size * 32 {
                        panic!("Index is out of range");
                    }
                    else {
                        let place_index = index / 32;
                        let byte_index = (index % 4) as usize;
                        let bit_index = index % 8;
                        let mut val = self.get_value($global_offset + $start..$global_offset + $start + <[bool; $size]>::SIZE / $size + place_index).unwrap_or(vec![[0u8; 4]])[0];
                        val[byte_index] &= !(1u8 << (7 - bit_index));
                        self.set_value(&[val], $global_offset + $start + place_index)
                    }
                }

                pub fn [<get_ $name>](&self, index: u16) -> Option<bool> {
                    use $crate::objects::traits::ObjectField;
                    if index >= $size * 32 {
                        panic!("Index is out of range");
                    }
                    else {
                        let place_index = index / 32;
                        let byte_index = (index % 4) as usize;
                        let bit_index = index % 8;
                        let val = self.get_value($global_offset + $start..$global_offset + $start + <[bool; $size]>::SIZE / $size + place_index)?[0][byte_index];
                        Some(val & (1u8 << (7 - bit_index)) != 0)
                    }
                }
            }
        }
    };
    ( $global_offset:expr => $struct_name:tt => $start:expr => $name:ident([$ty:ty; $size:expr]) ) => {
        impl $struct_name {
            paste! {const [<_$global_offset:upper _ $start:upper>]: () = ();}
            paste! {
                pub fn [<set_ $name>](&mut self, $name: $ty, index: u16) -> &mut Self {
                    use $crate::objects::traits::Storable;
                    if index >= $size {
                        panic!("Index is out of range");
                    }
                    let value = Storable::store($name);
                    self.set_value(value.as_slice(), $global_offset + $start + index)
                }

                pub fn [<get_ $name>](&self, index: u16) -> Option<$ty> {
                    use $crate::objects::traits::Storable;
                    use $crate::objects::traits::ObjectField;
                    if index >= $size {
                        panic!("Index is out of range");
                    }
                    let vals = self.get_value($global_offset + $start..$global_offset + $start + <[$ty; $size]>::SIZE / $size + index)?;
                    Some(Storable::load(vals.as_slice().try_into().unwrap()))
                }
            }
        }
    };
    ( $global_offset:expr => $struct_name:tt => $start:expr => $name:ident($ty:ty) ) => {
        impl $struct_name {
            paste! {const [<_$global_offset:upper _ $start:upper>]: () = ();}
            paste! {
                pub fn [<set_ $name>](&mut self, $name: $ty) -> &mut Self {
                    use $crate::objects::traits::Storable;
                    let value = Storable::store($name);
                    self.set_value(value.as_slice(), $global_offset + $start)
                }

                pub fn [<get_ $name>](&self) -> Option<$ty> {
                    use $crate::objects::traits::Storable;
                    use $crate::objects::traits::ObjectField;
                    let vals = self.get_value($global_offset + $start..$global_offset + $start + <$ty>::SIZE)?;
                    Some(Storable::load(vals.as_slice().try_into().unwrap()))
                }
            }
        }
    };
}

macro_rules! impl_accessors {
    (
        Offset: $offset:expr;
        Size: $size:expr;
        impl $struct_name:path {
            $ ( $start:expr => $name:ident: $type:tt; ) *
        }
    ) => {
        $ (
            make_field!($offset => $struct_name => $start => $name($type));
        ) *

        // Static alignment check
        const _: () = {
            use $crate::objects::traits::ObjectField;
            let starts = [$( $start, )* $size];
            let _i = 1;
            $(
            if starts[_i] != $start + <$type>::SIZE {
               panic!("Bad fields mapping")
            };
            let _i = _i + 1;
            )*
        };
    };
}

macro_rules! object_fields {
    ( $object_type:expr => $struct_name:tt ) => {
        impl_accessors!(
            Offset: 0x0000;
            Size: 0x0006;
            impl $struct_name {
                0x0000 => object_guid: Guid;
                0x0002 => object_type: u32;
                0x0003 => object_entry: u32;
                0x0004 => object_scale_x: f32;
                0x0005 => object_padding: ();
            }
        );
        impl UpdateFields for $struct_name {
            fn get_object_type(&self) -> ObjectType {
                $object_type
            }
        }
        impl $struct_name {
            pub fn new(guid: Guid) -> Self {
                let mut object = Self::default();
                object.set_guid(guid)
                .set_object_type($object_type as u32);
                object
            }
            fn get_value(&self, range: std::ops::Range<u16>) -> Option<Vec<[u8; 4]>> {
                if range.is_empty() {
                    panic!("Incorrect range requested, range: {range:?}");
                }
                let res = self
                    .inner
                    .range(range.clone())
                    .map(|(_, v)| *v)
                    .collect::<Vec<_>>();
                if res.len() < range.len() {
                    None
                } else {
                    Some(res)
                }
            }

            fn set_value(&mut self, value: &[[u8; 4]], offset: u16) -> &mut Self {
                for (value_offset, value) in value.iter().enumerate() {
                    self.inner.insert(offset + value_offset as u16, *value);
                }
                self
            }
        }
    };
}
