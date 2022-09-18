use crate::class::Class;
use crate::gender::Gender;

use crate::power::Power;
use crate::race::Race;

#[derive(Debug, Clone)]
pub struct UnitData {
    pub race: Race,
    pub class: Class,
    pub gender: Gender,
    pub power: Power,
}

#[derive(Debug, Clone)]
pub struct ClassSpecific {
    pub stand_state: u8,
    pub pet_talents: u8,
    pub vis_flag: u8,
    pub anim_tier: u8,
}

#[derive(Debug, Clone)]
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

macro_rules! unit_fields {
    (
        impl for $struct_name:ident
    ) => {
        impl_accessors!(
            Offset: 0x0006;
            Size: 0x008E;
            impl $struct_name {
                0x0000 => unit_charm: Guid;
                0x0002 => unit_summon: Guid;
                0x0004 => unit_critter: Guid;
                0x0006 => unit_charmed_by: Guid;
                0x0008 => unit_summoned_by: Guid;
                0x000A => unit_created_by: Guid;
                0x000C => unit_target: Guid;

                0x000E => unit_channel_object: Guid;
                0x0010 => unit_channel_spell: u32;

                0x0011 => unit_unit_data: UnitData;

                0x0012 => unit_health: u32;
                0x0013 => unit_power: [u32; 7];                           // [mana, rage, focus, energy, happiness, runes, runic_power]
                0x001A => unit_max_health: u32;
                0x001B => unit_max_power: [u32; 7];                       // [mana, rage, focus, energy, happiness, runes, runic_power]
                0x0022 => unit_regen_flat_mod: [u32; 7];                  // [mana, rage, focus, energy, happiness, runes, runic_power]
                0x0029 => unit_regen_interrupted_flat_mod: [u32; 7];      // [mana, rage, focus, energy, happiness, runes, runic_power]

                0x0030 => unit_level: u32;

                0x0031 => unit_faction_template: u32;
                0x0032 => unit_virtual_item_slot_id: [u32; 3];
                0x0035 => unit_flags: [bool; 2];

                0x0037 => unit_aura_state: u32;
                0x0038 => unit_base_attack_time_main_off_hand: [f32; 2];
                0x003A => unit_ranged_attack_time: f32;
                0x003B => unit_bounding_radius: f32;
                0x003C => unit_combat_reach: f32;

                0x003D => unit_display_id: u32;
                0x003E => unit_native_display_id: u32;
                0x003F => unit_mount_display_id: u32;

                0x0040 => unit_min_max_damage: [f32; 2];
                0x0042 => unit_min_max_offhand_damage: [f32; 2];

                0x0044 => unit_class_specific: ClassSpecific;

                0x0045 => unit_pet_number: u32;
                0x0046 => unit_pet_name_timestamp: u32;
                0x0047 => unit_pet_experience: u32;
                0x0048 => unit_pet_next_level_exp: u32;

                0x0049 => unit_dynamic_flags: [bool; 1];

                0x004A => unit_mod_cast_speed: f32;
                0x004B => unit_created_by_spell: u32;

                0x004C => unit_npc_flags: [bool; 1];
                0x004D => unit_npc_emote_state: u32;
                //
                0x004E => unit_stat: [u32; 5];                // [strength, agility, stamina, intellect, spirit]
                0x0053 => unit_stat_pos_effects:  [u32; 5];   // [strength, agility, stamina, intellect, spirit]
                0x0058 => unit_stat_neg_effects:  [u32; 5];   // [strength, agility, stamina, intellect, spirit]
                //
                0x005D => unit_resistance: [u32; 7];          // [normal, holy, fire, nature, frost, shadow, arcane]
                0x0064 => unit_resistance_pos: [u32; 7];      // [normal, holy, fire, nature, frost, shadow, arcane]
                0x006B => unit_resistance_neg: [u32; 7];      // [normal, holy, fire, nature, frost, shadow, arcane]
                //
                0x0072 => unit_base_mana: u32;
                0x0073 => unit_base_health: u32;
                //
                0x0074 => unit_bytes_2: u32;
                //
                0x0075 => unit_attack_power_melee: AttackPower;
                0x0078 => unit_attack_power_ranged: AttackPower;
                0x007B => unit_min_max_ranged_damage: [f32; 2];
                0x007D => unit_power_cost_mod: [u32; 7];      // [normal, holy, fire, nature, frost, shadow, arcane]
                0x0084 => unit_power_cost_mul: [u32; 7];      // [normal, holy, fire, nature, frost, shadow, arcane]
                0x008B => unit_max_health_modifier: u32;
                0x008C => unit_hover_height: u32;
                0x008D => unit_padding: ();
            }
        );
    };
}
