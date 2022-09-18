use crate::guid::{Guid, HighGuid};
use crate::object::ObjectType;
use crate::objects::game_object::{GameObjectBytes, GameObjectState, GameObjectTypes};
use crate::objects::item::ItemEnchantment;
use crate::objects::player::{
    Bytes1, Bytes2, Bytes3, Bytes4, PlayerEnchantment, PlayerFieldBytes2Offsets, QuestLogItem,
    Rune, VisibleItem,
};
use crate::objects::traits::{ObjectField, Storable};
use crate::objects::unit::{AttackPower, ClassSpecific, UnitData};
use crate::position::Vector3d;

use crate::class::Class;
use crate::gender::Gender;
use crate::power::Power;
use crate::race::Race;
use deku::prelude::*;

macro_rules! storable_array {
    ( $ ( [$t:ty; $n:expr] ), + ) => {
        $ (
            impl Storable for [$t; $n]
            {
                type StoredType = [[u8; 4]; Self::SIZE as usize];

                fn store(self) -> Self::StoredType {
                    let mut res = [[0_u8; 4]; Self::SIZE as usize];
                    for (i, entry) in self.into_iter().enumerate() {
                        let stored = <$t as Storable>::store(entry);
                        for ii in 0..stored.len() {
                            res[i * stored.len() + ii] = stored[ii];
                        }
                    }
                    res
                }

                fn load(val: Self::StoredType) -> Self {
                    let mut res = [<$t>::default(); $n];
                    for i in 0..$n {
                        res[i] = <$t>::load([val[i * 2], val[i * 2 + 1]]);
                    }
                    res
                }
            }
        ) *
    };
}

storable_array!([Guid; 36]);

impl Storable for u32 {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

    fn store(self) -> Self::StoredType {
        [self.to_le_bytes()]
    }

    fn load(val: Self::StoredType) -> Self {
        Self::from_le_bytes(val[0])
    }
}

impl Storable for f32 {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

    fn store(self) -> Self::StoredType {
        [self.to_le_bytes()]
    }

    fn load(val: Self::StoredType) -> Self {
        Self::from_le_bytes(val[0])
    }
}

impl Storable for i32 {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

    fn store(self) -> Self::StoredType {
        [self.to_le_bytes()]
    }

    fn load(val: Self::StoredType) -> Self {
        Self::from_le_bytes(val[0])
    }
}

impl Storable for u64 {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

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

impl Storable for (u8, u8, u8, u8) {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

    fn store(self) -> Self::StoredType {
        let a = self.0.to_le_bytes();
        let b = self.1.to_le_bytes();
        let c = self.2.to_le_bytes();
        let d = self.3.to_le_bytes();
        [[a[0], b[0], c[0], d[0]]]
    }

    fn load(val: Self::StoredType) -> Self {
        let a = u8::from_le_bytes([val[0][0]]);
        let b = u8::from_le_bytes([val[0][1]]);
        let c = u8::from_le_bytes([val[0][2]]);
        let d = u8::from_le_bytes([val[0][3]]);
        (a, b, c, d)
    }
}

impl Storable for (u16, u8, u8) {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

    fn store(self) -> Self::StoredType {
        let a = self.0.to_le_bytes();
        let b = self.1.to_le_bytes();
        let c = self.2.to_le_bytes();
        [[a[0], a[1], b[0], c[0]]]
    }

    fn load(val: Self::StoredType) -> Self {
        let a = u16::from_le_bytes([val[0][0], val[0][1]]);
        let b = u8::from_le_bytes([val[0][1]]);
        let c = u8::from_le_bytes([val[0][2]]);
        (a, b, c)
    }
}

impl Storable for (u8, u8, u16) {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

    fn store(self) -> Self::StoredType {
        let a = self.0.to_le_bytes();
        let b = self.1.to_le_bytes();
        let c = self.2.to_le_bytes();
        [[a[0], b[0], c[0], c[1]]]
    }

    fn load(val: Self::StoredType) -> Self {
        let a = u8::from_le_bytes([val[0][0]]);
        let b = u8::from_le_bytes([val[0][1]]);
        let c = u16::from_le_bytes([val[0][2], val[0][3]]);
        (a, b, c)
    }
}

impl Storable for (u16, u16) {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

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

// impl<T, TT> Storable for (T, TT)
//     where
//         T: Storable,
//         TT: Storable,
// {
//     type StoredType = [[u8; 4]; <(T, TT)>::SIZE as usize];
//
//     fn store(self) -> Self::StoredType {
//         let l = self.0.store();
//         let r = self.1.store();
//         [l[0], r[0]]
//     }
//
//     fn load(val: Self::StoredType) -> Self {
//         let l = T::load(val[0]);
//         let r = TT::load(val[1]);
//         (l, r)
//     }
// }

impl Storable for Guid {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

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

impl Storable for Vector3d {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

    fn store(self) -> Self::StoredType {
        let x = self.x.to_le_bytes();
        let y = self.y.to_le_bytes();
        let z = self.z.to_le_bytes();
        let rotation = self.rotation.to_le_bytes();

        [x, y, z, rotation]
    }

    fn load(val: Self::StoredType) -> Self {
        let x = <_ as Storable>::load([val[0]]);
        let y = <_ as Storable>::load([val[1]]);
        let z = <_ as Storable>::load([val[2]]);
        let rotation = <_ as Storable>::load([val[3]]);
        Self { x, y, z, rotation }
    }
}

impl Storable for ItemEnchantment {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

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
        Self {
            charges,
            duration,
            id,
        }
    }
}

impl Storable for GameObjectBytes {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

    fn store(self) -> Self::StoredType {
        (
            self.state as u8,
            self.r#type as u8,
            self.art_kit as u8,
            self.anim_progress as u8,
        )
            .store()
    }

    fn load(val: Self::StoredType) -> Self {
        let (state, r#type, art_kit, anim_progress) = <(_, _, _, _) as Storable>::load([val[0]]);

        let state = GameObjectState::from_bytes((&state.to_le_bytes(), 0))
            .expect("Corrupted GameObjectState")
            .1;
        let r#type = GameObjectTypes::from_bytes((&r#type.to_le_bytes(), 8))
            .expect("Corrupted GameObjectTypes")
            .1;
        Self {
            state,
            r#type,
            art_kit,
            anim_progress,
        }
    }
}

impl Storable for UnitData {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

    fn store(self) -> Self::StoredType {
        (
            self.race as u8,
            self.class as u8,
            self.gender as u8,
            self.power as u8,
        )
            .store()
    }

    fn load(val: Self::StoredType) -> Self {
        let race = Race::from_bytes((&val[0], 0))
            .expect("Corrupted GameObjectState")
            .1;
        let class = Class::from_bytes((&val[0], 8))
            .expect("Corrupted GameObjectTypes")
            .1;
        let gender = Gender::from_bytes((&val[0], 16))
            .expect("Corrupted GameObjectTypes")
            .1;
        let power = Power::from_bytes((&val[0], 32))
            .expect("Corrupted GameObjectTypes")
            .1;
        Self {
            race,
            class,
            gender,
            power,
        }
    }
}

impl Storable for ClassSpecific {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

    fn store(self) -> Self::StoredType {
        (
            self.stand_state,
            self.pet_talents,
            self.vis_flag,
            self.anim_tier,
        )
            .store()
    }

    fn load(val: Self::StoredType) -> Self {
        let (stand_state, pet_talents, vis_flag, anim_tier) =
            <(_, _, _, _) as Storable>::load([val[0]]);
        Self {
            stand_state,
            pet_talents,
            vis_flag,
            anim_tier,
        }
    }
}

impl Storable for AttackPower {
    type StoredType = [[u8; 4]; Self::SIZE as usize];

    fn store(self) -> Self::StoredType {
        let power = self.power.to_le_bytes();
        let ab = <(_, _) as Storable>::store((self.a, self.b));
        let mul = self.mul.to_le_bytes();

        [power, ab[0], mul]
    }

    fn load(val: Self::StoredType) -> Self {
        let power = <_ as Storable>::load([val[0]]);
        let (a, b) = <_ as Storable>::load([val[1]]);
        let mul = <_ as Storable>::load([val[2]]);
        Self { power, a, b, mul }
    }
}



// impl Storable for Bytes1 {
//     type StoredType = [[u8; 4]; Self::SIZE as usize];
//
//     fn store(self) -> Self::StoredType {
//         (self.skin, self.face, self.hairstyle, self.hair_color).store()
//     }
//
//     fn load(val: Self::StoredType) -> Self {
//         let (skin, face, hairstyle, hair_color) = <(_, _, _, _) as Storable>::load([val[0]]);
//         Self {
//             skin,
//             face,
//             hairstyle,
//             hair_color,
//         }
//     }
// }
//
// impl Storable for Bytes2 {
//     type StoredType = [[u8; 4]; Self::SIZE as usize];
//
//     fn store(self) -> Self::StoredType {
//         (
//             self.facial,
//             self.party,
//             self.bank_bag_slots,
//             self.rest_state,
//         )
//             .store()
//     }
//
//     fn load(val: Self::StoredType) -> Self {
//         let (facial, party, bank_bag_slots, rest_state) =
//             <(_, _, _, _) as Storable>::load([val[0]]);
//
//         Self {
//             facial,
//             party,
//             bank_bag_slots,
//             rest_state,
//         }
//     }
// }
//
// impl Storable for Bytes3 {
//     type StoredType = [[u8; 4]; Self::SIZE as usize];
//
//     fn store(self) -> Self::StoredType {
//         (
//             self.gender,
//             self.inebriation,
//             self.pvp_title,
//             self.arena_faction,
//         )
//             .store()
//     }
//
//     fn load(val: Self::StoredType) -> Self {
//         let (gender, inebriation, pvp_title, arena_faction) =
//             <(_, _, _, _) as Storable>::load([val[0]]);
//
//         Self {
//             gender,
//             inebriation,
//             pvp_title,
//             arena_faction,
//         }
//     }
// }
//
// impl Storable for Bytes4 {
//     type StoredType = [[u8; 4]; Self::SIZE as usize];
//
//     fn store(self) -> Self::StoredType {
//         (
//             self.flags,
//             self.raf_grantable_level,
//             self.action_bar_toggles,
//             self.lifetime_max_pvp_rank,
//         )
//             .store()
//     }
//
//     fn load(val: Self::StoredType) -> Self {
//         let (flags, raf_grantable_level, action_bar_toggles, lifetime_max_pvp_rank) =
//             <(_, _, _, _) as Storable>::load([val[0]]);
//
//         Self {
//             flags,
//             raf_grantable_level,
//             action_bar_toggles,
//             lifetime_max_pvp_rank,
//         }
//     }
// }
//
// impl Storable for PlayerFieldBytes2Offsets {
//     type StoredType = [[u8; 4]; Self::SIZE as usize];
//
//     fn store(self) -> Self::StoredType {
//         (
//             self.override_spells_id,
//             self.ignore_power_regen_prediction_mask,
//             self.aura_vision,
//         )
//             .store()
//     }
//
//     fn load(val: Self::StoredType) -> Self {
//         let (override_spells_id, ignore_power_regen_prediction_mask, aura_vision) =
//             <(_, _, _) as Storable>::load([val[0]]);
//
//         Self {
//             override_spells_id,
//             ignore_power_regen_prediction_mask,
//             aura_vision,
//         }
//     }
// }
//
// impl Storable for QuestLogItem {
//     type StoredType = [[u8; 4]; Self::SIZE as usize];
//
//     fn store(self) -> Self::StoredType {
//         let id = self.id.to_le_bytes();
//         let state = self.state.to_le_bytes();
//         let counts = <(_, _) as Storable>::store((self.counts1, self.counts2));
//         let time = self.time.to_le_bytes();
//
//         [id, state, counts[0], time]
//     }
//
//     fn load(val: Self::StoredType) -> Self {
//         let id = <_ as Storable>::load([val[0]]);
//         let state = <_ as Storable>::load([val[1]]);
//         let (counts1, counts2) = <_ as Storable>::load([val[2]]);
//         let time = <_ as Storable>::load([val[3]]);
//         Self {
//             id,
//             state,
//             counts1,
//             counts2,
//             time,
//         }
//     }
// }
//
// impl Storable for PlayerEnchantment {
//     type StoredType = [[u8; 4]; Self::SIZE as usize];
//
//     fn store(self) -> Self::StoredType {
//         let id = self.id.to_le_bytes();
//         let permanent_temporary = <(_, _) as Storable>::store((self.permanent, self.temporary));
//         [id, permanent_temporary[0]]
//     }
//
//     fn load(val: Self::StoredType) -> Self {
//         let id = <_ as Storable>::load([val[0]]);
//         let (permanent, temporary) = <_ as Storable>::load([val[1]]);
//         Self {
//             id,
//             permanent,
//             temporary,
//         }
//     }
// }
//
// impl Storable for VisibleItem {
//     type StoredType = [[u8; 4]; Self::SIZE as usize];
//
//     fn store(self) -> Self::StoredType {
//         let entry_id = self.entry_id.to_le_bytes();
//         let enchantment = <PlayerEnchantment as Storable>::store(self.enchantment);
//         [entry_id, enchantment[0], enchantment[1]]
//     }
//
//     fn load(val: Self::StoredType) -> Self {
//         let entry_id = <_ as Storable>::load([val[0]]);
//         let enchantment = <_ as Storable>::load([val[1], val[2]]);
//         Self {
//             entry_id,
//             enchantment,
//         }
//     }
// }
//
// impl Storable for bool {
//     type StoredType = [[u8; 4]; Self::SIZE as usize];
//
//     fn store(self) -> Self::StoredType {
//         todo!()
//     }
//
//     fn load(val: Self::StoredType) -> Self {
//         todo!()
//     }
// }
//
// impl Storable for Rune {
//     type StoredType = [[u8; 4]; Self::SIZE as usize];
//
//     fn store(self) -> Self::StoredType {
//         todo!()
//     }
//
//     fn load(val: Self::StoredType) -> Self {
//         todo!()
//     }
// }
