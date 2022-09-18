mod object_field_impls;
mod storable_impls;

pub trait ObjectField {
    const SIZE: u16;
}

pub trait Storable: ObjectField {
    type StoredType;

    fn store(self) -> Self::StoredType;
    fn load(val: Self::StoredType) -> Self;
}

#[cfg(test)]
mod test {
    use crate::guid::{Guid, HighGuid};
    use crate::object::ObjectType;
    use crate::objects::game_object::{GameObjectBytes, GameObjectState, GameObjectTypes};
    use crate::objects::item::ItemEnchantment;
    use crate::objects::traits::{ObjectField, Storable};
    use crate::position::Vector3d;

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
