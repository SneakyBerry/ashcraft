use crate::prelude::Guid;
use deku::prelude::*;
use ashcraft_derive::ServerPacket;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct GuildPermissionsTab {
    pub flags: u32,
    pub withdraw_limit: u32,
}

pub mod client {
    use super::*;

    #[derive(Debug, Clone, DekuRead)]
    pub struct QueryGuildInfo {
        pub guild_id: u32,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildCreate {
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        pub name: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildGetInfo;

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildGetRoster;

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildUpdateMotdText {
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        // TODO: Check TC validation
        pub motd: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct AcceptGuildInvite;

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildDeclineInvitation;

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildInviteByName {
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        // 48 bytes max in TC
        pub name: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildEventLogQuery;

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildPermissionsQuery;

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildSetRankPermissions {
        pub rank_id: u32,
        pub flags: u32,
        pub rank_name: u32,
        pub withdraw_gold_limit: u32,
        pub tabs: [GuildPermissionsTab; 6],
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildAddRank {
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        // 15 bytes max in TC
        pub name: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildDeleteRank;

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildUpdateInfoText {
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        // 500 bytes max in TC
        pub info: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildSetMemberNote {
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        pub name: String,
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        pub note: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildDelete;

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildDemoteMember {
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        pub name: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildPromoteMember {
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        pub name: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildOfficerRemoveMember {
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        pub name: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildLeave;

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildBankActivate {
        pub banker: Guid,
        pub full_update: bool,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildBankBuyTab {
        pub banker: Guid,
        pub tab: u8,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildBankUpdateTab {
        pub banker: Guid,
        pub tab: u8,
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        pub name: String,
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        pub icon: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildBankDepositMoney {
        pub banker: Guid,
        pub money: u32,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildBankQueryTab {
        pub banker: Guid,
        pub tab: u8,
        pub full_update: bool,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildBankRemainingWithdrawMoneyQuery;

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildBankWithdrawMoney {
        pub banker: Guid,
        pub money: u32,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct BankItem {
        pub tab: u8,
        pub slot: u8,
        pub id: u32,
    }

    #[derive(Debug, Clone, DekuRead)]
    #[deku(type = "bool")]
    pub enum BankOnly {
        #[deku(id = "true")]
        True {
            dst: BankItem,
            src: BankItem,
            auto_store: bool,
            item_count: i32,
        },
        #[deku(id = "false")]
        False { src: BankItem },
    }

    #[derive(Debug, Clone, DekuRead)]
    #[deku(type = "bool")]
    pub enum AutoStore {
        #[deku(id = "true")]
        True {
            item_count: i32,
            to_slot: u8,
            stack_count: i32,
        },
        #[deku(id = "false")]
        False {
            container_slot: u8,
            container_item_slot: u8,
            to_slot: u8,
            stack_count: i32,
        },
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildBankSwapItems {
        pub banker: Guid,
        pub bank_type: BankOnly,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildBankLogQuery {
        pub tab: u8,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildBankTextQuery {
        pub tab: u8,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildBankSetTabText {
        pub tab: u8,
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        pub text: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct GuildSetGuildMaster {
        #[deku(reader = "crate::read_c_string(deku::rest)")]
        pub name: String,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct SaveGuildEmblem {
        pub vendor: Guid,
        pub emblem_style: i32,
        pub emblem_color: i32,
        pub border_style: i32,
        pub border_color: i32,
        pub background_color: i32,
        pub background: i32,
    }
}

pub mod server {
    use super::*;
    use crate::opcodes::Opcode;
    use crate::prelude::{Area, Class, Gender};

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone, ServerPacket)]
    #[opcode(Opcode::SmsgGuildQueryResponse)]
    pub struct QueryGuildInfoResponse {
        pub id: u32,
        #[deku(writer = "crate::write_c_string(deku::output, &self.name)")]
        pub name: String,
        #[deku(
            writer = "self.ranks.iter().map(|val| crate::write_c_string(deku::output, val)).collect::<Result<_, _>>()"
        )]
        pub ranks: Vec<String>,
        pub emblem_style: u32,
        pub emblem_color: u32,
        pub border_style: u32,
        pub border_color: u32,
        pub background_color: u32,
        #[deku(temp, temp_value = "ranks.len() as u32")]
        pub rank_count: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgGuildInfo)]
    pub struct GuildInfoResponse {
        #[deku(writer = "crate::write_c_string(deku::output, &self.name)")]
        pub name: String,
        pub create_date: u32, // Packed time
        pub members_count: u32,
        pub accounts_count: u32,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct GuildRosterMemberData {
        pub guid: Guid,
        pub status: u8,
        #[deku(writer = "crate::write_c_string(deku::output, &self.name)")]
        pub name: String,
        pub rank_id: i32,
        pub level: u8,
        pub class: Class,
        pub gender: Gender,
        // TODO: Check, in TC it is i32
        pub area_id: Area,
        #[deku(cond = "!(self.last_save.is_some() && self.status != 0)", skip)]
        pub last_save: Option<f32>,
        #[deku(writer = "crate::write_c_string(deku::output, &self.note)")]
        pub note: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.officer_note)")]
        pub officer_note: String,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct GuildRankTab {
        pub flags: u32,
        pub gold_withdraw_limit: u32,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct GuildRankData {
        pub flags: u32,
        pub gold_withdraw_limit: u32,
        pub tabs: [GuildRankTab; 6],
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone, ServerPacket)]
    #[opcode(Opcode::SmsgGuildRoster)]
    pub struct GuildRoster {
        #[deku(temp, temp_value = "member_data.len() as u32")]
        pub member_data_size: u32,
        #[deku(writer = "crate::write_c_string(deku::output, &self.welcome_text)")]
        pub welcome_text: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.info_text)")]
        pub info_text: String,
        #[deku(temp, temp_value = "rank_data.len() as u32")]
        pub rank_data_size: u32,
        pub rank_data: Vec<GuildRankData>,
        pub member_data: Vec<GuildRosterMemberData>,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgGuildCommandResult)]
    pub struct GuildCommandResult {
        pub command: i32,
        #[deku(writer = "crate::write_c_string(deku::output, &self.name)")]
        pub name: String,
        pub result: i32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgGuildInvite)]
    pub struct GuildInvite {
        #[deku(writer = "crate::write_c_string(deku::output, &self.inviter_name)")]
        pub inviter_name: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.guild_name)")]
        pub guild_name: String,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u8")]
    pub enum GuildEvents {
        Promotion = 0,
        Demotion = 1,
        Motd = 2,
        Joined = 3,
        Left = 4,
        Removed = 5,
        LeaderIs = 6,
        LeaderChanged = 7,
        Disbanded = 8,
        Tabardchange = 9,
        RankUpdated = 10,
        RankDeleted = 11,
        SignedOn = 12,
        SignedOff = 13,
        GuildbankbagslotsChanged = 14,
        /// TODO: Sent when items are moved in gbank - all players with bank open will send tab query
        BankTabPurchased = 15,
        BankTabUpdated = 16,
        BankMoneySet = 17,
        BankTabAndMoneyUpdated = 18,
        BankTextChanged = 19,
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone, ServerPacket)]
    #[opcode(Opcode::SmsgGuildEvent)]
    pub struct GuildEvent {
        pub event_type: GuildEvents,
        #[deku(temp, temp_value = "params.len() as u8")]
        pub params_size: u8,
        #[deku(
            writer = "self.params.iter().map(|val| crate::write_c_string(deku::output, val)).collect::<Result<_, _>>()"
        )]
        pub params: Vec<String>,
        #[deku(
            cond = "!(self.guid.is_some() && matches!(self.event_type, GuildEvents::Joined | GuildEvents::Left | GuildEvents::SignedOn | GuildEvents::SignedOff))",
            skip
        )]
        pub guid: Option<Guid>,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct GuildEventEntry {
        pub player_guid: Guid,
        pub other_guid: Guid,
        pub transaction_type: u8,
        pub rank_id: u8,
        pub transaction_date: u32,
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone, ServerPacket)]
    #[opcode(Opcode::MsgGuildEventLogQuery)]
    pub struct GuildEventLogQueryResults {
        #[deku(temp, temp_value = "entries.len() as u8")]
        pub entry_size: u8,
        pub entries: Vec<GuildEventEntry>,
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone, ServerPacket)]
    #[opcode(Opcode::MsgGuildPermissions)]
    pub struct GuildPermissionsQueryResults {
        pub rank_id: u32,
        pub flags: u32,
        pub withdraw_gold_limit: u32,
        #[deku(temp, temp_value = "tabs.len() as u8")]
        pub num_tabs: u8,
        pub tabs: Vec<GuildPermissionsTab>,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::MsgGuildBankMoneyWithdrawn)]
    pub struct GuildBankRemainingWithdrawMoney {
        pub withdraw_remaining: i32,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct GuildBankSocketEnchant {
        pub index: u8,
        pub enchant_id: i32,
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone)]
    pub struct GuildBankItemInfo {
        pub item_id: u32,
        pub flags: i32,
        pub random_properties_id: i32,
        pub random_properties_seed: i32,
        pub count: i32,
        pub enchantment_id: i32,
        pub charges: u8,
        #[deku(temp, temp_value = "socket_enchants.len() as u8")]
        pub socket_enchant_size: u8,
        pub socket_enchants: Vec<GuildBankSocketEnchant>,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct GuildBankTabInfo {
        #[deku(writer = "crate::write_c_string(deku::output, &self.name)")]
        pub name: String,
        #[deku(writer = "crate::write_c_string(deku::output, &self.icon)")]
        pub icon: String,
    }


    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone)]
    pub struct TabUpdate {
        #[deku(temp, temp_value = "tabs.len() as u8")]
        pub size: u8,
        pub tabs: Vec<GuildBankTabInfo>,
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone, ServerPacket)]
    #[opcode(Opcode::SmsgGuildBankList)]
    pub struct GuildBankQueryResults {
        pub money: u64,
        pub tab: u8,
        pub withdraw_remaining: i32,
        pub full_update: bool,
        #[deku(cond = "!self.full_update && self.tab != 0", skip)]
        pub tab_update: Option<TabUpdate>,
        #[deku(temp, temp_value = "item_info.len() as u8")]
        pub item_info_size: u8,
        pub item_info: Vec<GuildBankItemInfo>,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "i8")]
    pub enum GuildBankEventLogTypes {
        DepositItem = 1,
        WithdrawItem = 2,
        MoveItem = 3,
        DepositMoney = 4,
        WithdrawMoney = 5,
        RepairMoney = 6,
        MoveItem2 = 7,
        Unk1 = 8,
        BuySlot = 9,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct GuildBankLogEntry {
        pub entry_type: GuildBankEventLogTypes,
        pub player_guid: Guid,
        #[deku(
            cond = "!matches!(self.entry_type, GuildBankEventLogTypes::DepositItem | GuildBankEventLogTypes::WithdrawItem)",
            skip
        )]
        pub item_id: Option<i32>,
        #[deku(
            cond = "!matches!(self.entry_type, GuildBankEventLogTypes::DepositItem | GuildBankEventLogTypes::WithdrawItem)",
            skip
        )]
        pub count: Option<i32>,
        #[deku(
            cond = "!matches!(self.entry_type, GuildBankEventLogTypes::DepositItem | GuildBankEventLogTypes::WithdrawItem | GuildBankEventLogTypes::MoveItem | GuildBankEventLogTypes::MoveItem2)",
            skip
        )]
        pub other_tab: Option<i32>,
        #[deku(
            cond = "matches!(self.entry_type, GuildBankEventLogTypes::DepositItem | GuildBankEventLogTypes::WithdrawItem | GuildBankEventLogTypes::MoveItem | GuildBankEventLogTypes::MoveItem2)",
            skip
        )]
        pub money: Option<u32>,
        pub time_offset: u32,
    }

    #[deku_derive(DekuWrite)]
    #[derive(Debug, Clone, ServerPacket)]
    #[opcode(Opcode::MsgGuildBankLogQuery)]
    pub struct GuildBankLogQueryResults {
        pub tab: u8,
        #[deku(temp, temp_value = "entries.len() as u8")]
        pub entries_size: u8,
        pub entries: Vec<GuildBankLogEntry>,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::MsgGuildBankLogQuery)]
    pub struct GuildBankTextQueryResult {
        pub tab: u8,
        #[deku(writer = "crate::write_c_string(deku::output, &self.text)")]
        pub text: String,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "i32")]
    pub enum GuildEmblemSaveResponse {
        Success = 0,
        InvalidTabardColors = 1,
        NoGuild = 2,
        NotGuildMaster = 3,
        NotEnoughMoney = 4,
        InvalidVendor = 5,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::MsgSaveGuildEmblem)]
    pub struct PlayerSaveGuildEmblem {
        pub error: GuildEmblemSaveResponse,
    }
}
