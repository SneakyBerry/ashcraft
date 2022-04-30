use crate::classes::Classes;
use crate::constants::ENABLE_ENCRYPTION_SEED;
use crate::crypt::{INITIALIZED_RSA, RSA};
use crate::packets::{ClientPacket, IntoServerPacket};
use crate::races::Races;
use crate::OpcodeServer;
use bytes::{BufMut, BytesMut};
use deku::prelude::*;
use hmac::{Hmac, Mac};
use rand::Rng;
use rustycraft_protocol::expansions::Expansions;
use rustycraft_protocol::rpc_responses::WowRpcResponse;
use sha2::{Digest, Sha256};

#[derive(Debug, DekuWrite)]
#[deku(type = "u32")]
#[deku(endian = "little")]
#[repr(u32)]
pub enum ConnectToSerial {
    None = 0,
    Realm = 14,
    WorldAttempt1 = 17,
    WorldAttempt2 = 35,
    WorldAttempt3 = 53,
    WorldAttempt4 = 71,
    WorldAttempt5 = 89,
}

#[derive(Debug, DekuRead)]
pub struct Ping {
    #[deku(endian = "little")]
    pub serial: u32,
    #[deku(endian = "little")]
    pub latency: u32,
}

#[derive(Debug, DekuWrite)]
pub struct Pong {
    #[deku(endian = "little")]
    serial: u32,
}

impl IntoServerPacket for Pong {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::Pong
    }
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
    #[deku(endian = "little")]
    dos_challenge: [u32; 8],
    challenge: [u8; 16],
    dos_zero_bits: u8,
}

impl IntoServerPacket for AuthChallenge {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::AuthChallenge
    }
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
    pub dos_response: u64,
    #[deku(endian = "little")]
    pub region_id: u32,
    #[deku(endian = "little")]
    pub battle_group_id: u32,
    #[deku(endian = "little")]
    pub realm_id: u32,
    pub local_challenge: [u8; 16],
    pub digest: [u8; 24],
    #[deku(bits = "1", pad_bits_after = "7")]
    pub use_ip_v6: bool,
    #[deku(endian = "little")]
    _realm_join_ticket_size: u32,
    #[deku(count = "_realm_join_ticket_size")]
    #[deku(map = "crate::utils::parse_string")]
    pub realm_join_ticket: String,
}

#[derive(Debug, DekuRead)]
pub struct AuthContinuedSession {
    #[deku(endian = "little")]
    pub dos_response: u64,
    #[deku(endian = "little")]
    pub raw: u64,
    pub local_challenge: [u8; 16],
    pub digest: [u8; 24],
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
    realm_name_actual_len: u8,
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
            unknown_735: 4,
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
    #[deku(bits = "7")]
    #[deku(pad_bits_after = "1")]
    name_len: u8, // Not actually 8 bits, only 7
    #[deku(bits = "10", endian = "little")]
    #[deku(pad_bits_after = "6")]
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
    class: Classes,
    active_expansion_level: Expansions,
    account_expansion_level: Expansions,
}

impl Class {
    pub fn new(
        class: Classes,
        active_expansion_level: Expansions,
        account_expansion_level: Expansions,
    ) -> Class {
        Class {
            class,
            active_expansion_level,
            account_expansion_level,
        }
    }
}

// TODO: Move to another module
#[derive(Debug, DekuWrite)]
pub struct RaceClassAvailability {
    race_id: Races,
    #[deku(update = "self.classes.len()", endian = "little")]
    classes_size: u32,
    #[deku(count = "classes_size")]
    classes: Vec<Class>,
}

impl RaceClassAvailability {
    pub fn new(race_id: Races, classes: Vec<Class>) -> RaceClassAvailability {
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
    ///the current server expansion
    active_expansion_level: Expansions,
    ///the current expansion of this account
    account_expansion_level: Expansions,
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
        active_expansion_level: Expansions,
        account_expansion_level: Expansions,
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

impl IntoServerPacket for AuthResponse {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::AuthResponse
    }
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

#[derive(Debug, DekuWrite)]
pub struct EncryptedMode {
    #[deku(count = "256")]
    hmac_sha_256: Vec<u8>,
    #[deku(bits = "1")]
    #[deku(pad_bits_after = "7")]
    enabled: bool,
}

impl IntoServerPacket for EncryptedMode {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::EnterEncryptedMode
    }
}

impl EncryptedMode {
    pub fn new(encryptor: &RSA, encryption_key: &[u8]) -> EncryptedMode {
        let mut hash = <Hmac<Sha256>>::new_from_slice(encryption_key).unwrap();
        hash.update(&[true as u8]);
        hash.update(&ENABLE_ENCRYPTION_SEED);
        let key_hash = hash.finalize().into_bytes();
        let signature = encryptor.sign(&key_hash);
        EncryptedMode {
            hmac_sha_256: signature,
            enabled: true,
        }
    }
}

#[derive(Debug, DekuWrite)]
pub struct ConnectTo {
    #[deku(skip)]
    signature_size: u32,
    #[deku(count = "signature_size")]
    signature: Vec<u8>,
    #[deku(skip)]
    where_buf_size: u32,
    #[deku(count = "where_buf_size")]
    where_buf: Vec<u8>,
    #[deku(endian = "little")]
    port: u16,
    serial: ConnectToSerial,
    conn: u8,
    #[deku(endian = "little")]
    key: u64,
}

#[derive(Debug, Clone, Copy, DekuWrite)]
#[deku(type = "u8")]
#[repr(u8)]
pub enum AddrType {
    IPv4 = 1,
    IPv6 = 2,
}

impl ConnectTo {
    pub fn new(
        account_id: u32,
        serial: ConnectToSerial,
        address: &[u8; 4],
        port: u16,
        addr_type: AddrType,
    ) -> ConnectTo {
        let key: u32 = rand::thread_rng().gen_range(0..0x7FFFFFFF);
        let key = account_id as u64 | 1 << 32 | ((key as u64) << 1) << 32;
        let mut where_buf = BytesMut::new();
        where_buf.put_u8(addr_type as u8);
        where_buf.extend(address);
        let tp = addr_type as u32;

        let mut hasher = sha2::Sha256::new();
        hasher.update(&where_buf);
        hasher.update(&tp.to_le_bytes());
        hasher.update(&port.to_le_bytes());
        let hash = hasher.finalize();

        let signature = INITIALIZED_RSA.sign(&hash);

        ConnectTo {
            signature_size: signature.len() as u32,
            signature,
            where_buf_size: where_buf.len() as u32,
            where_buf: where_buf.to_vec(),
            port,
            serial,
            conn: 1,
            key,
        }
    }
}

impl IntoServerPacket for ConnectTo {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::ConnectTo
    }
}

#[derive(Debug, Clone, Copy, DekuWrite)]
pub struct ResumeComms {}

impl IntoServerPacket for ResumeComms {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::ResumeComms
    }
}
