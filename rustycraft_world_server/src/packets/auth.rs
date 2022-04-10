use crate::OpcodeServer;
use deku::prelude::*;
use rustycraft_protocol::errors::WowRpcResponse;

#[derive(Debug, DekuRead)]
pub struct Ping {
    #[deku(endian = "little")]
    serial: u32,
    #[deku(endian = "little")]
    latency: u32,
}

#[derive(Debug, DekuWrite)]
pub struct Pong {
    #[deku(endian = "little")]
    serial: u32,
}

impl From<Ping> for Pong {
    fn from(ping: Ping) -> Self {
        Pong {
            serial: ping.serial,
        }
    }
}

#[derive(Debug, DekuWrite)]
pub struct AuthChallenge {
    challenge: [u8; 16],
    #[deku(endian = "little")]
    dos_challenge: [u32; 8],
    dos_zero_bits: u8,
}

impl AuthChallenge {
    pub fn new(challenge: [u8; 16], dos_challenge: [u32; 8]) -> AuthChallenge {
        AuthChallenge {
            challenge,
            dos_challenge,
            dos_zero_bits: 1,
        }
    }
}

#[derive(Debug, DekuRead)]
pub struct AuthSession {
    #[deku(endian = "little")]
    dos_response: u64,
    #[deku(endian = "little")]
    region_id: u32,
    #[deku(endian = "little")]
    battle_group_id: u32,
    #[deku(endian = "little")]
    realm_id: u32,
    local_challenge: [u8; 16],
    digest: [u8; 24],
    #[deku(bits = "1", pad_bits_after = "7")]
    use_ip_v6: bool,
    #[deku(endian = "little")]
    realm_join_ticket_size: u32,
    #[deku(count = "realm_join_ticket_size")]
    #[deku(map = "crate::utils::parse_string")]
    realm_join_ticket: String,
}

#[derive(Debug, DekuWrite)]
pub struct AuthWaitInfo {
    ///position of the account in the login queue
    #[deku(endian = "little")]
    wait_count: u32,
    ///Wait time in login queue in minutes, if sent queued and this value is 0 client displays "unknown time"
    #[deku(endian = "little")]
    wait_time: u32,
    ///true if the account has a forced character migration pending. @todo implement
    #[deku(bits = "1", pad_bits_after = "7")]
    has_fcm: bool,
}

impl AuthWaitInfo {
    pub fn new(wait_count: u32, wait_time: u32, has_fcm: bool) -> AuthWaitInfo {
        AuthWaitInfo {
            wait_count,
            wait_time,
            has_fcm,
        }
    }
}

#[derive(Debug, DekuWrite)]
#[deku(endian = "little")]
pub struct VirtualRealmNameInfo {
    #[deku(bits = "1")]
    is_local: bool,
    #[deku(bits = "1", pad_bits_after = "6")]
    is_internal: bool,
    #[deku(update = "self.realm_name_actual.len()")]
    realm_name_actual_len: u8,
    #[deku(update = "self.realm_name_normalized.len()")]
    realm_name_normalized_len: u8,
    #[deku(count = "realm_name_actual_len")]
    #[deku(writer = "crate::utils::string_writer(deku::output, &self.realm_name_actual)")]
    realm_name_actual: String,
    #[deku(count = "realm_name_normalized_len")]
    #[deku(writer = "crate::utils::string_writer(deku::output, &self.realm_name_normalized)")]
    realm_name_normalized: String,
}

impl VirtualRealmNameInfo {
    pub fn new(
        is_local: bool,
        is_internal: bool,
        realm_name_actual: String,
    ) -> VirtualRealmNameInfo {
        let realm_name_normalized = realm_name_actual.clone(); // TODO: Normalize
        VirtualRealmNameInfo {
            is_local,
            is_internal,
            realm_name_actual_len: realm_name_actual.len() as u8,
            realm_name_normalized_len: realm_name_normalized.len() as u8,
            realm_name_actual,
            realm_name_normalized,
        }
    }
}

#[derive(Debug, DekuWrite)]
pub struct VirtualRealmInfo {
    #[deku(endian = "little")]
    realm_address: u32,
    realm_name_info: VirtualRealmNameInfo,
}

impl VirtualRealmInfo {
    pub fn new(realm_address: u32, realm_name_info: VirtualRealmNameInfo) -> VirtualRealmInfo {
        VirtualRealmInfo {
            realm_address,
            realm_name_info,
        }
    }
}

#[derive(Debug, DekuWrite)]
pub struct GameTime {
    #[deku(endian = "little")]
    billing_plan: u32,
    #[deku(endian = "little")]
    time_remain: u32,
    #[deku(endian = "little")]
    unknown_735: u32,
    // 3x same bit is not a mistake - preserves legacy client behavior of BillingPlanFlags::SESSION_IGR
    #[deku(bits = "1")]
    in_game_room1: bool, // inGameRoom check in function checking which lua event to fire when remaining time is near end - BILLING_NAG_DIALOG vs IGR_BILLING_NAG_DIALOG
    #[deku(bits = "1")]
    in_game_room2: bool, // inGameRoom lua return from Script_GetBillingPlan
    #[deku(bits = "1", pad_bits_after = "5")]
    in_game_room3: bool, // not used anywhere in the client
}

impl GameTime {
    pub fn new(billing_plan: u32, time_remain: u32, in_game_room: bool) -> GameTime {
        GameTime {
            billing_plan,
            time_remain,
            unknown_735: 0,
            in_game_room1: in_game_room,
            in_game_room2: in_game_room,
            in_game_room3: in_game_room,
        }
    }
}

#[derive(Debug, DekuWrite)]
pub struct CharacterTemplateClass {
    faction_group: u8,
    class_id: u8,
}

impl CharacterTemplateClass {
    pub fn new(faction_group: u8, class_id: u8) -> CharacterTemplateClass {
        CharacterTemplateClass {
            faction_group,
            class_id,
        }
    }
}

// TODO: Move to another module
#[derive(Debug, DekuWrite)]
pub struct CharacterTemplate {
    #[deku(endian = "little")]
    template_set_id: u32,
    #[deku(update = "self.classes.len()", endian = "little")]
    classes_size: u32,
    #[deku(count = "classes_size")]
    classes: Vec<CharacterTemplateClass>,
    #[deku(bits = "7", update = "self.name.len()")]
    name_len: u8, // Not actually 8 bits, only 7
    #[deku(bits = "10", update = "self.description.len()", endian = "little")]
    #[deku(pad_bits_after = "7")]
    description_len: u16, // Not actually 16 bits, only 10
    #[deku(count = "name_len")]
    #[deku(writer = "crate::utils::string_writer(deku::output, &self.name)")]
    name: String,
    #[deku(count = "description_len")]
    #[deku(writer = "crate::utils::string_writer(deku::output, &self.description)")]
    description: String,
}

impl CharacterTemplate {
    pub fn new(
        template_set_id: u32,
        classes: Vec<CharacterTemplateClass>,
        name: String,
        description: String,
    ) -> CharacterTemplate {
        CharacterTemplate {
            template_set_id,
            classes_size: classes.len() as u32,
            classes,
            name_len: name.len() as u8,
            description_len: description.len() as u16,
            name,
            description,
        }
    }
}

#[derive(Debug, DekuWrite)]
pub struct Class {
    id: u8,
    active_expansion_level: u8,
    account_expansion_level: u8,
}

impl Class {
    pub fn new(id: u8, active_expansion_level: u8, account_expansion_level: u8) -> Class {
        Class {
            id,
            active_expansion_level,
            account_expansion_level,
        }
    }
}

// TODO: Move to another module
#[derive(Debug, DekuWrite)]
pub struct RaceClassAvailability {
    race_id: u8,
    #[deku(update = "self.classes.len()", endian = "little")]
    classes_size: u32,
    #[deku(count = "classes_size")]
    classes: Vec<Class>,
}

impl RaceClassAvailability {
    pub fn new(race_id: u8, classes: Vec<Class>) -> RaceClassAvailability {
        RaceClassAvailability {
            race_id,
            classes_size: classes.len() as u32,
            classes,
        }
    }
}

#[derive(Debug, DekuWrite)]
pub struct AuthSuccessInfo {
    ///a special identifier made from the Index, BattleGroup and Region.
    #[deku(endian = "little")]
    virtual_realm_address: u32,
    #[deku(update = "self.virtual_realms.len()", endian = "little")]
    virtual_realms_size: u32,
    ///affects the return value of the GetBillingTimeRested() client API call, it is the number of seconds you have left until the experience points and loot you receive from creatures and quests is reduced. It is only used in the Asia region in retail, it's not implemented in TC and will probably never be.
    #[deku(endian = "little")]
    time_rested: u32,
    ///the current server expansion, the possible values are in @ref Expansions
    active_expansion_level: u8,
    ///the current expansion of this account, the possible values are in @ref Expansions
    account_expansion_level: u8,
    ///@todo research
    #[deku(endian = "little")]
    time_seconds_until_pc_kick: u32,

    #[deku(update = "self.available_classes.len()", endian = "little")]
    available_classes_size: u32,
    #[deku(update = "self.templates.len()", endian = "little")]
    templates_size: u32,

    ///this is probably used for the ingame shop. @todo implement
    #[deku(endian = "little")]
    currency_id: u32,
    #[deku(endian = "little")]
    time: u64,

    ///the minimum AccountExpansion required to select race/class combinations
    available_classes: Vec<RaceClassAvailability>,

    #[deku(bits = "1")]
    is_expansion_trial: bool,
    ///forces the client to always use a character template when creating a new character. @see Templates. @todo implement
    #[deku(bits = "1")]
    force_character_template: bool,
    #[deku(update = "self.num_players_horde.is_some()")]
    #[deku(bits = "1")]
    has_num_players_horde: bool,
    #[deku(update = "self.num_players_alliance.is_some()")]
    #[deku(bits = "1")]
    has_num_players_alliance: bool,
    #[deku(update = "self.expansion_trial_expiration.is_some()")]
    #[deku(bits = "1")]
    #[deku(pad_bits_after = "3")]
    has_expansion_trial_expiration: bool,

    game_time_info: GameTime,

    ///number of horde players in this realm. @todo implement
    #[deku(endian = "little")]
    num_players_horde: Option<u16>,
    ///number of alliance players in this realm. @todo implement
    #[deku(endian = "little")]
    num_players_alliance: Option<u16>,
    ///expansion trial expiration unix timestamp
    #[deku(endian = "little")]
    expansion_trial_expiration: Option<i32>,

    ///list of realms connected to this one (inclusive) @todo implement
    virtual_realms: Vec<VirtualRealmInfo>,

    ///list of pre-made character templates.
    templates: Vec<CharacterTemplate>,
}

impl AuthSuccessInfo {
    pub fn new(
        virtual_realm_address: u32,
        time_rested: u32,
        active_expansion_level: u8,
        account_expansion_level: u8,
        time_seconds_until_pc_kick: u32,
        currency_id: u32,
        time: u64,
        available_classes: Vec<RaceClassAvailability>,
        is_expansion_trial: bool,
        force_character_template: bool,
        game_time_info: GameTime,
        num_players_horde: Option<u16>,
        num_players_alliance: Option<u16>,
        expansion_trial_expiration: Option<i32>,
        virtual_realms: Vec<VirtualRealmInfo>,
        templates: Vec<CharacterTemplate>,
    ) -> AuthSuccessInfo {
        AuthSuccessInfo {
            virtual_realm_address,
            virtual_realms_size: virtual_realms.len() as u32,
            time_rested,
            active_expansion_level,
            account_expansion_level,
            time_seconds_until_pc_kick,
            available_classes_size: available_classes.len() as u32,
            templates_size: templates.len() as u32,
            currency_id,
            time,
            available_classes,
            is_expansion_trial,
            force_character_template,
            has_num_players_horde: num_players_horde.is_some(),
            has_num_players_alliance: num_players_alliance.is_some(),
            has_expansion_trial_expiration: expansion_trial_expiration.is_some(),
            game_time_info,
            num_players_horde,
            num_players_alliance,
            expansion_trial_expiration,
            virtual_realms,
            templates,
        }
    }
}

#[derive(Debug, DekuWrite)]
pub struct AuthResponse {
    ///the result of the authentication process. Look at [rustycraft_protocol::errors::WowRpcResponse]
    result: WowRpcResponse,
    #[deku(bits = "1")]
    has_success_info: bool,
    #[deku(bits = "1")]
    #[deku(pad_bits_after = "6")]
    has_wait_info: bool,
    ///contains the packet data in case that it has account information (It is never set when WaitInfo is set), otherwise its contents are undefined.
    success_info: Option<AuthSuccessInfo>,
    ///contains the queue wait information in case the account is in the login queue.
    wait_info: Option<AuthWaitInfo>,
}

impl AuthResponse {
    pub fn new(
        result: WowRpcResponse,
        success_info: Option<AuthSuccessInfo>,
        wait_info: Option<AuthWaitInfo>,
    ) -> AuthResponse {
        AuthResponse {
            result,
            has_success_info: success_info.is_some(),
            has_wait_info: wait_info.is_some(),
            success_info,
            wait_info,
        }
    }
}
