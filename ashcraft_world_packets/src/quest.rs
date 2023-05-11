use deku::prelude::*;

pub mod client {
    use super::*;

    #[derive(Debug, PartialEq, DekuRead)]
    pub struct QueryQuestInfo {
        pub quest_id: u32,
    }
}

pub mod server {
    use super::*;
    use crate::define_flags;
    use crate::prelude::{Guid, Opcode};
    use deku::bitvec::{BitVec, Msb0};
    use ashcraft_derive::ServerPacket;

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum QuestType {
        Elite = 1,
        Life = 21,
        Pvp = 41,
        Raid = 62,
        Dungeon = 81,
        WorldEvent = 82,
        Legendary = 83,
        Escort = 84,
        Heroic = 85,
        Raid10 = 88,
        Raid25 = 89,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "i32")]
    pub enum QuestSort {
        Epic = 1,
        WailingCavernsOld = 21,
        Seasonal = 22,
        UndercityOld = 23,
        Herbalism = 24,
        Battlegrounds = 25,
        UldamnOld = 41,
        Warlock = 61,
        Warrior = 81,
        Shaman = 82,
        Fishing = 101,
        Blacksmithing = 121,
        Paladin = 141,
        Mage = 161,
        Rogue = 162,
        Alchemy = 181,
        Leatherworking = 182,
        Engineering = 201,
        TreasureMap = 221,
        SunkenTempleOld = 241,
        Hunter = 261,
        Priest = 262,
        Druid = 263,
        Tailoring = 264,
        Special = 284,
        Cooking = 304,
        FirstAid = 324,
        Legendary = 344,
        DarkmoonFaire = 364,
        AhnQirajWar = 365,
        LunarFestival = 366,
        Reputation = 367,
        Invasion = 368,
        Midsummer = 369,
        Brewfest = 370,
        Inscription = 371,
        DeathKnight = 372,
        Jewelcrafting = 373,
        Noblegarden = 374,
        PilgrimsBounty = 375,
        LoveIsInTheAir = 376,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum QuestMethod {
        AutoComplete = 0,
        Standard = 1,
        Tame = 2,
    }

    define_flags!(
        QuestFlags: u32 {
            // Flags used at server and sent to client
            QUEST_FLAGS_NONE                    = 0x00000000,
            QUEST_FLAGS_STAY_ALIVE              = 0x00000001,   // Not used currently
            QUEST_FLAGS_PARTY_ACCEPT            = 0x00000002,   // Not used currently. If player in party, all players that can accept this quest will receive confirmation box to accept quest CMSG_QUEST_CONFIRM_ACCEPT/SMSG_QUEST_CONFIRM_ACCEPT
            QUEST_FLAGS_EXPLORATION             = 0x00000004,   // Not used currently
            QUEST_FLAGS_SHARABLE                = 0x00000008,   // Can be shared: Player::CanShareQuest()
            QUEST_FLAGS_HAS_CONDITION           = 0x00000010,   // Not used currently
            QUEST_FLAGS_HIDE_REWARD_POI         = 0x00000020,   // Not used currently: Unsure of content
            QUEST_FLAGS_RAID                    = 0x00000040,   // Can be completed while in raid
            QUEST_FLAGS_TBC                     = 0x00000080,   // Not used currently: Available if TBC expansion enabled only
            QUEST_FLAGS_NO_MONEY_FROM_XP        = 0x00000100,   // Not used currently: Experience is not converted to gold at max level
            QUEST_FLAGS_HIDDEN_REWARDS          = 0x00000200,   // Items and money rewarded only sent in SMSG_QUESTGIVER_OFFER_REWARD (not in SMSG_QUEST_GIVER_QUEST_DETAILS or in client quest log(SMSG_QUEST_QUERY_RESPONSE))
            QUEST_FLAGS_TRACKING                = 0x00000400,   // These quests are automatically rewarded on quest complete and they will never appear in quest log client side.
            QUEST_FLAGS_DEPRECATE_REPUTATION    = 0x00000800,   // Not used currently
            QUEST_FLAGS_DAILY                   = 0x00001000,   // Used to know quest is Daily one
            QUEST_FLAGS_FLAGS_PVP               = 0x00002000,   // Having this quest in log forces PvP flag
            QUEST_FLAGS_UNAVAILABLE             = 0x00004000,   // Used on quests that are not generically available
            QUEST_FLAGS_WEEKLY                  = 0x00008000,
            QUEST_FLAGS_AUTOCOMPLETE            = 0x00010000,   // auto complete
            QUEST_FLAGS_DISPLAY_ITEM_IN_TRACKER = 0x00020000,   // Displays usable item in quest tracker
            QUEST_FLAGS_OBJ_TEXT                = 0x00040000,   // use Objective text as Complete text
            QUEST_FLAGS_AUTO_ACCEPT             = 0x00080000   // The client recognizes this flag as auto-accept. However, NONE of the current quests (3.3.5a) have this flag. Maybe blizz used to use it, or will use it in the future.

            // ... 4.x added flags up to 0x80000000 - all unknown for now
        }
    );

    #[derive(Debug, Clone, Copy, Default, DekuWrite)]
    pub struct QuestInfoChoiceItem {
        item_id: u32,
        quantity: u32,
    }

    #[derive(Debug, Clone)]
    pub enum Objective {
        Npc(u32),
        GameObject(u32),
    }

    impl DekuWrite for Objective {
        fn write(&self, output: &mut BitVec<u8, Msb0>, ctx: ()) -> Result<(), DekuError> {
            match self {
                Objective::Npc(npc_id) => {
                    npc_id.write(output, ctx)?;
                }
                Objective::GameObject(game_object_id) => {
                    (game_object_id | 0x80000000).write(output, ctx)?;
                }
            }
            Ok(())
        }
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct QuestObjective {
        pub objective: Objective,
        pub count: u32,
        #[deku(pad_bytes_after = "4")]
        pub item_drop: u32,
    }

    #[derive(Debug, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgQuestQueryResponse)]
    pub struct QuestQueryResponse {
        pub quest_id: u32,
        pub quest_method: QuestMethod,
        pub quest_level: i32, // may be -1, static data, in other cases must be used dynamic level: Player::GetQuestLevel (0 is not known, but assuming this is no longer valid for quest intended for client)
        pub quest_min_level: u32,
        pub quest_sort: QuestSort, // zone or sort to display in quest log
        pub quest_type: QuestType,
        pub quest_suggested_players: u32,
        pub require_faction_id: [u32; 2], // shown in quest log as part of quest objective (same/opposite faction)
        pub require_faction_value: [i32; 2], // shown in quest log as part of quest objective (same/opposite faction)
        pub next_quest_id: u32,              // client will request this quest from NPC, if not 0
        pub xp_difficulty: u32,              // used for calculating rewarded experience
        #[deku(
            update = "if self.flags.is_quest_flags_hidden_rewards() { self.reward_money } else { 0 }"
        )]
        pub reward_money: u32, // reward money (below max lvl)
        pub reward_bonus_money: u32,         // bonus money for max level
        pub reward_display_spell: u32, // reward spell, this spell will be displayed (icon) in quest completion window
        pub reward_spell: i32,         // reward spell, this spell will be casted on player
        pub reward_honor: u32,         // reward honor points
        pub reward_kill_honor: f32,    // reward honor points
        pub start_item: u32,           // start quest with item
        pub flags: QuestFlags,         // quest flags
        pub reward_title: u32,         // reward title
        pub required_player_kills: u32, // number of player kills required for quest completion
        pub reward_talents: u32,       // reward talents
        pub reward_arena_points: u32,  // reward arena points
        // TODO: Which flags?
        pub reward_faction_flags: u32, // reward faction id
        #[deku(
            update = "if self.flags.is_quest_flags_hidden_rewards() { self.reward_items } else { Default::default() }"
        )]
        pub reward_items: [u32; 4], // reward items
        #[deku(
            update = "if self.flags.is_quest_flags_hidden_rewards() { self.reward_amount } else { Default::default() }"
        )]
        pub reward_amount: [u32; 4], // number of reward items
        #[deku(
            update = "if self.flags.is_quest_flags_hidden_rewards() { self.reward_choice_items } else { Default::default() }"
        )]
        pub reward_choice_items: [QuestInfoChoiceItem; 6], // reward items that player can choose (if not 0 then reward_items will not be given)
        pub reward_faction_id: [u32; 5], // faction ids for the 5 factions that player can gain reputation with
        pub reward_faction_value: [i32; 5], // faction values for the 5 factions that player can gain reputation with
        pub reward_faction_value_override: [i32; 5], // unknown usage
        pub poi_continent: u32, // point of interest area id    TODO: Research what this is
        pub poi_x: f32,         // point of interest x
        pub poi_y: f32,         // point of interest y
        pub poi_priority: u32,  // point of interest priority
        #[deku(writer = "crate::write_c_string(deku::output, &self.title)")]
        pub title: String, // quest title
        #[deku(writer = "crate::write_c_string(deku::output, &self.objectives)")]
        pub objectives: String, // quest objectives
        #[deku(writer = "crate::write_c_string(deku::output, &self.details)")]
        pub details: String, // quest details
        #[deku(writer = "crate::write_c_string(deku::output, &self.area_description)")]
        pub area_description: String, // quest area description
        #[deku(writer = "crate::write_c_string(deku::output, &self.completed_text)")]
        pub completed_text: String, // display in quest objectives window once all objectives are completed
        pub quest_objectives: [QuestObjective; 4], // quest objectives
        pub quest_item_objectives: [QuestItemObjective; 6], // quest item objectives
        #[deku(
            writer = "self.objective_text.iter().map(|txt| crate::write_c_string(deku::output, txt)).collect::<Result<_, _>>()"
        )]
        pub objective_text: [String; 4], // quest objective text
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct QuestItemObjective {
        pub item_id: u32,
        pub count: u32,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct QuestChoiceItem {
        pub item_id: u32,
        pub quantity: u32,
        pub display_id: u32,
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone)]
    pub struct QuestRewards {
        #[deku(temp, temp_value = "unfiltered_choice_items.len() as u32")]
        pub unfiltered_choice_items_size: u32,
        pub unfiltered_choice_items: Vec<QuestChoiceItem>,
        #[deku(temp, temp_value = "reward_items.len() as u32")]
        pub reward_items_size: u32,
        pub reward_items: Vec<QuestItemObjective>,
        pub reward_money: u32,
        pub reward_xp_difficulty: u32,
        pub reward_honor: u32,
        pub reward_kill_honor: f32,
        pub reward_display_spell: u32,
        pub reward_spell: i32,
        pub reward_title: u32,
        pub reward_talents: u32,
        pub reward_arena_points: u32,
        pub reward_faction_flags: u32,
        pub reward_faction_id: [u32; 5],
        pub reward_faction_value: [i32; 5],
        pub reward_faction_override: [u32; 5],
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct DescEmote {
        pub emote_type: u32,
        pub delay: u32,
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, ServerPacket)]
    #[opcode(Opcode::SmsgQuestGiverQuestDetails)]
    pub struct QuestGiverQuestDetails {
        pub guid: Guid,
        pub inform_unit: Guid,
        pub quest_id: u32,
        #[deku(writer = "crate::write_c_string(deku::output, &self.title)")]
        pub title: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.details)")]
        pub details: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.objectives)")]
        pub objectives: String,
        pub is_auto_launched: bool,
        pub quest_flags: QuestFlags,
        pub quest_suggested_players: u32,
        pub start_cheat: bool,
        pub quest_rewards: QuestRewards,
        #[deku(temp, temp_value = "desc_emotes.len() as u32")]
        pub desc_emotes_size: u32,
        pub desc_emotes: Vec<DescEmote>,
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, ServerPacket)]
    #[opcode(Opcode::SmsgQuestGiverOfferRewardMessage)]
    pub struct QuestGiverOfferRewardMessage {
        pub guid: Guid,
        pub quest_id: u32,
        #[deku(writer = "crate::write_c_string(deku::output, &self.title)")]
        pub title: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.reward_text)")]
        pub reward_text: String,
        pub is_auto_launched: bool,
        pub quest_flags: QuestFlags,
        pub quest_suggested_players: u32,
        #[deku(temp, temp_value = "desc_emotes.len() as u32")]
        pub desc_emotes_size: u32,
        pub desc_emotes: Vec<DescEmote>,
        pub quest_rewards: QuestRewards,
    }
}
