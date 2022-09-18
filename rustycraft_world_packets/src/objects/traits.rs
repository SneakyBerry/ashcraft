use crate::guid::{Guid, HighGuid};
use crate::object::ObjectType;
use crate::objects::game_object::{GameObjectBytes, GameObjectState, GameObjectTypes};
use crate::position::Vector3d;
use deku::DekuContainerRead;
use crate::objects::item::ItemEnchantment;

pub trait ObjectField {
    const SIZE: u16;
}

impl ObjectField for u32 {
    const SIZE: u16 = 1;
}

impl ObjectField for i32 {
    const SIZE: u16 = 1;
}

impl ObjectField for bool {
    const SIZE: u16 = 1;
}

impl ObjectField for () {
    const SIZE: u16 = 1;
}

impl ObjectField for f32 {
    const SIZE: u16 = 1;
}

impl ObjectField for u64 {
    const SIZE: u16 = 2;
}

impl ObjectField for (u16, u16) {
    const SIZE: u16 = 1;
}

impl ObjectField for Guid {
    const SIZE: u16 = 2;
}

impl ObjectField for ObjectType {
    const SIZE: u16 = 1;
}

impl ObjectField for Vector3d {
    const SIZE: u16 = 4;
}

impl ObjectField for GameObjectBytes {
    const SIZE: u16 = 1;
}

impl ObjectField for ItemEnchantment {
    const SIZE: u16 = 3;
}

macro_rules! object_field_array {
    ( $ ( $n:expr ), + ) => {
        $ (
            impl<T> ObjectField for [T; $n]
            where
                T: ObjectField,
            {
                const SIZE: u16 = $n * T::SIZE;
            }
        ) *
    };
}

object_field_array!(36, 19, 5, 12);

pub trait Storable {
    type StoredType;

    fn store(self) -> Self::StoredType;
    fn load(val: Self::StoredType) -> Self;
}

impl Storable for u32 {
    type StoredType = [[u8; 4]; <Self as ObjectField>::SIZE as usize];

    fn store(self) -> Self::StoredType {
        [self.to_le_bytes()]
    }

    fn load(val: Self::StoredType) -> Self {
        Self::from_le_bytes(val[0])
    }
}

impl Storable for f32 {
    type StoredType = [[u8; 4]; <Self as ObjectField>::SIZE as usize];

    fn store(self) -> Self::StoredType {
        [self.to_le_bytes()]
    }

    fn load(val: Self::StoredType) -> Self {
        Self::from_le_bytes(val[0])
    }
}

impl Storable for i32 {
    type StoredType = [[u8; 4]; <Self as ObjectField>::SIZE as usize];

    fn store(self) -> Self::StoredType {
        [self.to_le_bytes()]
    }

    fn load(val: Self::StoredType) -> Self {
        Self::from_le_bytes(val[0])
    }
}

impl Storable for u64 {
    type StoredType = [[u8; 4]; <Self as ObjectField>::SIZE as usize];

    fn store(self) -> Self::StoredType {
        let self_as_bytes = self.to_le_bytes();
        let mut res = [[0; 4]; 2];
        for i in 0..2 {
            for ii in 0..4 {
                res[i][ii] = self_as_bytes[i * res[i].len() + ii];
            }
        }
        res
    }

    fn load(val: Self::StoredType) -> Self {
        let mut tmp = [0_u8; 8];
        for i in 0..2 {
            for ii in 0..4 {
                tmp[i * 4 + ii] = val[i][ii];
            }
        }
        Self::from_le_bytes(tmp)
    }
}

impl Storable for Guid {
    type StoredType = [[u8; 4]; <Self as ObjectField>::SIZE as usize];

    fn store(self) -> Self::StoredType {
        self.as_u64().store()
    }

    /// Panics if high guid is incorrect
    fn load(val: Self::StoredType) -> Self {
        let u_val = <u64 as Storable>::load(val);
        let high_guid = HighGuid::from_bytes((&val[1][2..], 0))
            .expect("Corrupted high guid")
            .1;
        Self::new(high_guid, u_val)
    }
}

impl Storable for ObjectType {
    type StoredType = [[u8; 4]; 1];

    fn store(self) -> Self::StoredType {
        [(self as u32).to_le_bytes()]
    }

    /// Panics if object is incorrect
    fn load(val: Self::StoredType) -> Self {
        ObjectType::from_bytes((&val[0], 0))
            .expect("Corrupted object type")
            .1
    }
}

impl Storable for [Guid; 36] {
    type StoredType = [[u8; 4]; <Self as ObjectField>::SIZE as usize];

    fn store(self) -> Self::StoredType {
        let mut res = [[0_u8; 4]; 72];
        for (i, guid) in self.into_iter().enumerate() {
            let stored_guid = <Guid as Storable>::store(guid);
            res[i * 2] = stored_guid[0];
            res[i * 2 + 1] = stored_guid[1];
        }
        res
    }

    fn load(val: Self::StoredType) -> Self {
        let mut res = [Guid::blank(); 36];
        for i in 0..36 {
            res[i] = <Guid as Storable>::load([val[i * 2], val[i * 2 + 1]]);
        }
        res
    }
}

impl Storable for Vector3d {
    type StoredType = [[u8; 4]; <Self as ObjectField>::SIZE as usize];

    fn store(self) -> Self::StoredType {
        let x = self.x.to_le_bytes();
        let y = self.y.to_le_bytes();
        let z = self.z.to_le_bytes();
        let rotation = self.rotation.to_le_bytes();

        [x, y, z, rotation]
    }

    fn load(val: Self::StoredType) -> Self {
        let x = <f32 as Storable>::load([val[0]]);
        let y = <f32 as Storable>::load([val[1]]);
        let z = <f32 as Storable>::load([val[2]]);
        let rotation = <f32 as Storable>::load([val[3]]);
        Self { x, y, z, rotation }
    }
}

impl Storable for ItemEnchantment {
    type StoredType = [[u8; 4]; <Self as ObjectField>::SIZE as usize];

    fn store(self) -> Self::StoredType {
        let charges = self.charges.to_le_bytes();
        let duration = self.duration.to_le_bytes();
        let id = self.id.to_le_bytes();

        [charges, duration, id]
    }

    fn load(val: Self::StoredType) -> Self {
        let charges = <u32 as Storable>::load([val[0]]);
        let duration = <u32 as Storable>::load([val[1]]);
        let id = <u32 as Storable>::load([val[2]]);
        Self { charges, duration, id }
    }
}

impl Storable for (u16, u16) {
    type StoredType = [[u8; 4]; <Self as ObjectField>::SIZE as usize];

    fn store(self) -> Self::StoredType {
        let l = self.0.to_le_bytes();
        let r = self.1.to_le_bytes();
        [[l[0], l[1], r[0], r[1]]]
    }

    fn load(val: Self::StoredType) -> Self {
        let l = u16::from_le_bytes([val[0][0], val[0][1]]);
        let r = u16::from_le_bytes([val[0][2], val[0][3]]);
        (l, r)
    }
}

impl Storable for GameObjectBytes {
    type StoredType = [[u8; 4]; <Self as ObjectField>::SIZE as usize];

    fn store(self) -> Self::StoredType {
        let mut res = [0; 4];
        res[0] = (self.state as u8).to_le_bytes()[0];
        res[1] = (self.r#type as u8).to_le_bytes()[0];
        res[2] = (self.art_kit as u8).to_le_bytes()[0];
        res[3] = (self.anim_progress as u8).to_le_bytes()[0];
        [res]
    }

    fn load(val: Self::StoredType) -> Self {
        let state = GameObjectState::from_bytes((&val[0], 0))
            .expect("Corrupted GameObjectState")
            .1;
        let r#type = GameObjectTypes::from_bytes((&val[0], 8))
            .expect("Corrupted GameObjectTypes")
            .1;
        let art_kit = val[0][2];
        let anim_progress = val[0][3];
        Self {
            state,
            r#type,
            art_kit,
            anim_progress,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_store_load {
        (
            $ ( $test_name:ident: $ty:ty $test_val: block ) +
        ) => {
            $ (
                #[test]
                fn $test_name() {
                    let test_value: $ty = $test_val;
                    let intermidiate = test_value.clone().store();
                    let result = <$ty>::load(intermidiate);
                    assert_eq!(test_value, result);
                }
            ) *
        };
    }

    test_store_load!(
        u32: u32 { 0x2D296378 }
        f32: f32 { 12345.6789 }
        u64: u64 { 0x00F9B6DD91FA0543 }
        guid: Guid { Guid::new( HighGuid::Corpse, 0x00F9B6DD91FA0543 ) }
        object_type: ObjectType { ObjectType::Container }
        guid_36: [Guid; 36] {
            let mut guids = [Guid::blank(); 36];
            for i in 0..36_u64 {
                guids[i as usize] = Guid::new(HighGuid::Corpse, i)
            };
            guids
        }
        vect_3d: Vector3d { Vector3d {
            x: 12.0,
            y: 34.0,
            z: 56.0,
            rotation: 78.0,
        } }
        two_u16: (u16, u16) { (103, 306) }
        go_bytes: GameObjectBytes { GameObjectBytes {
            state: GameObjectState::Destroyed,
            r#type: GameObjectTypes::AreaDamage,
            art_kit: 123,
            anim_progress: 253,
        } }
    );
}
