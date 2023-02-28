use deku::prelude::*;

pub mod client {
    use super::*;

    use crate::guid::Guid;

    #[derive(Debug, Clone, Eq, PartialEq, DekuRead)]
    pub struct NameQuery {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Eq, PartialEq, DekuRead)]
    pub struct QueryCreature {
        pub creature_id: u32,
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Eq, PartialEq, DekuRead)]
    pub struct QueryGameObject {
        pub object_id: u32,
        pub guid: Guid,
    }

    #[derive(Debug, Clone, Eq, PartialEq, DekuRead)]
    pub struct QueryItemSingle {
        pub item_id: u32,
    }
}

pub mod server {
    use rustycraft_derive::ServerPacket;

    use super::*;

    use crate::define_flags;
    use crate::prelude::{Class, DamageSchool, Gender, Opcode, PackedGuid, Race, SchoolIndexed};

    define_flags!(
        CreatureTypeFlags: u32 {
            CREATURE_TYPE_FLAG_TAMEABLE                             = 0x00000001,   // Makes the mob tameable (must also be a beast and have family set)
            CREATURE_TYPE_FLAG_VISIBLE_TO_GHOSTS                    = 0x00000002,   // Creature is also visible for not alive player. Allows gossip interaction if npcflag allows?
            CREATURE_TYPE_FLAG_BOSS_MOB                             = 0x00000004,   // Changes creature's visible level to "??" in the creature's portrait - Immune Knockback.
            CREATURE_TYPE_FLAG_DO_NOT_PLAY_WOUND_ANIM               = 0x00000008,   // Does not play wound animation on parry
            CREATURE_TYPE_FLAG_NO_FACTION_TOOLTIP                   = 0x00000010,
            CREATURE_TYPE_FLAG_MORE_AUDIBLE                         = 0x00000020,   // Sound related
            CREATURE_TYPE_FLAG_SPELL_ATTACKABLE                     = 0x00000040,
            CREATURE_TYPE_FLAG_INTERACT_WHILE_DEAD                  = 0x00000080,   // Player can interact with the creature if creature is dead (not if player is dead)
            CREATURE_TYPE_FLAG_SKIN_WITH_HERBALISM                  = 0x00000100,   // Can be looted by herbalist
            CREATURE_TYPE_FLAG_SKIN_WITH_MINING                     = 0x00000200,   // Can be looted by miner
            CREATURE_TYPE_FLAG_NO_DEATH_MESSAGE                     = 0x00000400,   // Death event will not show up in combat log
            CREATURE_TYPE_FLAG_ALLOW_MOUNTED_COMBAT                 = 0x00000800,   // Creature can remain mounted when entering combat
            CREATURE_TYPE_FLAG_CAN_ASSIST                           = 0x00001000,   // ? Can aid any player in combat if in range?
            CREATURE_TYPE_FLAG_NO_PET_BAR                           = 0x00002000,
            CREATURE_TYPE_FLAG_MASK_UID                             = 0x00004000,
            CREATURE_TYPE_FLAG_SKIN_WITH_ENGINEERING                = 0x00008000,   // Can be looted by engineer
            CREATURE_TYPE_FLAG_TAMEABLE_EXOTIC                      = 0x00010000,   // Can be tamed by hunter as exotic pet
            CREATURE_TYPE_FLAG_USE_MODEL_COLLISION_SIZE             = 0x00020000,   // Collision related. (always using default collision box?)
            CREATURE_TYPE_FLAG_ALLOW_INTERACTION_WHILE_IN_COMBAT    = 0x00040000,
            CREATURE_TYPE_FLAG_COLLIDE_WITH_MISSILES                = 0x00080000,   // Projectiles can collide with this creature - interacts with TARGET_DEST_TRAJ
            CREATURE_TYPE_FLAG_NO_NAME_PLATE                        = 0x00100000,
            CREATURE_TYPE_FLAG_DO_NOT_PLAY_MOUNTED_ANIMATIONS       = 0x00200000,
            CREATURE_TYPE_FLAG_LINK_ALL                             = 0x00400000,
            CREATURE_TYPE_FLAG_INTERACT_ONLY_WITH_CREATOR           = 0x00800000,
            CREATURE_TYPE_FLAG_DO_NOT_PLAY_UNIT_EVENT_SOUNDS        = 0x01000000,
            CREATURE_TYPE_FLAG_HAS_NO_SHADOW_BLOB                   = 0x02000000,
            CREATURE_TYPE_FLAG_TREAT_AS_RAID_UNIT                   = 0x04000000,   // ! Creature can be targeted by spells that require target to be in caster's party/raid
            CREATURE_TYPE_FLAG_FORCE_GOSSIP                         = 0x08000000,   // Allows the creature to display a single gossip option.
            CREATURE_TYPE_FLAG_DO_NOT_SHEATHE                       = 0x10000000,
            CREATURE_TYPE_FLAG_DO_NOT_TARGET_ON_INTERACTION         = 0x20000000,
            CREATURE_TYPE_FLAG_DO_NOT_RENDER_OBJECT_NAME            = 0x40000000,
            CREATURE_TYPE_FLAG_QUEST_BOSS                           = 0x80000000    // Not verified
        }
    );

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum CreatureFamily {
        None = 0,
        Wolf = 1,
        Cat = 2,
        Spider = 3,
        Bear = 4,
        Boar = 5,
        Crocolisk = 6,
        CarrionBird = 7,
        Crab = 8,
        Gorilla = 9,
        HorseCustom = 10, // Does not exist in DBC but used for horse like beasts in DB
        Raptor = 11,
        Tallstrider = 12,
        Felhunter = 15,
        Voidwalker = 16,
        Succubus = 17,
        Doomguard = 19,
        Scorpid = 20,
        Turtle = 21,
        Imp = 23,
        Bat = 24,
        Hyena = 25,
        BirdOfPrey = 26,
        WindSerpent = 27,
        RemoteControl = 28,
        Felguard = 29,
        Dragonhawk = 30,
        Ravager = 31,
        WarpStalker = 32,
        Sporebat = 33,
        NetherRay = 34,
        Serpent = 35,
        Moth = 37,
        Chimaera = 38,
        Devilsaur = 39,
        Ghoul = 40,
        Silithid = 41,
        Worm = 42,
        Rhino = 43,
        Wasp = 44,
        CoreHound = 45,
        SpiritBeast = 46,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum CreatureType {
        Beast = 1,
        Dragonkin = 2,
        Demon = 3,
        Elemental = 4,
        Giant = 5,
        Undead = 6,
        Humanoid = 7,
        Critter = 8,
        Mechanical = 9,
        NotSpecified = 10,
        Totem = 11,
        NonCombatPet = 12,
        GasCloud = 13,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum CreatureClass {
        Normal = 0,
        Elite = 1,
        RareElite = 2,
        WorldBoss = 3,
        Rare = 4,
        Trivial = 5,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct CreatureStats {
        pub id: u32,
        #[deku(pad_bytes_after = "3")]
        #[deku(writer = "crate::write_c_string(deku::output, &self.name)")]
        pub name: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.alt_name)")]
        pub alt_name: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.cursor_name)")]
        pub cursor_name: String,
        pub flags: CreatureTypeFlags,
        pub creature_type: CreatureType,
        pub creature_family: CreatureFamily,
        pub classification: CreatureClass,
        pub kill_credit: [u32; 2],
        pub creature_display_id: [u32; 4],
        pub hp_multiplier: f32,
        pub energy_multiplier: f32,
        pub is_leader: bool,
        pub quest_items: [u32; 6],
        pub movement_info_id: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgCreatureQueryResponse)]
    pub struct CreatureQueryResponse {
        #[deku(
            update = "if self.stats.is_none() { self.creature_id | 0x80000000 } else { self.creature_id }"
        )]
        pub creature_id: u32,
        #[deku(
            update = "if let Some(mut s) = self.stats.take() { s.update()?; Some(s) } else { None }"
        )]
        pub stats: Option<CreatureStats>,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct GameObjectStats {
        pub object_type: u32,
        pub display_id: u32,
        #[deku(pad_bytes_after = "3")]
        #[deku(writer = "crate::write_c_string(deku::output, &self.name)")]
        pub name: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.icon_name)")]
        pub icon_name: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.cast_bar_caption)")]
        pub cast_bar_caption: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.unk)")]
        pub unk: String,
        pub data: [u32; 24],
        pub size: f32,
        pub quest_items: [u32; 6],
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgGameObjectQueryResponse)]
    pub struct GameobjectQueryResponse {
        #[deku(
            update = "if self.stats.is_none() { self.game_object_id | 0x80000000 } else { self.game_object_id }"
        )]
        pub game_object_id: u32,
        #[deku(
            update = "if let Some(mut s) = self.stats.take() { s.update()?; Some(s) } else { None }"
        )]
        pub stats: Option<GameObjectStats>,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct KnownName {
        #[deku(writer = "crate::write_c_string(deku::output, &self.name)")]
        pub name: String,
        pub realm_name: u8,
        pub race: Race,
        pub sex: Gender,
        pub class: Class,
        pub declined: bool,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u8")]
    pub enum NameQueryData {
        #[deku(id = "0x01")]
        Unknown,
        #[deku(id = "0x00")]
        Known(KnownName),
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgNameQueryResponse)]
    pub struct NameQueryResponse {
        pub guid: PackedGuid,
        pub response: NameQueryData,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgItemQuerySingleResponse)]
    pub struct ItemQuerySingleResponse {
        #[deku(
            update = "if self.response.is_none() { self.item_id | 0x80000000 } else { self.item_id }"
        )]
        pub item_id: u32,
        #[deku(
            update = "if let Some(mut r) = self.response.take() { r.update()?; Some(r) } else { None }"
        )]
        pub response: Option<ItemStats>,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Consumable {
        Consumable = 0,
        Potion = 1,
        Elixir = 2,
        Flask = 3,
        Scroll = 4,
        Food = 5,
        ItemEnhancement = 6,
        Bandage = 7,
        ConsumableOther = 8,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Container {
        Container = 0,
        SoulContainer = 1,
        HerbContainer = 2,
        EnchantingContainer = 3,
        EngineeringContainer = 4,
        GemContainer = 5,
        MiningContainer = 6,
        LeatherworkingContainer = 7,
        InscriptionContainer = 8,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Weapon {
        Axe = 0,
        Axe2 = 1,
        Bow = 2,
        Gun = 3,
        Mace = 4,
        Mace2 = 5,
        Polearm = 6,
        Sword = 7,
        Sword2 = 8,
        Obsolete = 9,
        Staff = 10,
        Exotic = 11,
        Exotic2 = 12,
        Fist = 13,
        Misc = 14,
        Dagger = 15,
        Thrown = 16,
        Spear = 17,
        Crossbow = 18,
        Wand = 19,
        FishingPole = 20,
    }

    // #define ITEM_SUBCLASS_MASK_WEAPON_RANGED (\
    // (1 << ITEM_SUBCLASS_WEAPON_BOW) | (1 << ITEM_SUBCLASS_WEAPON_GUN) |\
    // (1 << ITEM_SUBCLASS_WEAPON_CROSSBOW) | (1 << ITEM_SUBCLASS_WEAPON_THROWN))

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Gem {
        Red = 0,
        Blue = 1,
        Yellow = 2,
        Purple = 3,
        Green = 4,
        Orange = 5,
        Meta = 6,
        Simple = 7,
        Prismatic = 8,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Armor {
        Misc = 0,
        Cloth = 1,
        Leather = 2,
        Mail = 3,
        Plate = 4,
        Buckler = 5,
        Shield = 6,
        Libram = 7,
        Idol = 8,
        Totem = 9,
        Sigil = 10,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Reagent {
        Reagent = 0,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Projectile {
        Wand = 0, // ABS
        Bolt = 1, // ABS
        Arrow = 2,
        Bullet = 3,
        Thrown = 4, // ABS
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum TradeGoods {
        TradeGoods = 0,
        Parts = 1,
        Explosives = 2,
        Devices = 3,
        JewelCrafting = 4,
        Cloth = 5,
        Leather = 6,
        MetalStone = 7,
        Meat = 8,
        Herb = 9,
        Elemental = 10,
        TradeGoodsOther = 11,
        Enchanting = 12,
        Material = 13,
        ArmorEnchantment = 14,
        WeaponEnchantment = 15,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Generic {
        Generic = 0,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Recipe {
        Book = 0,
        LeatherworkPattern = 1,
        TailoringPattern = 2,
        EngineeringSchematic = 3,
        Blacksmith = 4,
        CookingRecipe = 5,
        AlchemyRecipe = 6,
        FirstAidManual = 7,
        EnchantingFormula = 8,
        FishingManual = 9,
        JewelCraftingRecipe = 10,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Money {
        Money = 0,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Quiver {
        Quiver0 = 0, // ABS
        Quiver1 = 1, // ABS
        Quiver = 2,
        AmmoPouch = 3,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Quest {
        Quest = 0,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Misc {
        Misc = 0,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Key {
        Key = 0,
        Lockpick = 1,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Permanent {
        Permanent = 0,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Junk {
        Junk = 0,
        JunkReagent = 1,
        JunkPet = 2,
        JunkHoliday = 3,
        JunkOther = 4,
        JunkMount = 5,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Glyph {
        GlyphWarrior = 1,
        GlyphPaladin = 2,
        GlyphHunter = 3,
        GlyphRogue = 4,
        GlyphPriest = 5,
        GlyphDeathKnight = 6,
        GlyphShaman = 7,
        GlyphMage = 8,
        GlyphWarlock = 9,
        GlyphDruid = 11,
    }

    /// ID from ItemClass.dbc
    /// All nested enums is id from ItemSubClass.dbc
    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum ItemClass {
        #[deku(id = "0")]
        Consumable(Consumable),
        #[deku(id = "1")]
        Container(Container),
        #[deku(id = "2")]
        Weapon(Weapon),
        #[deku(id = "3")]
        Gem(Gem),
        #[deku(id = "4")]
        Armor(Armor),
        #[deku(id = "5")]
        Reagent(Reagent),
        #[deku(id = "6")]
        Projectile(Projectile),
        #[deku(id = "7")]
        TradeGoods(TradeGoods),
        #[deku(id = "8")]
        Generic(Generic),
        #[deku(id = "9")]
        Recipe(Recipe),
        #[deku(id = "10")]
        Money(Money),
        #[deku(id = "11")]
        Quiver(Quiver),
        #[deku(id = "12")]
        Quest(Quest),
        #[deku(id = "13")]
        Key(Key),
        #[deku(id = "14")]
        Permanent(Permanent),
        #[deku(id = "15")]
        Misc(Misc),
        #[deku(id = "16")]
        Glyph(Glyph),
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum InventoryTypeU32 {
        NonEquip = 0,
        Head = 1,
        Neck = 2,
        Shoulders = 3,
        Body = 4,
        Chest = 5,
        Waist = 6,
        Legs = 7,
        Feet = 8,
        Wrists = 9,
        Hands = 10,
        Finger = 11,
        Trinket = 12,
        Weapon = 13,
        Shield = 14,
        Ranged = 15,
        Cloak = 16,
        WeaponTwoHands = 17,
        Bag = 18,
        Tabard = 19,
        Robe = 20,
        WeaponMainHand = 21,
        WeaponOffHand = 22,
        Holdable = 23,
        Ammo = 24,
        Thrown = 25,
        RangedRight = 26,
        Quiver = 27,
        Relic = 28,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum ItemBondingType {
        NoBind = 0,
        WhenPickedUp = 1,
        WhenEquiped = 2,
        WhenUse = 3,
        QuestItem = 4,
        QuestItem1 = 5, // not used in game
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    #[repr(u32)]
    pub enum ItemMod {
        #[deku(id = "0")]
        Mana(i32),
        #[deku(id = "1")]
        Health(i32),
        #[deku(id = "3")]
        Agility(i32),
        #[deku(id = "4")]
        Strength(i32),
        #[deku(id = "5")]
        Intellect(i32),
        #[deku(id = "6")]
        Spirit(i32),
        #[deku(id = "7")]
        Stamina(i32),
        #[deku(id = "12")]
        DefenseSkillRating(i32),
        #[deku(id = "13")]
        DodgeRating(i32),
        #[deku(id = "14")]
        ParryRating(i32),
        #[deku(id = "15")]
        BlockRating(i32),
        #[deku(id = "16")]
        HitMeleeRating(i32),
        #[deku(id = "17")]
        HitRangedRating(i32),
        #[deku(id = "18")]
        HitSpellRating(i32),
        #[deku(id = "19")]
        CritMeleeRating(i32),
        #[deku(id = "20")]
        CritRangedRating(i32),
        #[deku(id = "21")]
        CritSpellRating(i32),
        #[deku(id = "22")]
        HitTakenMeleeRating(i32),
        #[deku(id = "23")]
        HitTakenRangedRating(i32),
        #[deku(id = "24")]
        HitTakenSpellRating(i32),
        #[deku(id = "25")]
        CritTakenMeleeRating(i32),
        #[deku(id = "26")]
        CritTakenRangedRating(i32),
        #[deku(id = "27")]
        CritTakenSpellRating(i32),
        #[deku(id = "28")]
        HasteMeleeRating(i32),
        #[deku(id = "29")]
        HasteRangedRating(i32),
        #[deku(id = "30")]
        HasteSpellRating(i32),
        #[deku(id = "31")]
        HitRating(i32),
        #[deku(id = "32")]
        CritRating(i32),
        #[deku(id = "33")]
        HitTakenRating(i32),
        #[deku(id = "34")]
        CritTakenRating(i32),
        #[deku(id = "35")]
        ResilienceRating(i32),
        #[deku(id = "36")]
        HasteRating(i32),
        #[deku(id = "37")]
        ExpertiseRating(i32),
        #[deku(id = "38")]
        AttackPower(i32),
        #[deku(id = "39")]
        RangedAttackPower(i32),
        #[deku(id = "40")]
        FeralAttackPower(i32),
        #[deku(id = "41")]
        SpellHealingDone(i32), // deprecated
        #[deku(id = "42")]
        SpellDamageDone(i32), // deprecated
        #[deku(id = "43")]
        ManaRegeneration(i32),
        #[deku(id = "44")]
        ArmorPenetrationRating(i32),
        #[deku(id = "45")]
        SpellPower(i32),
        #[deku(id = "46")]
        HealthRegen(i32),
        #[deku(id = "47")]
        SpellPenetration(i32),
        #[deku(id = "48")]
        BlockValue(i32),
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum ItemQuality {
        Poor = 0,
        Common = 1,
        Uncommon = 2,
        Rare = 3,
        Epic = 4,
        Legendary = 5,
        Artifact = 6,
        Heirloom = 7,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum ReputationRank {
        Hated = 0,
        Hostile = 1,
        Unfriendly = 2,
        Neutral = 3,
        Friendly = 4,
        Honored = 5,
        Revered = 6,
        Exalted = 7,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum Sheath {
        None = 0,
        MainHand = 1,
        OffHand = 2,
        LargeWeapon = 3,
        TwoHand = 4,
        Bow = 5,
        Crossbow = 6,
        Gun = 7,
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone)]
    pub struct ItemStats {
        pub class: ItemClass,
        /// id from ItemSubClass.dbc to override weapon sound
        pub sound_override_subclass: i32,
        #[deku(writer = "crate::write_c_string(deku::output, &self.name)")]
        #[deku(pad_bytes_after = "3")]
        pub name: String,
        /// id from ItemDisplayInfo.dbc
        pub display_info_id: u32,
        pub quality: ItemQuality,
        pub flags: ItemFlags,
        pub flags2: ItemFlags2,
        pub buy_price: i32,
        pub sell_price: u32,
        pub inventory_type: InventoryTypeU32,
        pub allowable_class: u32, // ClassMask
        pub allowable_race: u32,  // RaceMask
        pub item_level: u32,
        pub required_level: u32,
        /// Id from SkillLine.dbc
        pub required_skill: u32,
        pub required_skill_rank: u32,
        /// Id from Spell.dbc
        pub required_spell: u32,
        pub required_honor_rank: u32,
        pub required_city_rank: u32,
        /// Id from Faction.dbc
        pub required_reputation_faction: u32,
        pub required_reputation_rank: ReputationRank,
        /// <= 0: item not limited, > 0: item limited count
        pub max_count: i32,
        /// 0: not allowed, -1: put in player coin info tab and don't limit stacking (so 1 slot)
        pub stackable: i32,
        pub container_slots: u32,
        #[deku(temp, temp_value = "item_stat.len() as u32")]
        pub stats_count: u32,
        pub item_stat: Vec<ItemMod>,
        /// Id from ScalingStatDistribution.dbc
        pub scaling_stat_distribution: u32,
        /// Mask for selecting columns in ScalingStatValues.dbc
        pub scaling_stat_value: u32,
        pub damage: [ItemDamageData; 2],
        pub resistance: SchoolIndexed<u32>,
        pub delay: u32,
        pub ammo_type: u32,
        pub ranged_mod_range: f32,
        pub spells: [ItemSpellData; 5],
        pub bonding: ItemBondingType,
        #[deku(writer = "crate::write_c_string(deku::output, &self.description)")]
        pub description: String,
        pub page_text: u32,
        pub language_id: u32,   // ENUM
        pub page_material: u32, // Also ENUM ?
        /// Id from QuestCache.wdb
        pub start_quest: u32,
        pub lock_id: u32, // LockType.dbc
        /// Id from Material.dbc
        pub material: i32,
        pub sheath: Sheath,
        /// Id from RandomProperties.dbc
        pub random_property: u32,
        /// Id from RandomSuffix.dbc
        pub random_suffix: u32,
        pub block: u32,
        /// Id from ItemSet.dbc
        pub item_set: u32,
        pub max_durability: u32,
        /// Id from AreaTable.dbc
        pub area: u32,
        /// Id from Map.dbc
        pub map: u32,
        /// Bitmask (1 << id from ItemBagFamily.dbc)
        pub bag_family: u32,
        /// Id from TotemCategory.dbc
        pub totem_category: u32,
        pub socket: [ItemSocketData; 3],
        /// Id from SpellItemEnchantment.dbc
        pub socket_bonus: u32,
        /// Id from GemProperties.dbc
        pub gem_properties: u32,
        pub required_disenchant_skill: u32,
        pub armor_damage_modifier: f32,
        pub duration: u32,
        /// Id from ItemLimitCategory.dbc
        pub item_limit_category: u32,
        /// Id from Holidays.dbc
        pub holiday_id: u32,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct ItemDamageData {
        pub min: f32,
        pub max: f32,
        pub damage_type: DamageSchool,
    }

    #[derive(Debug, Clone, Copy, DekuWrite)]
    pub struct ItemSpellData {
        /// Id from Spell.dbc
        pub spell_id: i32,
        pub spell_trigger: u32,
        pub spell_charges: i32,
        pub spell_cooldown: i32,
        /// Id from SpellCategory.dbc
        pub spell_category: u32,
        pub spell_category_cooldown: i32,
    }

    #[derive(Debug, Clone, Copy, DekuWrite)]
    #[deku(type = "u32")]
    pub enum SocketColor {
        Empty = 0,
        SocketColorMeta = 1,
        SocketColorRed = 2,
        SocketColorYellow = 4,
        SocketColorBlue = 8,
        Unknown = 14,
    }

    #[derive(Debug, Clone, Copy, DekuWrite)]
    pub struct ItemSocketData {
        pub color: SocketColor,
        pub content: u32,
    }

    define_flags!(
        ItemFieldFlags: u32 {
            ITEM_FIELD_FLAG_SOULBOUND = 0x00000001, // Item is soulbound and cannot be traded <<--
            ITEM_FIELD_FLAG_UNK1 = 0x00000002,      // ?
            ITEM_FIELD_FLAG_UNLOCKED = 0x00000004,  // Item had lock but can be opened now
            ITEM_FIELD_FLAG_WRAPPED = 0x00000008,   // Item is wrapped and contains another item
            ITEM_FIELD_FLAG_UNK2 = 0x00000010,      // ?
            ITEM_FIELD_FLAG_UNK3 = 0x00000020,      // ?
            ITEM_FIELD_FLAG_UNK4 = 0x00000040,      // ?
            ITEM_FIELD_FLAG_UNK5 = 0x00000080,      // ?
            ITEM_FIELD_FLAG_BOP_TRADEABLE = 0x00000100, // Allows trading soulbound items
            ITEM_FIELD_FLAG_READABLE = 0x00000200,  // Opens text page when right clicked
            ITEM_FIELD_FLAG_UNK6 = 0x00000400,      // ?
            ITEM_FIELD_FLAG_UNK7 = 0x00000800,      // ?
            ITEM_FIELD_FLAG_REFUNDABLE = 0x00001000, // Item can be returned to vendor for its original cost (extended cost)
            ITEM_FIELD_FLAG_UNK8 = 0x00002000,       // ?
            ITEM_FIELD_FLAG_UNK9 = 0x00004000,       // ?
            ITEM_FIELD_FLAG_UNK10 = 0x00008000,      // ?
            ITEM_FIELD_FLAG_UNK11 = 0x00010000,      // ?
            ITEM_FIELD_FLAG_UNK12 = 0x00020000,      // ?
            ITEM_FIELD_FLAG_UNK13 = 0x00040000,      // ?
            ITEM_FIELD_FLAG_UNK14 = 0x00080000,      // ?
            ITEM_FIELD_FLAG_UNK15 = 0x00100000,      // ?
            ITEM_FIELD_FLAG_UNK16 = 0x00200000,      // ?
            ITEM_FIELD_FLAG_UNK17 = 0x00400000,      // ?
            ITEM_FIELD_FLAG_UNK18 = 0x00800000,      // ?
            ITEM_FIELD_FLAG_UNK19 = 0x01000000,      // ?
            ITEM_FIELD_FLAG_UNK20 = 0x02000000,      // ?
            ITEM_FIELD_FLAG_UNK21 = 0x04000000,      // ?
            ITEM_FIELD_FLAG_UNK22 = 0x08000000,      // ?
            ITEM_FIELD_FLAG_UNK23 = 0x10000000,      // ?
            ITEM_FIELD_FLAG_UNK24 = 0x20000000,      // ?
            ITEM_FIELD_FLAG_UNK25 = 0x40000000,      // ?
            ITEM_FIELD_FLAG_UNK26 = 0x80000000,      // ?

            ITEM_FLAG_MAIL_TEXT_MASK = Self::ITEM_FIELD_FLAG_READABLE as u32
                | Self::ITEM_FIELD_FLAG_UNK13 as u32
                | Self::ITEM_FIELD_FLAG_UNK14 as u32
        }
    );

    define_flags!(
        ItemFlags: u32 {
            ITEM_FLAG_NO_PICKUP = 0x00000001,
            ITEM_FLAG_CONJURED = 0x00000002,          // Conjured item
            ITEM_FLAG_HAS_LOOT = 0x00000004,          // Item can be right clicked to open for loot
            ITEM_FLAG_HEROIC_TOOLTIP = 0x00000008,    // Makes green "Heroic" text appear on item
            ITEM_FLAG_DEPRECATED = 0x00000010,        // Cannot equip or use
            ITEM_FLAG_NO_USER_DESTROY = 0x00000020, // Item can not be destroyed, except by using spell (item can be reagent for spell)
            ITEM_FLAG_PLAYERCAST = 0x00000040,      // Item's spells are castable by players
            ITEM_FLAG_NO_EQUIP_COOLDOWN = 0x00000080, // No default 30 seconds cooldown when equipped
            ITEM_FLAG_MULTI_LOOT_QUEST = 0x00000100,
            ITEM_FLAG_IS_WRAPPER = 0x00000200, // Item can wrap other items
            ITEM_FLAG_USES_RESOURCES = 0x00000400,
            ITEM_FLAG_MULTI_DROP = 0x00000800, // Looting this item does not remove it from available loot
            ITEM_FLAG_ITEM_PURCHASE_RECORD = 0x00001000, // Item can be returned to vendor for its original cost (extended cost)
            ITEM_FLAG_PETITION = 0x00002000,             // Item is guild or arena charter
            ITEM_FLAG_HAS_TEXT = 0x00004000,             // Only readable items have this (but not all)
            ITEM_FLAG_NO_DISENCHANT = 0x00008000,
            ITEM_FLAG_REAL_DURATION = 0x00010000,
            ITEM_FLAG_NO_CREATOR = 0x00020000,
            ITEM_FLAG_IS_PROSPECTABLE = 0x00040000, // Item can be prospected
            ITEM_FLAG_UNIQUE_EQUIPPABLE = 0x00080000, // You can only equip one of these
            ITEM_FLAG_IGNORE_FOR_AURAS = 0x00100000,
            ITEM_FLAG_IGNORE_DEFAULT_ARENA_RESTRICTIONS = 0x00200000, // Item can be used during arena match
            ITEM_FLAG_NO_DURABILITY_LOSS = 0x00400000, // Some Thrown weapons have it (and only Thrown) but not all
            ITEM_FLAG_USE_WHEN_SHAPESHIFTED = 0x00800000, // Item can be used in shapeshift forms
            ITEM_FLAG_HAS_QUEST_GLOW = 0x01000000,
            ITEM_FLAG_HIDE_UNUSABLE_RECIPE = 0x02000000, // Profession recipes: can only be looted if you meet requirements and don't already know it
            ITEM_FLAG_NOT_USEABLE_IN_ARENA = 0x04000000, // Item cannot be used in arena
            ITEM_FLAG_IS_BOUND_TO_ACCOUNT = 0x08000000, // Item binds to account and can be sent only to your own characters
            ITEM_FLAG_NO_REAGENT_COST = 0x10000000,     // Spell is cast ignoring reagents
            ITEM_FLAG_IS_MILLABLE = 0x20000000,         // Item can be milled
            ITEM_FLAG_REPORT_TO_GUILD_CHAT = 0x40000000,
            ITEM_FLAG_NO_PROGRESSIVE_LOOT = 0x80000000
        }
    );

    define_flags!(
        ItemFlags2: u32 {
            ITEM_FLAG2_FACTION_HORDE = 0x00000001,
            ITEM_FLAG2_FACTION_ALLIANCE = 0x00000002,
            ITEM_FLAG2_DONT_IGNORE_BUY_PRICE = 0x00000004, // when item uses extended cost, gold is also required
            ITEM_FLAG2_CLASSIFY_AS_CASTER = 0x00000008,
            ITEM_FLAG2_CLASSIFY_AS_PHYSICAL = 0x00000010,
            ITEM_FLAG2_EVERYONE_CAN_ROLL_NEED = 0x00000020,
            ITEM_FLAG2_NO_TRADE_BIND_ON_ACQUIRE = 0x00000040,
            ITEM_FLAG2_CAN_TRADE_BIND_ON_ACQUIRE = 0x00000080,
            ITEM_FLAG2_CAN_ONLY_ROLL_GREED = 0x00000100,
            ITEM_FLAG2_CASTER_WEAPON = 0x00000200,
            ITEM_FLAG2_DELETE_ON_LOGIN = 0x00000400,
            ITEM_FLAG2_INTERNAL_ITEM = 0x00000800,
            ITEM_FLAG2_NO_VENDOR_VALUE = 0x00001000,
            ITEM_FLAG2_SHOW_BEFORE_DISCOVERED = 0x00002000,
            ITEM_FLAG2_OVERRIDE_GOLD_COST = 0x00004000,
            ITEM_FLAG2_IGNORE_DEFAULT_RATED_BG_RESTRICTIONS = 0x00008000,
            ITEM_FLAG2_NOT_USABLE_IN_RATED_BG = 0x00010000,
            ITEM_FLAG2_BNET_ACCOUNT_TRADE_OK = 0x00020000,
            ITEM_FLAG2_CONFIRM_BEFORE_USE = 0x00040000,
            ITEM_FLAG2_REEVALUATE_BONDING_ON_TRANSFORM = 0x00080000,
            ITEM_FLAG2_NO_TRANSFORM_ON_CHARGE_DEPLETION = 0x00100000,
            ITEM_FLAG2_NO_ALTER_ITEM_VISUAL = 0x00200000,
            ITEM_FLAG2_NO_SOURCE_FOR_ITEM_VISUAL = 0x00400000,
            ITEM_FLAG2_IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE = 0x00800000,
            ITEM_FLAG2_NO_DURABILITY = 0x01000000,
            ITEM_FLAG2_ROLE_TANK = 0x02000000,
            ITEM_FLAG2_ROLE_HEALER = 0x04000000,
            ITEM_FLAG2_ROLE_DAMAGE = 0x08000000,
            ITEM_FLAG2_CAN_DROP_IN_CHALLENGE_MODE = 0x10000000,
            ITEM_FLAG2_NEVER_STACK_IN_LOOT_UI = 0x20000000,
            ITEM_FLAG2_DISENCHANT_TO_LOOT_TABLE = 0x40000000,
            ITEM_FLAG2_USED_IN_A_TRADESKILL = 0x80000000
        }
    );

    define_flags!(
        ItemFlagsCustom: u32 {
            ITEM_FLAGS_CU_DURATION_REAL_TIME = 0x0001, // Item duration will tick even if player is offline
            ITEM_FLAGS_CU_IGNORE_QUEST_STATUS = 0x0002, // No quest status will be checked when this item drops
            ITEM_FLAGS_CU_FOLLOW_LOOT_RULES = 0x0004 // Item will always follow group/master/need before greed looting rules
        }
    );
}
