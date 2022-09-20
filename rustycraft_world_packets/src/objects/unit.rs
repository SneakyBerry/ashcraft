use deku::prelude::*;

use crate::class::Class;
use crate::gender::Gender;
use crate::guid::Guid;
use crate::objects::base::{BaseObject, Storage};
use crate::objects::player::PlayerFields;
use crate::objects::{InnerState, UpdateMaskObjectType};
use crate::power::Power;
use crate::race::Race;

#[derive(Debug, Default, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
pub struct Unit {
    #[deku(reader = "crate::objects::utils::read_object_btree_map(deku::rest)")]
    #[deku(writer = "crate::objects::utils::write_object_btree_map(deku::output, &self.inner)")]
    inner: InnerState,
}

impl Unit {
    pub fn new(guid: Guid) -> Box<Self> {
        let mut object = Self::default();
        <Self as BaseObject<0x0000>>::set_guid(&mut object, guid);
        <Self as BaseObject<0x0000>>::set_object_type(
            &mut object,
            UpdateMaskObjectType::Unit as u32,
        );
        Box::new(object)
    }
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
#[deku(endian = "little")]
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

impl<T> UnitFields for T where T: BaseObject<0x0006> + PlayerFields {}
pub trait UnitFields: BaseObject<0x0006> {
    fn set_unit_charm(&mut self, val: Guid) -> &mut Self {
        self.set_value(val, 0x0000)
    }
    fn get_unit_charm(&self) -> Option<Guid> {
        self.get_value(0x0000)
    }
    fn set_unit_summon(&mut self, val: Guid) -> &mut Self {
        self.set_value(val, 0x0002)
    }
    fn get_unit_summon(&self) -> Option<Guid> {
        self.get_value(0x0002)
    }
    fn set_unit_critter(&mut self, val: Guid) -> &mut Self {
        self.set_value(val, 0x0004)
    }
    fn get_unit_critter(&self) -> Option<Guid> {
        self.get_value(0x0004)
    }
    fn set_unit_charmed_by(&mut self, val: Guid) -> &mut Self {
        self.set_value(val, 0x0006)
    }
    fn get_unit_charmed_by(&self) -> Option<Guid> {
        self.get_value(0x0006)
    }
    fn set_unit_summoned_by(&mut self, val: Guid) -> &mut Self {
        self.set_value(val, 0x0008)
    }
    fn get_unit_summoned_by(&self) -> Option<Guid> {
        self.get_value(0x0008)
    }
    fn set_unit_created_by(&mut self, val: Guid) -> &mut Self {
        self.set_value(val, 0x000A)
    }
    fn get_unit_created_by(&self) -> Option<Guid> {
        self.get_value(0x000A)
    }
    fn set_unit_target(&mut self, val: Guid) -> &mut Self {
        self.set_value(val, 0x000C)
    }
    fn get_unit_target(&self) -> Option<Guid> {
        self.get_value(0x000C)
    }
    fn set_unit_channel_object(&mut self, val: Guid) -> &mut Self {
        self.set_value(val, 0x000E)
    }
    fn get_unit_channel_object(&self) -> Option<Guid> {
        self.get_value(0x000E)
    }
    fn set_unit_channel_spell(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0010)
    }
    fn get_unit_channel_spell(&self) -> Option<u32> {
        self.get_value(0x0010)
    }
    fn set_unit_unit_data(&mut self, val: UnitData) -> &mut Self {
        self.set_value(val, 0x0011)
    }
    fn get_unit_unit_data(&self) -> Option<UnitData> {
        self.get_value(0x0011)
    }
    fn set_unit_health(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0012)
    }
    fn get_unit_health(&self) -> Option<u32> {
        self.get_value(0x0012)
    }
    fn set_unit_power(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x0013 + index)
    }
    fn get_unit_power(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x0013 + index)
    }
    fn set_unit_max_health(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x001A)
    }
    fn get_unit_max_health(&self) -> Option<u32> {
        self.get_value(0x001A)
    }
    /// [mana, rage, focus, energy, happiness, runes, runic_power]
    fn set_unit_max_power(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x001B + index)
    }
    /// [mana, rage, focus, energy, happiness, runes, runic_power]
    fn get_unit_max_power(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x001B + index)
    }
    /// [mana, rage, focus, energy, happiness, runes, runic_power]
    fn set_unit_regen_flat_mod(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x0022 + index)
    }
    fn get_unit_regen_flat_mod(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x0022 + index)
    }
    /// [mana, rage, focus, energy, happiness, runes, runic_power]
    fn set_unit_regen_interrupted_flat_mod(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x0029 + index)
    }
    fn get_unit_regen_interrupted_flat_mod(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x0029 + index)
    }
    fn set_unit_level(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0030)
    }
    fn get_unit_level(&self) -> Option<u32> {
        self.get_value(0x0030)
    }
    fn set_unit_faction_template(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0031)
    }
    fn get_unit_faction_template(&self) -> Option<u32> {
        self.get_value(0x0031)
    }
    fn set_unit_virtual_item_slot_id(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 3, "Index is out of range");
        self.set_value(val, 0x0032 + index)
    }
    fn get_unit_virtual_item_slot_id(&self, index: usize) -> Option<u32> {
        assert!(index < 3, "Index is out of range");
        self.get_value(0x0032 + index)
    }
    fn set_unit_flags(&mut self, mask: u32, index: usize) -> &mut Self {
        assert!(index < 2, "Index is out of range");
        self.apply_and(0x0035 + index, mask)
    }
    fn unset_unit_flags(&mut self, mask: u32, index: usize) -> &mut Self {
        assert!(index < 2, "Index is out of range");
        self.apply_and(0x0035 + index, !(mask))
    }
    fn get_unit_flags(&mut self, index: usize) -> Option<u32> {
        assert!(index < 2, "Index is out of range");
        self.get_value(0x0035 + index)
    }

    fn set_unit_aura_state(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0037)
    }
    fn get_unit_aura_state(&self) -> Option<u32> {
        self.get_value(0x0037)
    }
    fn set_unit_base_attack_time_main_off_hand(&mut self, min: f32, max: f32) -> &mut Self {
        self.set_value((min, max), 0x0038)
    }
    fn get_unit_base_attack_time_main_off_hand(&self) -> Option<(f32, f32)> {
        self.get_value(0x0038)
    }
    fn set_unit_ranged_attack_time(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x003A)
    }
    fn get_unit_ranged_attack_time(&self) -> Option<f32> {
        self.get_value(0x003A)
    }
    fn set_unit_bounding_radius(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x003B)
    }
    fn get_unit_bounding_radius(&self) -> Option<f32> {
        self.get_value(0x003B)
    }
    fn set_unit_combat_reach(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x003C)
    }
    fn get_unit_combat_reach(&self) -> Option<f32> {
        self.get_value(0x003C)
    }
    fn set_unit_display_id(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x003D)
    }
    fn get_unit_display_id(&self) -> Option<u32> {
        self.get_value(0x003D)
    }
    fn set_unit_native_display_id(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x003E)
    }
    fn get_unit_native_display_id(&self) -> Option<u32> {
        self.get_value(0x003E)
    }
    fn set_unit_mount_display_id(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x003F)
    }
    fn get_unit_mount_display_id(&self) -> Option<u32> {
        self.get_value(0x003F)
    }
    fn set_unit_min_max_damage(&mut self, min: f32, max: f32) -> &mut Self {
        self.set_value((min, max), 0x0040)
    }
    fn get_unit_min_max_damage(&self) -> Option<(f32, f32)> {
        self.get_value(0x0040)
    }
    fn set_unit_min_max_offhand_damage(&mut self, min: f32, max: f32) -> &mut Self {
        self.set_value((min, max), 0x0042)
    }
    fn get_unit_min_max_offhand_damage(&self) -> Option<(f32, f32)> {
        self.get_value(0x0042)
    }
    fn set_unit_class_specific(&mut self, val: ClassSpecific) -> &mut Self {
        self.set_value(val, 0x0044)
    }
    fn get_unit_class_specific(&self) -> Option<ClassSpecific> {
        self.get_value(0x0044)
    }
    fn set_unit_pet_number(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0045)
    }
    fn get_unit_pet_number(&self) -> Option<u32> {
        self.get_value(0x0045)
    }
    fn set_unit_pet_name_timestamp(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0046)
    }
    fn get_unit_pet_name_timestamp(&self) -> Option<u32> {
        self.get_value(0x0046)
    }
    fn set_unit_pet_experience(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0047)
    }
    fn get_unit_pet_experience(&self) -> Option<u32> {
        self.get_value(0x0047)
    }
    fn set_unit_pet_next_level_exp(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0048)
    }
    fn get_unit_pet_next_level_exp(&self) -> Option<u32> {
        self.get_value(0x0048)
    }

    fn set_unit_dynamic_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x0049, mask)
    }
    fn unset_unit_dynamic_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x0049, !mask)
    }
    fn get_unit_dynamic_flags(&mut self) -> Option<u32> {
        self.get_value(0x0049)
    }

    fn set_unit_mod_cast_speed(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x004A)
    }
    fn get_unit_mod_cast_speed(&self) -> Option<f32> {
        self.get_value(0x004A)
    }
    fn set_unit_created_by_spell(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x004B)
    }
    fn get_unit_created_by_spell(&self) -> Option<u32> {
        self.get_value(0x004B)
    }

    fn set_unit_npc_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x004C, mask)
    }
    fn unset_unit_npc_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x004C, !mask)
    }
    fn get_unit_npc_flags(&mut self) -> Option<u32> {
        self.get_value(0x004C)
    }

    fn set_unit_npc_emote_state(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x004D)
    }
    fn get_unit_npc_emote_state(&self) -> Option<u32> {
        self.get_value(0x004D)
    }
    /// [strength, agility, stamina, intellect, spirit]
    fn set_unit_stat(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 5, "Index is out of range");
        self.set_value(val, 0x004E + index)
    }
    /// [strength, agility, stamina, intellect, spirit]
    fn get_unit_stat(&self, index: usize) -> Option<u32> {
        assert!(index < 5, "Index is out of range");
        self.get_value(0x004E + index)
    }
    /// [strength, agility, stamina, intellect, spirit]
    fn set_unit_stat_pos_effects(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 5, "Index is out of range");
        self.set_value(val, 0x0053 + index)
    }
    fn get_unit_stat_pos_effects(&self, index: usize) -> Option<u32> {
        assert!(index < 5, "Index is out of range");
        self.get_value(0x0053 + index)
    }
    /// [strength, agility, stamina, intellect, spirit]
    fn set_unit_stat_neg_effects(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 5, "Index is out of range");
        self.set_value(val, 0x0058 + index)
    }
    /// [strength, agility, stamina, intellect, spirit]
    fn get_unit_stat_neg_effects(&self, index: usize) -> Option<u32> {
        assert!(index < 5, "Index is out of range");
        self.get_value(0x0058 + index)
    }
    /// [normal, holy, fire, nature, frost, shadow, arcane]
    fn set_unit_resistance(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x005D + index)
    }
    /// [normal, holy, fire, nature, frost, shadow, arcane]
    fn get_unit_resistance(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x005D + index)
    }
    /// [normal, holy, fire, nature, frost, shadow, arcane]
    fn set_unit_resistance_pos(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x0064 + index)
    }
    /// [normal, holy, fire, nature, frost, shadow, arcane]
    fn get_unit_resistance_pos(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x0064 + index)
    }
    /// [normal, holy, fire, nature, frost, shadow, arcane]
    fn set_unit_resistance_neg(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x006B + index)
    }
    /// [normal, holy, fire, nature, frost, shadow, arcane]
    fn get_unit_resistance_neg(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x006B + index)
    }
    fn set_unit_base_mana(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0072)
    }
    fn get_unit_base_mana(&self) -> Option<u32> {
        self.get_value(0x0072)
    }
    fn set_unit_base_health(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0073)
    }
    fn get_unit_base_health(&self) -> Option<u32> {
        self.get_value(0x0073)
    }
    fn set_unit_bytes_2(&mut self, val: [u8; 4]) -> &mut Self {
        self.set_value(val, 0x0074)
    }
    fn get_unit_bytes_2(&self) -> Option<[u8; 4]> {
        self.get_value(0x0074)
    }
    fn set_unit_attack_power_melee(&mut self, val: AttackPower) -> &mut Self {
        self.set_value(val, 0x0075)
    }
    fn get_unit_attack_power_melee(&self) -> Option<AttackPower> {
        self.get_value(0x0075)
    }
    fn set_unit_attack_power_ranged(&mut self, val: AttackPower) -> &mut Self {
        self.set_value(val, 0x0078)
    }
    fn get_unit_attack_power_ranged(&self) -> Option<AttackPower> {
        self.get_value(0x0078)
    }
    fn set_unit_min_max_ranged_damage(&mut self, min: f32, max: f32) -> &mut Self {
        self.set_value((min, max), 0x007B)
    }
    fn get_unit_min_max_ranged_damage(&self) -> Option<(f32, f32)> {
        self.get_value(0x007B)
    }
    /// [normal, holy, fire, nature, frost, shadow, arcane]
    fn set_unit_power_cost_mod(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x007D + index)
    }
    /// [normal, holy, fire, nature, frost, shadow, arcane]
    fn get_unit_power_cost_mod(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x007D + index)
    }
    /// [normal, holy, fire, nature, frost, shadow, arcane]
    fn set_unit_power_cost_mul(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x0084 + index)
    }
    /// [normal, holy, fire, nature, frost, shadow, arcane]
    fn get_unit_power_cost_mul(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x0084 + index)
    }
    fn set_unit_max_health_modifier(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x008B)
    }
    fn get_unit_max_health_modifier(&self) -> Option<u32> {
        self.get_value(0x008B)
    }
    fn set_unit_hover_height(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x008C)
    }
    fn get_unit_hover_height(&self) -> Option<u32> {
        self.get_value(0x008C)
    }
}

impl Storage for Unit {
    fn get_inner(&self) -> &InnerState {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut InnerState {
        &mut self.inner
    }
}
impl UnitFields for Unit {}
