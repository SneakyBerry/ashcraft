use crate::prelude::Guid;
use deku::prelude::*;

pub mod client {
    use super::*;

    #[derive(Debug, Clone, DekuRead)]
    pub struct CancelCast {
        pub cast_id: u8,
        pub spell_id: u32,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct CancelAura {
        pub spell_id: u32,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct PetCancelAura {
        pub pet_guid: Guid,
        pub spell_id: u32,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct CancelGrowthAura;

    #[derive(Debug, Clone, DekuRead)]
    pub struct CancelMountAura;

    #[derive(Debug, Clone, DekuRead)]
    pub struct CancelAutoRepeatSpell;

    #[derive(Debug, Clone, DekuRead)]
    pub struct CancelChannelling {
        pub channel_spell_id: u32,
    }
}

pub mod server {
    use super::*;
    use crate::define_flags;
    use crate::prelude::{Guid, Opcode, PackedGuid, Vector3d};
    use rustycraft_derive::ServerPacket;

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u8")]
    pub enum SpellMissInfo {
        #[deku(id = "0")]
        None,
        #[deku(id = "1")]
        Miss,
        #[deku(id = "2")]
        Resist,
        #[deku(id = "3")]
        Dodge,
        #[deku(id = "4")]
        Parry,
        #[deku(id = "5")]
        Block,
        #[deku(id = "6")]
        Evade,
        #[deku(id = "7")]
        Immune,
        #[deku(id = "8")]
        Immune2, // one of these 2 is MISS_TEMPIMMUNE
        #[deku(id = "9")]
        Deflect,
        #[deku(id = "10")]
        Absorb,
        #[deku(id = "11")]
        Reflect(u8),
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct SpellMissStatus {
        pub target: Guid,
        pub reason: SpellMissInfo,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct RuneData {
        pub start: u8,
        pub count: u8,
        pub cooldowns: Vec<u8>, // std::vector<uint8> Cooldowns;
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct MissileTrajectoryResult {
        pub pitch: f32,
        pub travel_time: u32,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct SpellAmmo {
        pub display_id: u32,
        pub inventory_type: u32,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct CreatureImmunities {
        pub school: u32,
        pub value: u32,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct TargetLocation {
        pub transport: PackedGuid,
        pub location: Vector3d,
    }

    define_flags!(SpellCastTargetFlags : u32 {
        NONE            = 0x00000000,
        UNUSED_1        = 0x00000001,               // not used
        UNIT            = 0x00000002,               // pguid
        UNIT_RAID       = 0x00000004,               // not sent, used to validate target (if raid member)
        UNIT_PARTY      = 0x00000008,               // not sent, used to validate target (if party member)
        ITEM            = 0x00000010,               // pguid
        SOURCE_LOCATION = 0x00000020,               // pguid, 3 float
        DEST_LOCATION   = 0x00000040,               // pguid, 3 float
        UNIT_ENEMY      = 0x00000080,               // not sent, used to validate target (if enemy)
        UNIT_ALLY       = 0x00000100,               // not sent, used to validate target (if ally)
        CORPSE_ENEMY    = 0x00000200,               // pguid
        UNIT_DEAD       = 0x00000400,               // not sent, used to validate target (if dead creature)
        GAMEOBJECT      = 0x00000800,               // pguid, used with TARGET_GAMEOBJECT_TARGET
        TRADE_ITEM      = 0x00001000,               // pguid
        STRING          = 0x00002000,               // string
        GAMEOBJECT_ITEM = 0x00004000,               // not sent, used with TARGET_GAMEOBJECT_ITEM_TARGET
        CORPSE_ALLY     = 0x00008000,               // pguid
        UNIT_MINIPET    = 0x00010000,               // pguid, used to validate target (if non combat pet)
        GLYPH_SLOT      = 0x00020000,               // used in glyph spells
        DEST_TARGET     = 0x00040000,               // sometimes appears with DEST_TARGET spells (may appear or not for a given spell)
        UNUSED20        = 0x00080000,               // uint32 counter, loop { vec3 - screen position (?), guid }, not used so far
        UNIT_PASSENGER  = 0x00100000,               // guessed, used to validate target (if vehicle passenger)

        UNIT_MASK = Self::UNIT | Self::UNIT_RAID | Self::UNIT_PARTY
            | Self::UNIT_ENEMY | Self::UNIT_ALLY | Self::UNIT_DEAD
            | Self::UNIT_MINIPET | Self::UNIT_PASSENGER,
        GAMEOBJECT_MASK = Self::GAMEOBJECT | Self::GAMEOBJECT_ITEM,
        CORPSE_MASK = Self::CORPSE_ALLY | Self::CORPSE_ENEMY,
        ITEM_MASK = Self::TRADE_ITEM | Self::ITEM | Self::GAMEOBJECT_ITEM
    });

    #[derive(Debug, Clone, DekuWrite)]
    pub struct SpellTargetData {
        pub flags: SpellCastTargetFlags,
        pub unit: Option<PackedGuid>,
        pub item: Option<PackedGuid>,
        pub src_location: Option<TargetLocation>,
        pub dst_location: Option<TargetLocation>,
        #[deku(
            writer = "if let Some(ref name) = self.name { crate::write_c_string(deku::output, name) } else { Ok(()) }"
        )]
        pub name: Option<String>,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct SizedVec<T>
    where
        T: DekuWrite,
    {
        size: u32,
        data: Vec<T>,
    }

    impl<T> From<Vec<T>> for SizedVec<T>
    where
        T: DekuWrite,
    {
        fn from(data: Vec<T>) -> Self {
            SizedVec {
                size: data.len() as u32,
                data,
            }
        }
    }

    define_flags!(SpellCastFlags: u32 {
        NONE               = 0x00000000,
        PENDING            = 0x00000001,              // aoe combat log?
        UNKNOWN_2          = 0x00000002,
        UNKNOWN_3          = 0x00000004,
        UNKNOWN_4          = 0x00000008,              // ignore AOE visual
        UNKNOWN_5          = 0x00000010,
        AMMO               = 0x00000020,              // Projectiles visual
        UNKNOWN_7          = 0x00000040,
        UNKNOWN_8          = 0x00000080,
        UNKNOWN_9          = 0x00000100,
        UNKNOWN_10         = 0x00000200,
        UNKNOWN_11         = 0x00000400,
        POWER_LEFT_SELF    = 0x00000800,
        UNKNOWN_13         = 0x00001000,
        UNKNOWN_14         = 0x00002000,
        UNKNOWN_15         = 0x00004000,
        UNKNOWN_16         = 0x00008000,
        UNKNOWN_17         = 0x00010000,
        ADJUST_MISSILE     = 0x00020000,
        NO_GCD             = 0x00040000,              // no GCD for spell casts from charm/summon (vehicle spells is an example)
        VISUAL_CHAIN       = 0x00080000,
        UNKNOWN_21         = 0x00100000,
        RUNE_LIST          = 0x00200000,
        UNKNOWN_23         = 0x00400000,
        UNKNOWN_24         = 0x00800000,
        UNKNOWN_25         = 0x01000000,
        UNKNOWN_26         = 0x02000000,
        IMMUNITY           = 0x04000000,
        UNKNOWN_28         = 0x08000000,
        UNKNOWN_29         = 0x10000000,
        UNKNOWN_30         = 0x20000000,
        UNKNOWN_31         = 0x40000000,
        UNKNOWN_32         = 0x80000000
    }
    );

    #[derive(Debug, Clone, DekuWrite)]
    pub struct SpellCastData {
        pub caster: PackedGuid,
        pub caster_unit: PackedGuid,
        pub cast_id: u8,
        pub spell_id: u32,
        pub cast_flags: SpellCastFlags,
        pub cast_time: u32,
        #[deku(
            update = "if let Some(mut hit_targets) = self.hit_targets.take() { hit_targets.update()?; Some(hit_targets) } else { None }"
        )]
        pub hit_targets: Option<SizedVec<Guid>>,
        #[deku(
            update = "if let Some(mut miss_status) = self.miss_status.take() { miss_status.update()?; Some(miss_status) } else { None }"
        )]
        pub miss_status: Option<SizedVec<SpellMissStatus>>,
        pub target: SpellTargetData,
        pub remaining_power: Option<u32>,
        pub remaining_runes: Option<RuneData>,
        pub missile_trajectory: Option<MissileTrajectoryResult>,
        pub ammo: Option<SpellAmmo>,
        pub immunities: Option<CreatureImmunities>,
        #[deku(cond = "!self.cast_flags.is_visual_chain()", skip)]
        #[deku(update = "0")]
        pub pad_1: u64,
        #[deku(cond = "!self.target.flags.is_dest_location()", skip)]
        #[deku(update = "0")]
        pub pad_2: u8,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgSpellGo)]
    pub struct SpellGo {
        pub data: SpellCastData,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgSpellStart)]
    pub struct SpellStart {
        pub data: SpellCastData,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct ResyncRune {
        pub rune: u8,
        pub cooldown: u8,
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone, ServerPacket)]
    #[opcode(Opcode::SmsgResyncRunes)]
    pub struct ResyncRunes {
        #[deku(temp, temp_value = "self.data.len() as u32")]
        pub count: u32,
        pub data: Vec<ResyncRune>,
    }
}
