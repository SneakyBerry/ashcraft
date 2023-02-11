use deku::prelude::*;

use crate::common::class::Class;
use crate::common::gender::Gender;
use crate::guid::Guid;
use crate::objects::size_helper::FieldSize;
use crate::objects::object::Object;
use crate::objects::UpdateFields;
use crate::power::Power;
use crate::race::Race;

use rustycraft_derive::IntoUpdateFields;

#[derive(Debug, Default, Clone, IntoUpdateFields, Builder)]
#[meta(offset = 0x0006, tag = 0x0009)]
pub struct Unit {
    #[nested]
    pub object: Object,
    pub charm: Option<Guid>,
    pub summon: Option<Guid>,
    pub critter: Option<Guid>,
    pub charmed_by: Option<Guid>,
    pub summoned_by: Option<Guid>,
    pub created_by: Option<Guid>,
    pub target: Option<Guid>,
    pub channel_object: Option<Guid>,
    pub channel_spell: Option<u32>,
    pub data: Option<UnitData>,
    pub health: Option<u32>,
    pub power: [Option<u32>; 7],
    pub max_health: Option<u32>,
    pub max_power: [Option<u32>; 7],
    pub power_regen_flat_modifier: [Option<u32>; 7],
    pub power_regen_interrupted_flat_modifier: [Option<u32>; 7],
    pub level: Option<u32>,
    pub faction_template: Option<u32>,
    pub virtual_item_slot_id: [Option<u32>; 3],
    pub flags: [Option<u32>; 2],
    pub aura_state: Option<u32>,
    pub attack_round_base_time: [Option<u32>; 2],
    pub ranged_attack_time: Option<f32>,
    pub bounding_radius: Option<f32>,
    pub combat_reach: Option<f32>,
    pub display_id: Option<u32>,
    pub native_display_id: Option<u32>,
    pub mount_display_id: Option<u32>,
    pub min_damage: Option<f32>,
    pub max_damage: Option<f32>,
    pub min_offhand_damage: Option<f32>,
    pub max_offhand_damage: Option<f32>,
    pub class_specific: Option<ClassSpecific>,
    pub pet_number: Option<u32>,
    pub pet_name_timestamp: Option<u32>,
    pub pet_experience: Option<u32>,
    pub pet_next_level_exp: Option<u32>,
    pub dynamic_flags: Option<u32>,
    pub mod_cast_speed: Option<f32>,
    pub created_by_spell: Option<u32>,
    pub npc_flags: Option<u32>,
    pub npc_emote_state: Option<u32>,
    pub stat: [Option<u32>; 5],
    pub stat_pos_effects: [Option<u32>; 5],
    pub stat_neg_effects: [Option<u32>; 5],
    pub resistance: [Option<u32>; 7],
    pub resistance_pos: [Option<u32>; 7],
    pub resistance_neg: [Option<u32>; 7],
    pub base_mana: Option<u32>,
    pub base_health: Option<u32>,
    pub bytes_2: Option<[u8; 4]>,
    pub attack_power_melee: Option<AttackPower>,
    pub attack_power_ranged: Option<AttackPower>,
    pub min_ranged_damage: Option<f32>,
    pub max_ranged_damage: Option<f32>,
    pub power_cost_modifier: [Option<u32>; 7],
    pub power_cost_multiplier: [Option<u32>; 7],
    pub max_health_modifier: Option<u32>,
    pub hover_height: Option<u32>,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct UnitData {
    pub race: Race,
    pub class: Class,
    pub gender: Gender,
    pub power: Power,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct ClassSpecific {
    pub stand_state: u8,
    pub pet_talents: u8,
    pub vis_flag: u8,
    pub anim_tier: u8,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]

pub struct AttackPower {
    pub power: u32,
    pub a: u16,
    pub b: u16,
    pub mul: f32,
}

#[repr(u32)]
pub enum UnitFlags {
    ServerControlled = 0x003B0001, // set only when unit movement is controlled by server - by SPLINE/MONSTER_MOVE packets, together with STUNNED; only set to units controlled by client; client function CGUnit_C::IsClientControlled returns false when set for owner
    NonAttackable = 0x003B0002, // not attackable, set when creature starts to cast spells with SPELL_EFFECT_SPAWN and cast time, removed when spell hits caster, original name is SPAWNING. Rename when it will be removed from all scripts
    RemoveClientControl = 0x003B0004, // This is a legacy flag used to disable movement player's movement while controlling other units, SMSG_CLIENT_CONTROL replaces this functionality clientside now. CONFUSED and FLEEING flags have the same effect on client movement asDISABLE_MOVE_CONTROL in addition to preventing spell casts/autoattack (they all allow climbing steeper hills and emotes while moving)
    PlayerControlled = 0x003B0008, // controlled by player, use _IMMUNE_TO_PC instead of _IMMUNE_TO_NPC
    Rename = 0x003B0010,
    Preparation = 0x003B0020, // don't take reagents for spells with SPELL_ATTR5_NO_REAGENT_WHILE_PREP
    Unk6 = 0x003B0040,
    NotAttackable1 = 0x003B0080, // ?? (PlayerControlled | NotAttackable1) is NON_PVP_ATTACKABLE
    ImmuneToPc = 0x003B0100, // disables combat/assistance with PlayerCharacters (PC) - see Unit::IsValidAttackTarget, Unit::IsValidAssistTarget
    ImmuneToNpc = 0x003B0200, // disables combat/assistance with NonPlayerCharacters (NPC) - see Unit::IsValidAttackTarget, Unit::IsValidAssistTarget
    Looting = 0x003B0400,     // loot animation
    PetInCombat = 0x003B0800, // on player pets: whether the pet is chasing a target to attack || on other units: whether any of the unit's minions is in combat
    PvpEnabling = 0x003B1000, // changed in 3.0.3, now UNIT_BYTES_2_OFFSET_PVP_FLAG from UNIT_FIELD_BYTES_2
    Silenced = 0x003B2000,    // silenced, 2.1.1
    CannotSwim = 0x003B4000,  // 2.0.8
    CanSwim = 0x003B8000,     // shows swim animation in water
    NonAttackable2 = 0x00010000, // removes attackable icon, if on yourself, cannot assist self but can cast TARGET_SELF spells - added by SPELL_AURA_MOD_UNATTACKABLE
    Pacified = 0x00020000,       // 3.0.3 ok
    Stunned = 0x00040000,        // 3.0.3 ok
    InCombat = 0x00080000,
    OnTaxi = 0x00100000, // disable casting at client side spell not allowed by taxi flight (mounted?), probably used with 0x4 flag
    Disarmed = 0x00200000, // 3.0.3, disable melee spells casting..., "Required melee weapon" added to melee spells tooltip.
    Confused = 0x00400000,
    Fleeing = 0x00800000,
    Possessed = 0x01000000, // under direct client control by a player (possess or vehicle)
    Uninteractible = 0x02000000,
    Scinnable = 0x04000000,
    Mount = 0x08000000,
    Unk28 = 0x10000000,
    PreventEmotesFromChatText = 0x20000000, // Prevent automatically playing emotes from parsing chat text, for example "lol" in /say, ending message with ? or !, or using /yell
    Sheathe = 0x40000000,
    Immune = 0x80000000, // Immune to damage

                         // DISALLOWED            = (ServerControlled | NonAttackable | RemoveClientControl |
                         // PlayerControlled | RENAME | PREPARATION | /* Unk6 | */
                         // NotAttackable1 | LOOTING | PetInCombat | PvpEnabling |
                         // SILENCED | NonAttackable2 | PACIFIED | STUNNED |
                         // InCombat | OnTaxi | DISARMED | CONFUSED | FLEEING |
                         // POSSESSED | SKINNABLE | MOUNT | Unk28 |
                         // PreventEmotesFromChatText | SHEATHE | IMMUNE),
                         //
                         // ALLOWED               = (0xFFFFFFFF & ~DISALLOWED)
}

#[repr(u32)]
pub enum UnitFlags2 {
    FeignDeath = 0x003B0001,
    HideBody = 0x003B0002, // Hide unit model (show only player equip)
    IgnoreReputation = 0x003B0004,
    ComprehendLang = 0x003B0008,
    MirrorImage = 0x003B0010,
    DoNotFadeIn = 0x003B0020, // Unit model instantly appears when summoned (does not fade in)
    ForceMovement = 0x003B0040,
    DisarmOffhand = 0x003B0080,
    DisablePredStats = 0x003B0100, // Player has disabled predicted stats (Used by raid frames)
    Unk1 = 0x003B0200,
    DisarmRanged = 0x003B0400, // this does not disable ranged weapon display (maybe additional flag needed?)
    RegeneratePower = 0x003B0800,
    RestrictPartyInteraction = 0x003B1000, // Restrict interaction to party or raid
    PreventSpellClick = 0x003B2000,        // Prevent spellclick
    AllowEnemyInteract = 0x003B4000,
    CannotTurn = 0x003B8000,
    Unk2 = 0x00010000,
    PlayDeathAnim = 0x00020000, // Plays special death animation upon death
    AllowCheatSpells = 0x00040000, // Allows casting spells with AttributesEx7 & SPELL_ATTR7_IS_CHEAT_SPELL
    Unused1 = 0x00080000,
    Unused2 = 0x00100000,
    Unused3 = 0x00200000,
    Unused4 = 0x00400000,
    Unused5 = 0x00800000,
    Unused6 = 0x01000000,
    Unused7 = 0x02000000,
    Unused8 = 0x04000000,
    Unused9 = 0x08000000,
    Unused10 = 0x10000000,
    Unused11 = 0x20000000,
    Unused12 = 0x40000000,
    Unused13 = 0x80000000,
}

#[repr(u32)]
pub enum UnitDynFlags {
    None = 0x003B,
    Lootable = 0x0001,
    TrackUnit = 0x0002,
    Tapped = 0x0004,         // Lua_UnitIsTapped
    TappedByPlayer = 0x0008, // Lua_UnitIsTappedByPlayer
    Specialinfo = 0x0010,
    Dead = 0x0020,
    ReferAFriend = 0x0040,
    TappedByAllThreatList = 0x0080, // Lua_UnitIsTappedByAllThreatList
}

#[derive(Debug, Clone)]
#[repr(u32)]
pub enum NPCFlags {
    None = 0x003B0000,       // SKIP
    Gossip = 0x003B0001,     // TITLE has gossip menu DESCRIPTION 100%
    QuestGiver = 0x003B0002, // TITLE is quest giver DESCRIPTION guessed, probably ok
    Unk1 = 0x003B0004,
    Unk2 = 0x003B0008,
    Trainer = 0x003B0010,           // TITLE is trainer DESCRIPTION 100%
    TrainerClass = 0x003B0020,      // TITLE is class trainer DESCRIPTION 100%
    TrainerProfession = 0x003B0040, // TITLE is profession trainer DESCRIPTION 100%
    Vendor = 0x003B0080,            // TITLE is vendor (generic) DESCRIPTION 100%
    VendorAmmo = 0x003B0100,        // TITLE is vendor (ammo) DESCRIPTION 100%, general goods vendor
    VendorFood = 0x003B0200,        // TITLE is vendor (food) DESCRIPTION 100%
    VendorPoison = 0x003B0400,      // TITLE is vendor (poison) DESCRIPTION guessed
    VendorReagent = 0x003B0800,     // TITLE is vendor (reagents) DESCRIPTION 100%
    Repair = 0x003B1000,            // TITLE can repair DESCRIPTION 100%
    FlightMaster = 0x003B2000,      // TITLE is flight master DESCRIPTION 100%
    SpiritHealer = 0x003B4000,      // TITLE is spirit healer DESCRIPTION guessed
    SpiritGuide = 0x003B8000,       // TITLE is spirit guide DESCRIPTION guessed
    Innkeeper = 0x00010000,         // TITLE is innkeeper
    Banker = 0x00020000,            // TITLE is banker DESCRIPTION 100%
    Petitioner = 0x00040000, // TITLE handles guild/arena petitions DESCRIPTION 100% 0xC0000 = guild petitions, 0x40000 = arena team petitions
    TabardDesigner = 0x00080000, // TITLE is guild tabard designer DESCRIPTION 100%
    BattleMaster = 0x00100000, // TITLE is battlemaster DESCRIPTION 100%
    Auctioneer = 0x00200000, // TITLE is auctioneer DESCRIPTION 100%
    StableMaster = 0x00400000, // TITLE is stable master DESCRIPTION 100%
    GuildBanker = 0x00800000, // TITLE is guild banker DESCRIPTION cause client to send 997 opcode
    SpellClick = 0x01000000, // TITLE has spell click enabled DESCRIPTION cause client to send 1015 opcode (spell click)
    PlayerVehicle = 0x02000000, // TITLE is player vehicle DESCRIPTION players with mounts that have vehicle data should have it set
    Mailbox = 0x04000000,       // TITLE is mailbox
}
