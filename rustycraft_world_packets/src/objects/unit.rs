use deku::prelude::*;
use rustycraft_derive::CalcUpdate;

use crate::common::class::Class;
use crate::common::gender::Gender;
use crate::guid::Guid;
use crate::objects::object::ObjectUpdate;
use crate::power::Power;
use crate::race::Race;

use crate::common::emotes::Emote;
use crate::define_flags;
use crate::objects::helpers::ArrayWrapped;

#[derive(Debug, Default, Clone, CalcUpdate, Builder)]
#[meta(offset = 0x0006, tag = 0x0009)]
pub struct UnitUpdate {
    #[nested]
    pub object: ObjectUpdate,
    pub charm: Guid,
    pub summon: Guid,
    pub critter: Guid,
    pub charmed_by: Guid,
    pub summoned_by: Guid,
    pub created_by: Guid,
    pub target: Guid,
    pub channel_object: Guid,
    pub channel_spell: u32,
    pub data: UnitData,
    pub health: u32,
    pub power: ArrayWrapped<u32, 7>,
    pub max_health: u32,
    pub max_power: ArrayWrapped<u32, 7>,
    pub power_regen_flat_modifier: ArrayWrapped<u32, 7>,
    pub power_regen_interrupted_flat_modifier: ArrayWrapped<u32, 7>,
    pub level: u32,
    pub faction_template: u32,
    pub virtual_item_slot_id: ArrayWrapped<u32, 3>,
    pub flags_1: UnitFlags,
    pub flags_2: UnitFlags2,
    pub aura_state: u32,
    pub attack_round_base_time: ArrayWrapped<u32, 2>,
    pub ranged_attack_time: f32,
    pub bounding_radius: f32,
    pub combat_reach: f32,
    pub display_id: u32,
    pub native_display_id: u32,
    pub mount_display_id: u32,
    pub min_damage: f32,
    pub max_damage: f32,
    pub min_offhand_damage: f32,
    pub max_offhand_damage: f32,
    pub class_specific: ClassSpecific,
    pub pet_number: u32,
    pub pet_name_timestamp: u32,
    pub pet_experience: u32,
    pub pet_next_level_exp: u32,
    pub dynamic_flags: UnitDynFlags,
    pub mod_cast_speed: f32,
    pub created_by_spell: u32,
    pub npc_flags: NPCFlags,
    pub emote_state: Emote,
    pub stat: ArrayWrapped<u32, 5>,
    pub stat_pos_effects: ArrayWrapped<u32, 5>,
    pub stat_neg_effects: ArrayWrapped<u32, 5>,
    pub resistance: ArrayWrapped<u32, 7>,
    pub resistance_pos: ArrayWrapped<u32, 7>,
    pub resistance_neg: ArrayWrapped<u32, 7>,
    pub base_mana: u32,
    pub base_health: u32,
    pub bytes_2: [u8; 4],
    pub attack_power_melee: AttackPower,
    pub attack_power_ranged: AttackPower,
    pub min_ranged_damage: f32,
    pub max_ranged_damage: f32,
    pub power_cost_modifier: ArrayWrapped<u32, 7>,
    pub power_cost_multiplier: ArrayWrapped<u32, 7>,
    pub max_health_modifier: u32,
    pub hover_height: u32,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct UnitData {
    pub race: Race,
    pub class: Class,
    pub gender: Gender,
    pub power: Power,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct ClassSpecific {
    pub stand_state: u8,
    pub pet_talents: u8,
    pub vis_flag: u8,
    pub anim_tier: u8,
}

#[derive(Debug, Default, Clone, PartialEq, DekuRead, DekuWrite)]
pub struct AttackPower {
    pub power: u32,
    pub a: u16,
    pub b: u16,
    pub mul: f32,
}

pub enum AuraType {
    None = 0,
    BindSight = 1,
    ModPossess = 2,
    PeriodicDamage = 3,
    Dummy = 4,
    ModConfuse = 5,
    ModCharm = 6,
    ModFear = 7,
    PeriodicHeal = 8,
    ModAttackspeed = 9,
    ModThreat = 10,
    ModTaunt = 11,
    ModStun = 12,
    ModDamageDone = 13,
    ModDamageTaken = 14,
    DamageShield = 15,
    ModStealth = 16,
    ModStealthDetect = 17,
    ModInvisibility = 18,
    ModInvisibilityDetect = 19,
    ObsModHealth = 20, // 20, 21 unofficial
    ObsModPower = 21,
    ModResistance = 22,
    PeriodicTriggerSpell = 23,
    PeriodicEnergize = 24,
    ModPacify = 25,
    ModRoot = 26,
    ModSilence = 27,
    ReflectSpells = 28,
    ModStat = 29,
    ModSkill = 30,
    ModIncreaseSpeed = 31,
    ModIncreaseMountedSpeed = 32,
    ModDecreaseSpeed = 33,
    ModIncreaseHealth = 34,
    ModIncreaseEnergy = 35,
    ModShapeshift = 36,
    EffectImmunity = 37,
    StateImmunity = 38,
    SchoolImmunity = 39,
    DamageImmunity = 40,
    DispelImmunity = 41,
    ProcTriggerSpell = 42,
    ProcTriggerDamage = 43,
    TrackCreatures = 44,
    TrackResources = 45,
    State46 = 46, // Ignore all Gear test spells
    ModParryPercent = 47,
    PeriodicTriggerSpellFromClient = 48, // One periodic spell
    ModDodgePercent = 49,
    ModCriticalHealingAmount = 50,
    ModBlockPercent = 51,
    ModWeaponCritPercent = 52,
    PeriodicLeech = 53,
    ModHitChance = 54,
    ModSpellHitChance = 55,
    Transform = 56,
    ModSpellCritChance = 57,
    ModIncreaseSwimSpeed = 58,
    ModDamageDoneCreature = 59,
    ModPacifySilence = 60,
    ModScale = 61,
    PeriodicHealthFunnel = 62,
    State63 = 63, // old SPELL_AURA_PERIODIC_MANA_FUNNEL
    PeriodicManaLeech = 64,
    ModCastingSpeedNotStack = 65,
    FeignDeath = 66,
    ModDisarm = 67,
    ModStalked = 68,
    SchoolAbsorb = 69,
    ExtraAttacks = 70,
    ModSpellCritChanceSchool = 71,
    ModPowerCostSchoolPct = 72,
    ModPowerCostSchool = 73,
    ReflectSpellsSchool = 74,
    ModLanguage = 75,
    FarSight = 76,
    MechanicImmunity = 77,
    Mounted = 78,
    ModDamagePercentDone = 79,
    ModPercentStat = 80,
    SplitDamagePct = 81,
    WaterBreathing = 82,
    ModBaseResistance = 83,
    ModRegen = 84,
    ModPowerRegen = 85,
    ChannelDeathItem = 86,
    ModDamagePercentTaken = 87,
    ModHealthRegenPercent = 88,
    PeriodicDamagePercent = 89,
    State90 = 90, // old SPELL_AURA_MOD_RESIST_CHANCE
    ModDetectRange = 91,
    PreventsFleeing = 92,
    ModUnattackable = 93,
    InterruptRegen = 94,
    Ghost = 95,
    SpellMagnet = 96,
    ManaShield = 97,
    ModSkillTalent = 98,
    ModAttackPower = 99,
    AurasVisible = 100,
    ModResistancePct = 101,
    ModMeleeAttackPowerVersus = 102,
    ModTotalThreat = 103,
    WaterWalk = 104,
    FeatherFall = 105,
    Hover = 106,
    AddFlatModifier = 107,
    AddPctModifier = 108,
    AddTargetTrigger = 109,
    ModPowerRegenPercent = 110,
    AddCasterHitTrigger = 111,
    OverrideClassScripts = 112,
    ModRangedDamageTaken = 113,
    ModRangedDamageTakenPct = 114,
    ModHealing = 115,
    ModRegenDuringCombat = 116,
    ModMechanicResistance = 117,
    ModHealingPct = 118,
    State119 = 119, // old SPELL_AURA_SHARE_PET_TRACKING
    Untrackable = 120,
    Empathy = 121,
    ModOffhandDamagePct = 122,
    ModTargetResistance = 123,
    ModRangedAttackPower = 124,
    ModMeleeDamageTaken = 125,
    ModMeleeDamageTakenPct = 126,
    RangedAttackPowerAttackerBonus = 127,
    ModPossessPet = 128,
    ModSpeedAlways = 129,
    ModMountedSpeedAlways = 130,
    ModRangedAttackPowerVersus = 131,
    ModIncreaseEnergyPercent = 132,
    ModIncreaseHealthPercent = 133,
    ModManaRegenInterrupt = 134,
    ModHealingDone = 135,
    ModHealingDonePercent = 136,
    ModTotalStatPercentage = 137,
    ModMeleeHaste = 138,
    ForceReaction = 139,
    ModRangedHaste = 140,
    ModRangedAmmoHaste = 141,
    ModBaseResistancePct = 142,
    ModResistanceExclusive = 143,
    SafeFall = 144,
    ModPetTalentPoints = 145,
    AllowTamePetType = 146,
    MechanicImmunityMask = 147,
    RetainComboPoints = 148,
    ReducePushback = 149, //    Reduce Pushback
    ModShieldBlockvaluePct = 150,
    TrackStealthed = 151,    //    Track Stealthed
    ModDetectedRange = 152,  //    Mod Detected Range
    SplitDamageFlat = 153,   //    Split Damage Flat
    ModStealthLevel = 154,   //    Stealth Level Modifier
    ModWaterBreathing = 155, //    Mod Water Breathing
    ModReputationGain = 156, //    Mod Reputation Gain
    PetDamageMulti = 157,    //    Mod Pet Damage
    ModShieldBlockvalue = 158,
    NoPvpCredit = 159,
    ModAoeAvoidance = 160,
    ModHealthRegenInCombat = 161,
    PowerBurn = 162,
    ModCritDamageBonus = 163,
    State164 = 164,
    MeleeAttackPowerAttackerBonus = 165,
    ModAttackPowerPct = 166,
    ModRangedAttackPowerPct = 167,
    ModDamageDoneVersus = 168,
    ModCritPercentVersus = 169,
    DetectAmore = 170,
    ModSpeedNotStack = 171,
    ModMountedSpeedNotStack = 172,
    State173 = 173,                    // old SPELL_AURA_ALLOW_CHAMPION_SPELLS
    ModSpellDamageOfStatPercent = 174, // by defeult intelect, dependent from ModSpellHealingOfStatPercent
    ModSpellHealingOfStatPercent = 175,
    SpiritOfRedemption = 176,
    AoeCharm = 177,
    ModDebuffResistance = 178,
    ModAttackerSpellCritChance = 179,
    ModFlatSpellDamageVersus = 180,
    State181 = 181, // old SPELL_AURA_MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS - possible flat spell crit damage versus
    ModResistanceOfStatPercent = 182,
    ModCriticalThreat = 183,
    ModAttackerMeleeHitChance = 184,
    ModAttackerRangedHitChance = 185,
    ModAttackerSpellHitChance = 186,
    ModAttackerMeleeCritChance = 187,
    ModAttackerRangedCritChance = 188,
    ModRating = 189,
    ModFactionReputationGain = 190,
    UseNormalMovementSpeed = 191,
    ModMeleeRangedHaste = 192,
    MeleeSlow = 193,
    ModTargetAbsorbSchool = 194,
    ModTargetAbilityAbsorbSchool = 195,
    ModCooldown = 196, // only 24818 Noxious Breath
    ModAttackerSpellAndWeaponCritChance = 197,
    State198 = 198, // old SPELL_AURA_MOD_ALL_WEAPON_SKILLS
    ModIncreasesSpellPctToHit = 199,
    ModXpPct = 200,
    Fly = 201,
    IgnoreCombatResult = 202,
    ModAttackerMeleeCritDamage = 203,
    ModAttackerRangedCritDamage = 204,
    ModSchoolCritDmgTaken = 205,
    ModIncreaseVehicleFlightSpeed = 206,
    ModIncreaseMountedFlightSpeed = 207,
    ModIncreaseFlightSpeed = 208,
    ModMountedFlightSpeedAlways = 209,
    ModVehicleSpeedAlways = 210,
    ModFlightSpeedNotStack = 211,
    ModRangedAttackPowerOfStatPercent = 212,
    ModRageFromDamageDealt = 213,
    State214 = 214,
    ArenaPreparation = 215,
    HasteSpells = 216,
    ModMeleeHaste2 = 217, // NYI
    HasteRanged = 218,
    ModManaRegenFromStat = 219,
    ModRatingFromStat = 220,
    ModDetaunt = 221,
    State222 = 222,
    RaidProcFromCharge = 223,
    State224 = 224,
    RaidProcFromChargeWithValue = 225,
    PeriodicDummy = 226,
    PeriodicTriggerSpellWithValue = 227,
    DetectStealth = 228,
    ModAoeDamageAvoidance = 229,
    State230 = 230,
    ProcTriggerSpellWithValue = 231,
    MechanicDurationMod = 232,
    ChangeModelForAllHumanoids = 233, // client-side only
    MechanicDurationModNotStack = 234,
    ModDispelResist = 235,
    ControlVehicle = 236,
    ModSpellDamageOfAttackPower = 237,
    ModSpellHealingOfAttackPower = 238,
    ModScale2 = 239,
    ModExpertise = 240,
    ForceMoveForward = 241,
    ModSpellDamageFromHealing = 242,
    ModFaction = 243,
    ComprehendLanguage = 244,
    ModAuraDurationByDispel = 245,
    ModAuraDurationByDispelNotStack = 246,
    CloneCaster = 247,
    ModCombatResultChance = 248,
    ConvertRune = 249,
    ModIncreaseHealth2 = 250,
    ModEnemyDodge = 251,
    ModSpeedSlowAll = 252,
    ModBlockCritChance = 253,
    ModDisarmOffhand = 254,
    ModMechanicDamageTakenPercent = 255,
    NoReagentUse = 256,
    ModTargetResistBySpellClass = 257,
    State258 = 258,
    ModHotPct = 259,
    ScreenEffect = 260,
    Phase = 261,
    AbilityIgnoreAurastate = 262,
    AllowOnlyAbility = 263,
    State264 = 264,
    State265 = 265,
    State266 = 266,
    ModImmuneAuraApplySchool = 267,
    ModAttackPowerOfStatPercent = 268,
    ModIgnoreTargetResist = 269,
    ModAbilityIgnoreTargetResist = 270, // Possibly need swap vs 195 aura used only in 1 spell Chaos Bolt Passive
    ModDamageFromCaster = 271,
    IgnoreMeleeReset = 272,
    XRay = 273,
    AbilityConsumeNoAmmo = 274,
    ModIgnoreShapeshift = 275,
    ModDamageDoneForMechanic = 276, // NYI
    ModMaxAffectedTargets = 277,
    ModDisarmRanged = 278,
    InitializeImages = 279,
    ModArmorPenetrationPct = 280,
    ModHonorGainPct = 281,
    ModBaseHealthPct = 282,
    ModHealingReceived = 283, // Possibly only for some spell family class spells
    Linked = 284,
    ModAttackPowerOfArmor = 285,
    AbilityPeriodicCrit = 286,
    DeflectSpells = 287,
    IgnoreHitDirection = 288,
    PreventDurabilityLoss = 289,
    ModCritPct = 290,
    ModXpQuestPct = 291,
    OpenStable = 292,
    OverrideSpells = 293,
    PreventRegeneratePower = 294,
    State295 = 295,
    SetVehicleId = 296,
    BlockSpellFamily = 297,
    Strangulate = 298,
    State299 = 299,
    ShareDamagePct = 300,
    SchoolHealAbsorb = 301,
    State302 = 302,
    ModDamageDoneVersusAurastate = 303,
    ModFakeInebriate = 304,
    ModMinimumSpeed = 305,
    State306 = 306,
    HealAbsorbTest = 307,
    ModCritChanceForCaster = 308,
    State309 = 309,
    ModCreatureAoeDamageAvoidance = 310,
    State311 = 311,
    State312 = 312,
    State313 = 313,
    PreventResurrection = 314,
    UnderwaterWalking = 315,
    PeriodicHaste = 316,
}

define_flags!(
    StructName: UnitFlags
    InnerType: u32 {
        SERVER_CONTROLLED     = 0x00000001,           // set only when unit movement is controlled by server - by SPLINE/MONSTER_MOVE packets, together with STUNNED; only set to units controlled by client; client function CGUnit_C::IsClientControlled returns false when set for owner
        NON_ATTACKABLE        = 0x00000002,           // not attackable, set when creature starts to cast spells with SPELL_EFFECT_SPAWN and cast time, removed when spell hits caster, original name is SPAWNING. Rename when it will be removed from all scripts
        REMOVE_CLIENT_CONTROL = 0x00000004,           // This is a legacy flag used to disable movement player's movement while controlling other units, SMSG_CLIENT_CONTROL replaces this functionality clientside now. CONFUSED and FLEEING flags have the same effect on client movement asDISABLE_MOVE_CONTROL in addition to preventing spell casts/autoattack (they all allow climbing steeper hills and emotes while moving)
        PLAYER_CONTROLLED     = 0x00000008,           // controlled by player, use _IMMUNE_TO_PC instead of _IMMUNE_TO_NPC
        RENAME                = 0x00000010,
        PREPARATION           = 0x00000020,           // don't take reagents for spells with SPELL_ATTR5_NO_REAGENT_WHILE_PREP
        UNK_6                 = 0x00000040,
        NOT_ATTACKABLE_1      = 0x00000080,           // ?? (PLAYER_CONTROLLED | NOT_ATTACKABLE_1) is NON_PVP_ATTACKABLE
        IMMUNE_TO_PC          = 0x00000100,           // disables combat/assistance with PlayerCharacters (PC) - see Unit::IsValidAttackTarget, Unit::IsValidAssistTarget
        IMMUNE_TO_NPC         = 0x00000200,           // disables combat/assistance with NonPlayerCharacters (NPC) - see Unit::IsValidAttackTarget, Unit::IsValidAssistTarget
        LOOTING               = 0x00000400,           // loot animation
        PET_IN_COMBAT         = 0x00000800,           // on player pets: whether the pet is chasing a target to attack || on other units: whether any of the unit's minions is in combat
        PVP_ENABLING          = 0x00001000,           // changed in 3.0.3, now UNIT_BYTES_2_OFFSET_PVP_FLAG from UNIT_FIELD_BYTES_2
        SILENCED              = 0x00002000,           // silenced, 2.1.1
        CANNOT_SWIM           = 0x00004000,           // 2.0.8
        CAN_SWIM              = 0x00008000,           // shows swim animation in water
        NON_ATTACKABLE_2      = 0x00010000,           // removes attackable icon, if on yourself, cannot assist self but can cast TARGET_SELF spells - added by AuraType::ModUnattackable
        PACIFIED              = 0x00020000,           // 3.0.3 ok
        STUNNED               = 0x00040000,           // 3.0.3 ok
        IN_COMBAT             = 0x00080000,
        ON_TAXI               = 0x00100000,           // disable casting at client side spell not allowed by taxi flight (mounted?), probably used with 0x4 flag
        DISARMED              = 0x00200000,           // 3.0.3, disable melee spells casting..., "Required melee weapon" added to melee spells tooltip.
        CONFUSED              = 0x00400000,
        FLEEING               = 0x00800000,
        POSSESSED             = 0x01000000,           // under direct client control by a player (possess or vehicle)
        UNINTERACTIBLE        = 0x02000000,
        SKINNABLE             = 0x04000000,
        MOUNT                 = 0x08000000,
        UNK_28                = 0x10000000,
        PREVENT_EMOTES_FROM_CHAT_TEXT = 0x20000000,   // Prevent automatically playing emotes from parsing chat text, for example "lol" in /say, ending message with ? or !, or using /yell
        SHEATHE               = 0x40000000,
        IMMUNE                = 0x80000000,           // Immune to damage

        DISALLOWED            = (Self::SERVER_CONTROLLED | Self::NON_ATTACKABLE | Self::REMOVE_CLIENT_CONTROL |
                                           Self::PLAYER_CONTROLLED | Self::RENAME | Self::PREPARATION | /* UNK_6 | */
                                           Self::NOT_ATTACKABLE_1 | Self::LOOTING | Self::PET_IN_COMBAT | Self::PVP_ENABLING |
                                           Self::SILENCED | Self::NON_ATTACKABLE_2 | Self::PACIFIED | Self::STUNNED |
                                           Self::IN_COMBAT | Self::ON_TAXI | Self::DISARMED | Self::CONFUSED | Self::FLEEING |
                                           Self::POSSESSED | Self::SKINNABLE | Self::MOUNT | Self::UNK_28 |
                                           Self::PREVENT_EMOTES_FROM_CHAT_TEXT | Self::SHEATHE | Self::IMMUNE),

        ALLOWED               = (0xFFFFFFFF & !Self::DISALLOWED)
    }
);

define_flags!(
    StructName: UnitFlags2
    InnerType: u32 {
        FEIGN_DEATH                  = 0x00000001,
        HIDE_BODY                    = 0x00000002,   // Hide unit model (show only player equip)
        IGNORE_REPUTATION            = 0x00000004,
        COMPREHEND_LANG              = 0x00000008,
        MIRROR_IMAGE                 = 0x00000010,
        DO_NOT_FADE_IN               = 0x00000020,   // Unit model instantly appears when summoned (does not fade in)
        FORCE_MOVEMENT               = 0x00000040,
        DISARM_OFFHAND               = 0x00000080,
        DISABLE_PRED_STATS           = 0x00000100,   // Player has disabled predicted stats (Used by raid frames)
        UNK_1                        = 0x00000200,
        DISARM_RANGED                = 0x00000400,   // this does not disable ranged weapon display (maybe additional flag needed?)
        REGENERATE_POWER             = 0x00000800,
        RESTRICT_PARTY_INTERACTION   = 0x00001000,   // Restrict interaction to party or raid
        PREVENT_SPELL_CLICK          = 0x00002000,   // Prevent spellclick
        ALLOW_ENEMY_INTERACT         = 0x00004000,
        CANNOT_TURN                  = 0x00008000,
        UNK2                         = 0x00010000,
        PLAY_DEATH_ANIM              = 0x00020000,   // Plays special death animation upon death
        ALLOW_CHEAT_SPELLS           = 0x00040000,   // Allows casting spells with AttributesEx7 & SPELL_ATTR7_IS_CHEAT_SPELL
        UNUSED_1                     = 0x00080000,
        UNUSED_2                     = 0x00100000,
        UNUSED_3                     = 0x00200000,
        UNUSED_4                     = 0x00400000,
        UNUSED_5                     = 0x00800000,
        UNUSED_6                     = 0x01000000,
        UNUSED_7                     = 0x02000000,
        UNUSED_8                     = 0x04000000,
        UNUSED_9                     = 0x08000000,
        UNUSED_10                    = 0x10000000,
        UNUSED_11                    = 0x20000000,
        UNUSED_12                    = 0x40000000,
        UNUSED_13                    = 0x80000000,

        DISALLOWED                   = (Self::FEIGN_DEATH | Self::IGNORE_REPUTATION | Self::COMPREHEND_LANG |
                                                   Self::MIRROR_IMAGE | Self::FORCE_MOVEMENT | Self::DISARM_OFFHAND |
                                                   Self::DISABLE_PRED_STATS | Self::UNK_1 | Self::DISARM_RANGED |
                                                /* REGENERATE_POWER | */ Self::RESTRICT_PARTY_INTERACTION |
                                                   Self::PREVENT_SPELL_CLICK | Self::ALLOW_ENEMY_INTERACT | /* UNK2 | */
                                                /* PLAY_DEATH_ANIM | */ Self::ALLOW_CHEAT_SPELLS | Self::UNUSED_1 |
                                                   Self::UNUSED_2 | Self::UNUSED_3 | Self::UNUSED_4 | Self::UNUSED_5 |
                                                   Self::UNUSED_6 | Self::UNUSED_7 | Self::UNUSED_8 | Self::UNUSED_9 |
                                                   Self::UNUSED_10 | Self::UNUSED_11 | Self::UNUSED_12 | Self::UNUSED_13),

        ALLOWED                      = (0xFFFFFFFF & !Self::DISALLOWED)
    }
);

define_flags!(
    StructName: NPCFlags
    InnerType: u32 {
        NONE                  = 0x00000000,       // SKIP
        GOSSIP                = 0x00000001,       // TITLE has gossip menu DESCRIPTION 100%
        QUESTGIVER            = 0x00000002,       // TITLE is quest giver DESCRIPTION guessed, probably ok
        UNK1                  = 0x00000004,
        UNK2                  = 0x00000008,
        TRAINER               = 0x00000010,       // TITLE is trainer DESCRIPTION 100%
        TRAINER_CLASS         = 0x00000020,       // TITLE is class trainer DESCRIPTION 100%
        TRAINER_PROFESSION    = 0x00000040,       // TITLE is profession trainer DESCRIPTION 100%
        VENDOR                = 0x00000080,       // TITLE is vendor (generic) DESCRIPTION 100%
        VENDOR_AMMO           = 0x00000100,       // TITLE is vendor (ammo) DESCRIPTION 100%, general goods vendor
        VENDOR_FOOD           = 0x00000200,       // TITLE is vendor (food) DESCRIPTION 100%
        VENDOR_POISON         = 0x00000400,       // TITLE is vendor (poison) DESCRIPTION guessed
        VENDOR_REAGENT        = 0x00000800,       // TITLE is vendor (reagents) DESCRIPTION 100%
        REPAIR                = 0x00001000,       // TITLE can repair DESCRIPTION 100%
        FLIGHTMASTER          = 0x00002000,       // TITLE is flight master DESCRIPTION 100%
        SPIRITHEALER          = 0x00004000,       // TITLE is spirit healer DESCRIPTION guessed
        SPIRITGUIDE           = 0x00008000,       // TITLE is spirit guide DESCRIPTION guessed
        INNKEEPER             = 0x00010000,       // TITLE is innkeeper
        BANKER                = 0x00020000,       // TITLE is banker DESCRIPTION 100%
        PETITIONER            = 0x00040000,       // TITLE handles guild/arena petitions DESCRIPTION 100% 0xC0000 = guild petitions, 0x40000 = arena team petitions
        TABARDDESIGNER        = 0x00080000,       // TITLE is guild tabard designer DESCRIPTION 100%
        BATTLEMASTER          = 0x00100000,       // TITLE is battlemaster DESCRIPTION 100%
        AUCTIONEER            = 0x00200000,       // TITLE is auctioneer DESCRIPTION 100%
        STABLEMASTER          = 0x00400000,       // TITLE is stable master DESCRIPTION 100%
        GUILD_BANKER          = 0x00800000,       // TITLE is guild banker DESCRIPTION cause client to send 997 opcode
        SPELLCLICK            = 0x01000000,       // TITLE has spell click enabled DESCRIPTION cause client to send 1015 opcode (spell click)
        PLAYER_VEHICLE        = 0x02000000,       // TITLE is player vehicle DESCRIPTION players with mounts that have vehicle data should have it set
        MAILBOX               = 0x04000000        // TITLE is mailbox
    }
);

define_flags!(
    StructName: UnitDynFlags
    InnerType: u32 {
        NONE                       = 0x0000,
        LOOTABLE                   = 0x0001,
        TRACK_UNIT                 = 0x0002,
        TAPPED                     = 0x0004,       // Lua_UnitIsTapped
        TAPPED_BY_PLAYER           = 0x0008,       // Lua_UnitIsTappedByPlayer
        SPECIALINFO                = 0x0010,
        DEAD                       = 0x0020,
        REFER_A_FRIEND             = 0x0040,
        TAPPED_BY_ALL_THREAT_LIST  = 0x0080        // Lua_UnitIsTappedByAllThreatList
    }
);
