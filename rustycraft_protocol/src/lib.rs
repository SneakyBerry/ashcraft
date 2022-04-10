pub mod errors;
pub mod messages;
#[allow(dead_code)]
#[allow(warnings)]
pub mod google {
    pub mod protobuf {}
}
pub mod blizzard {
    pub mod telemetry {
        pub mod wow {
            pub mod client {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClientInfo {
                    #[prost(uint32, optional, tag = "1")]
                    pub cfg_realm_id: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "2")]
                    pub cfg_region_id: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "3")]
                    pub cfg_site_id: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "4")]
                    pub realm_address: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "5")]
                    pub num_clients: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct WorldInfo {
                    #[prost(uint64, optional, tag = "1")]
                    pub player_id: ::core::option::Option<u64>,
                    #[prost(uint32, optional, tag = "2")]
                    pub map_id: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "3")]
                    pub area_id: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AuroraError {
                    #[prost(uint32, optional, tag = "1")]
                    pub error_code: ::core::option::Option<u32>,
                    #[prost(message, optional, tag = "2")]
                    pub client: ::core::option::Option<ClientInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct CasInitError {
                    #[prost(uint32, optional, tag = "1")]
                    pub error_code: ::core::option::Option<u32>,
                    #[prost(string, optional, tag = "2")]
                    pub error_string: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(message, optional, tag = "3")]
                    pub client: ::core::option::Option<ClientInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClientCancelConnect {
                    #[prost(message, optional, tag = "1")]
                    pub client: ::core::option::Option<ClientInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClientDisconnect {
                    #[prost(uint32, optional, tag = "1")]
                    pub disconnect_reason: ::core::option::Option<u32>,
                    #[prost(message, optional, tag = "2")]
                    pub client: ::core::option::Option<ClientInfo>,
                    #[prost(message, optional, tag = "3")]
                    pub world: ::core::option::Option<WorldInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct CriticalStreamingError {
                    #[prost(int32, optional, tag = "1")]
                    pub file_data_id: ::core::option::Option<i32>,
                    #[prost(string, optional, tag = "2")]
                    pub reason: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(message, optional, tag = "3")]
                    pub client: ::core::option::Option<ClientInfo>,
                    #[prost(message, optional, tag = "4")]
                    pub world: ::core::option::Option<WorldInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct WorldLoadFailed {
                    #[prost(int32, optional, tag = "1")]
                    pub file_data_id: ::core::option::Option<i32>,
                    #[prost(message, optional, tag = "2")]
                    pub client: ::core::option::Option<ClientInfo>,
                    #[prost(message, optional, tag = "3")]
                    pub world: ::core::option::Option<WorldInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct WorldSceneLoadFailed {
                    #[prost(int32, optional, tag = "1")]
                    pub file_data_id: ::core::option::Option<i32>,
                    #[prost(message, optional, tag = "2")]
                    pub client: ::core::option::Option<ClientInfo>,
                    #[prost(message, optional, tag = "3")]
                    pub world: ::core::option::Option<WorldInfo>,
                }
            }
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct FileOptions {
            #[prost(string, optional, tag = "1")]
            pub telemetry_message: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct MessageOptions {
            #[prost(bool, optional, tag = "100")]
            pub realtime: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "150")]
            pub longterm: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "200")]
            pub metric_set: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "201")]
            pub metric: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "301")]
            pub entity_time_series: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "401")]
            pub logging: ::core::option::Option<bool>,
            #[prost(string, optional, tag = "501")]
            pub attribution: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "601")]
            pub crm: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct LogMessageOptions {
            #[prost(string, optional, tag = "1")]
            pub grok: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(bool, optional, tag = "2")]
            pub is_json: ::core::option::Option<bool>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct FieldOptions {
            #[prost(string, optional, tag = "1")]
            pub mapped_type: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "100")]
            pub elastic_analyzer: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(bool, optional, tag = "200")]
            pub realtime: ::core::option::Option<bool>,
            #[prost(string, optional, tag = "300")]
            pub elastic_path_move: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "400")]
            pub crm_field_name: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct LogFieldOptions {
            #[prost(string, optional, tag = "1")]
            pub syslog_field: ::core::option::Option<::prost::alloc::string::String>,
        }
    }
}
pub mod bgs {
    pub mod protocol {
        pub mod account {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AccountId {
                    #[prost(fixed32, required, tag = "1")]
                    pub id: u32,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AccountLicense {
                    #[prost(uint32, required, tag = "1")]
                    pub id: u32,
                    #[prost(uint64, optional, tag = "2")]
                    pub expires: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountHandle {
                    #[prost(fixed32, required, tag = "1")]
                    pub id: u32,
                    #[prost(fixed32, required, tag = "2")]
                    pub program: u32,
                    #[prost(uint32, required, tag = "3")]
                    pub region: u32,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AccountReference {
                    #[prost(fixed32, optional, tag = "1")]
                    pub id: ::core::option::Option<u32>,
                    #[prost(string, optional, tag = "2")]
                    pub email: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(message, optional, tag = "3")]
                    pub handle: ::core::option::Option<GameAccountHandle>,
                    #[prost(string, optional, tag = "4")]
                    pub battle_tag: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "5")]
                    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(uint32, optional, tag = "10", default = "0")]
                    pub region: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct Identity {
                    #[prost(message, optional, tag = "1")]
                    pub account: ::core::option::Option<AccountId>,
                    #[prost(message, optional, tag = "2")]
                    pub game_account: ::core::option::Option<GameAccountHandle>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ProgramTag {
                    #[prost(fixed32, optional, tag = "1")]
                    pub program: ::core::option::Option<u32>,
                    #[prost(fixed32, optional, tag = "2")]
                    pub tag: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct RegionTag {
                    #[prost(fixed32, optional, tag = "1")]
                    pub region: ::core::option::Option<u32>,
                    #[prost(fixed32, optional, tag = "2")]
                    pub tag: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AccountFieldTags {
                    #[prost(fixed32, optional, tag = "2")]
                    pub account_level_info_tag: ::core::option::Option<u32>,
                    #[prost(fixed32, optional, tag = "3")]
                    pub privacy_info_tag: ::core::option::Option<u32>,
                    #[prost(fixed32, optional, tag = "4")]
                    pub parental_control_info_tag: ::core::option::Option<u32>,
                    #[prost(message, repeated, tag = "7")]
                    pub game_level_info_tags: ::prost::alloc::vec::Vec<ProgramTag>,
                    #[prost(message, repeated, tag = "9")]
                    pub game_status_tags: ::prost::alloc::vec::Vec<ProgramTag>,
                    #[prost(message, repeated, tag = "11")]
                    pub game_account_tags: ::prost::alloc::vec::Vec<RegionTag>,
                    #[prost(fixed32, optional, tag = "12")]
                    pub security_status_tag: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountFieldTags {
                    #[prost(fixed32, optional, tag = "2")]
                    pub game_level_info_tag: ::core::option::Option<u32>,
                    #[prost(fixed32, optional, tag = "3")]
                    pub game_time_info_tag: ::core::option::Option<u32>,
                    #[prost(fixed32, optional, tag = "4")]
                    pub game_status_tag: ::core::option::Option<u32>,
                    #[prost(fixed32, optional, tag = "5")]
                    pub raf_info_tag: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AccountFieldOptions {
                    #[prost(bool, optional, tag = "1")]
                    pub all_fields: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "2")]
                    pub field_account_level_info: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "3")]
                    pub field_privacy_info: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "4")]
                    pub field_parental_control_info: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "6")]
                    pub field_game_level_info: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "7")]
                    pub field_game_status: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "8")]
                    pub field_game_accounts: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "9")]
                    pub field_security_status: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountFieldOptions {
                    #[prost(bool, optional, tag = "1")]
                    pub all_fields: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "2")]
                    pub field_game_level_info: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "3")]
                    pub field_game_time_info: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "4")]
                    pub field_game_status: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "5")]
                    pub field_raf_info: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SubscriberReference {
                    #[prost(uint64, optional, tag = "1", default = "0")]
                    pub object_id: ::core::option::Option<u64>,
                    #[prost(message, optional, tag = "2")]
                    pub entity_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, optional, tag = "3")]
                    pub account_options: ::core::option::Option<AccountFieldOptions>,
                    #[prost(message, optional, tag = "4")]
                    pub account_tags: ::core::option::Option<AccountFieldTags>,
                    #[prost(message, optional, tag = "5")]
                    pub game_account_options: ::core::option::Option<GameAccountFieldOptions>,
                    #[prost(message, optional, tag = "6")]
                    pub game_account_tags: ::core::option::Option<GameAccountFieldTags>,
                    #[prost(uint64, optional, tag = "7", default = "0")]
                    pub subscriber_id: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AccountLevelInfo {
                    #[prost(message, repeated, tag = "3")]
                    pub licenses: ::prost::alloc::vec::Vec<AccountLicense>,
                    #[prost(fixed32, optional, tag = "4")]
                    pub default_currency: ::core::option::Option<u32>,
                    #[prost(string, optional, tag = "5")]
                    pub country: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(uint32, optional, tag = "6")]
                    pub preferred_region: ::core::option::Option<u32>,
                    #[prost(string, optional, tag = "7")]
                    pub full_name: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "8")]
                    pub battle_tag: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(bool, optional, tag = "9")]
                    pub muted: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "10")]
                    pub manual_review: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "11")]
                    pub account_paid_any: ::core::option::Option<bool>,
                    #[prost(enumeration = "IdentityVerificationStatus", optional, tag = "12")]
                    pub identity_check_status: ::core::option::Option<i32>,
                    #[prost(string, optional, tag = "13")]
                    pub email: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(bool, optional, tag = "14")]
                    pub headless_account: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "15")]
                    pub test_account: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "17")]
                    pub is_sms_protected: ::core::option::Option<bool>,
                    #[prost(uint32, optional, tag = "18")]
                    pub ratings_board_minimum_age: ::core::option::Option<u32>,
                    #[prost(string, optional, tag = "19")]
                    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct PrivacyInfo {
                    #[prost(bool, optional, tag = "3")]
                    pub is_using_rid: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "4")]
                    pub is_visible_for_view_friends: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "5")]
                    pub is_hidden_from_friend_finder: ::core::option::Option<bool>,
                    #[prost(
                        enumeration = "privacy_info::GameInfoPrivacy",
                        optional,
                        tag = "6",
                        default = "PrivacyFriends"
                    )]
                    pub game_info_privacy: ::core::option::Option<i32>,
                    #[prost(bool, optional, tag = "7")]
                    pub only_allow_friend_whispers: ::core::option::Option<bool>,
                }
                #[doc = " Nested message and enum types in `PrivacyInfo`."]
                pub mod privacy_info {
                    #[derive(
                        serde :: Serialize,
                        Clone,
                        Copy,
                        Debug,
                        PartialEq,
                        Eq,
                        Hash,
                        PartialOrd,
                        Ord,
                        :: prost :: Enumeration,
                    )]
                    #[repr(i32)]
                    pub enum GameInfoPrivacy {
                        PrivacyMe = 0,
                        PrivacyFriends = 1,
                        PrivacyEveryone = 2,
                    }
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ParentalControlInfo {
                    #[prost(string, optional, tag = "3")]
                    pub timezone: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(uint32, optional, tag = "4")]
                    pub minutes_per_day: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "5")]
                    pub minutes_per_week: ::core::option::Option<u32>,
                    #[prost(bool, optional, tag = "6")]
                    pub can_receive_voice: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "7")]
                    pub can_send_voice: ::core::option::Option<bool>,
                    #[prost(bool, repeated, packed = "false", tag = "8")]
                    pub play_schedule: ::prost::alloc::vec::Vec<bool>,
                    #[prost(bool, optional, tag = "9")]
                    pub can_join_group: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "10")]
                    pub can_use_profile: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct PlayScheduleRestriction {
                    #[prost(bool, repeated, packed = "false", tag = "1")]
                    pub play_schedule: ::prost::alloc::vec::Vec<bool>,
                    #[prost(string, optional, tag = "2")]
                    pub timezone: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameLevelInfo {
                    #[prost(bool, optional, tag = "4")]
                    pub is_trial: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "5")]
                    pub is_lifetime: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "6")]
                    pub is_restricted: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "7")]
                    pub is_beta: ::core::option::Option<bool>,
                    #[prost(string, optional, tag = "8")]
                    pub name: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(fixed32, optional, tag = "9")]
                    pub program: ::core::option::Option<u32>,
                    #[prost(message, repeated, tag = "10")]
                    pub licenses: ::prost::alloc::vec::Vec<AccountLicense>,
                    #[prost(uint32, optional, tag = "11")]
                    pub realm_permissions: ::core::option::Option<u32>,
                    #[prost(uint64, optional, tag = "12")]
                    pub last_logout_time_ms: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameTimeInfo {
                    #[prost(bool, optional, tag = "3")]
                    pub is_unlimited_play_time: ::core::option::Option<bool>,
                    #[prost(uint64, optional, tag = "5")]
                    pub play_time_expires: ::core::option::Option<u64>,
                    #[prost(bool, optional, tag = "6")]
                    pub is_subscription: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "7")]
                    pub is_recurring_subscription: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameTimeRemainingInfo {
                    #[prost(uint32, optional, tag = "1")]
                    pub minutes_remaining: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "2")]
                    pub parental_daily_minutes_remaining: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "3")]
                    pub parental_weekly_minutes_remaining: ::core::option::Option<u32>,
                    #[deprecated]
                    #[prost(uint32, optional, tag = "4")]
                    pub seconds_remaining_until_kick: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameStatus {
                    #[prost(bool, optional, tag = "4")]
                    pub is_suspended: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "5")]
                    pub is_banned: ::core::option::Option<bool>,
                    #[prost(uint64, optional, tag = "6")]
                    pub suspension_expires: ::core::option::Option<u64>,
                    #[prost(fixed32, optional, tag = "7")]
                    pub program: ::core::option::Option<u32>,
                    #[prost(bool, optional, tag = "8")]
                    pub is_locked: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "9")]
                    pub is_bam_unlockable: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct RafInfo {
                    #[prost(bytes = "vec", optional, tag = "1")]
                    pub raf_info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameSessionInfo {
                    #[deprecated]
                    #[prost(uint32, optional, tag = "3")]
                    pub start_time: ::core::option::Option<u32>,
                    #[prost(message, optional, tag = "4")]
                    pub location: ::core::option::Option<GameSessionLocation>,
                    #[prost(bool, optional, tag = "5")]
                    pub has_benefactor: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "6")]
                    pub is_using_igr: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "7")]
                    pub parental_controls_active: ::core::option::Option<bool>,
                    #[prost(uint64, optional, tag = "8")]
                    pub start_time_sec: ::core::option::Option<u64>,
                    #[prost(message, optional, tag = "9")]
                    pub igr_id: ::core::option::Option<IgrId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameSessionUpdateInfo {
                    #[prost(message, optional, tag = "8")]
                    pub cais: ::core::option::Option<Cais>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameSessionLocation {
                    #[prost(string, optional, tag = "1")]
                    pub ip_address: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(uint32, optional, tag = "2")]
                    pub country: ::core::option::Option<u32>,
                    #[prost(string, optional, tag = "3")]
                    pub city: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct Cais {
                    #[prost(uint32, optional, tag = "1")]
                    pub played_minutes: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "2")]
                    pub rested_minutes: ::core::option::Option<u32>,
                    #[prost(uint64, optional, tag = "3")]
                    pub last_heard_time: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountList {
                    #[prost(uint32, optional, tag = "3")]
                    pub region: ::core::option::Option<u32>,
                    #[prost(message, repeated, tag = "4")]
                    pub handle: ::prost::alloc::vec::Vec<GameAccountHandle>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SecurityStatus {
                    #[prost(bool, optional, tag = "1")]
                    pub sms_protect_enabled: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "2")]
                    pub email_verified: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "3")]
                    pub authenticator_enabled: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "4")]
                    pub sqa_enabled: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "5")]
                    pub authenticator_required: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AccountState {
                    #[prost(message, optional, tag = "1")]
                    pub account_level_info: ::core::option::Option<AccountLevelInfo>,
                    #[prost(message, optional, tag = "2")]
                    pub privacy_info: ::core::option::Option<PrivacyInfo>,
                    #[prost(message, optional, tag = "3")]
                    pub parental_control_info: ::core::option::Option<ParentalControlInfo>,
                    #[prost(message, repeated, tag = "5")]
                    pub game_level_info: ::prost::alloc::vec::Vec<GameLevelInfo>,
                    #[prost(message, repeated, tag = "6")]
                    pub game_status: ::prost::alloc::vec::Vec<GameStatus>,
                    #[prost(message, repeated, tag = "7")]
                    pub game_accounts: ::prost::alloc::vec::Vec<GameAccountList>,
                    #[prost(message, optional, tag = "8")]
                    pub security_status: ::core::option::Option<SecurityStatus>,
                    #[prost(message, optional, tag = "9")]
                    pub government_curfew: ::core::option::Option<PlayScheduleRestriction>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AccountStateTagged {
                    #[prost(message, optional, tag = "1")]
                    pub account_state: ::core::option::Option<AccountState>,
                    #[prost(message, optional, tag = "2")]
                    pub account_tags: ::core::option::Option<AccountFieldTags>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountState {
                    #[prost(message, optional, tag = "1")]
                    pub game_level_info: ::core::option::Option<GameLevelInfo>,
                    #[prost(message, optional, tag = "2")]
                    pub game_time_info: ::core::option::Option<GameTimeInfo>,
                    #[prost(message, optional, tag = "3")]
                    pub game_status: ::core::option::Option<GameStatus>,
                    #[prost(message, optional, tag = "4")]
                    pub raf_info: ::core::option::Option<RafInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountStateTagged {
                    #[prost(message, optional, tag = "1")]
                    pub game_account_state: ::core::option::Option<GameAccountState>,
                    #[prost(message, optional, tag = "2")]
                    pub game_account_tags: ::core::option::Option<GameAccountFieldTags>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AuthorizedData {
                    #[prost(string, optional, tag = "1")]
                    pub data: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(uint32, repeated, packed = "false", tag = "2")]
                    pub license: ::prost::alloc::vec::Vec<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct IgrId {
                    #[prost(oneof = "igr_id::Type", tags = "1, 2, 3")]
                    pub r#type: ::core::option::Option<igr_id::Type>,
                }
                #[doc = " Nested message and enum types in `IgrId`."]
                pub mod igr_id {
                    #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Oneof)]
                    pub enum Type {
                        #[prost(message, tag = "1")]
                        GameAccount(super::GameAccountHandle),
                        #[prost(fixed32, tag = "2")]
                        ExternalId(u32),
                        #[prost(string, tag = "3")]
                        Uuid(::prost::alloc::string::String),
                    }
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct IgrAddress {
                    #[prost(string, optional, tag = "1")]
                    pub client_address: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(uint32, optional, tag = "2")]
                    pub region: ::core::option::Option<u32>,
                }
                #[derive(
                    serde :: Serialize,
                    Clone,
                    Copy,
                    Debug,
                    PartialEq,
                    Eq,
                    Hash,
                    PartialOrd,
                    Ord,
                    :: prost :: Enumeration,
                )]
                #[repr(i32)]
                pub enum IdentityVerificationStatus {
                    IdentNoData = 0,
                    IdentPending = 1,
                    IdentOver18 = 2,
                    IdentUnder18 = 3,
                    IdentFailed = 4,
                    IdentSuccess = 5,
                    IdentSuccMnl = 6,
                    IdentUnknown = 7,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ResolveAccountRequest {
                    #[prost(message, optional, tag = "1")]
                    pub r#ref: ::core::option::Option<AccountReference>,
                    #[prost(bool, optional, tag = "12")]
                    pub fetch_id: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ResolveAccountResponse {
                    #[prost(message, optional, tag = "12")]
                    pub id: ::core::option::Option<AccountId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountFlagUpdateRequest {
                    #[prost(message, optional, tag = "1")]
                    pub game_account: ::core::option::Option<GameAccountHandle>,
                    #[prost(uint64, optional, tag = "2")]
                    pub flag: ::core::option::Option<u64>,
                    #[prost(bool, optional, tag = "3")]
                    pub active: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SubscriptionUpdateRequest {
                    #[prost(message, repeated, tag = "2")]
                    pub r#ref: ::prost::alloc::vec::Vec<SubscriberReference>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SubscriptionUpdateResponse {
                    #[prost(message, repeated, tag = "1")]
                    pub r#ref: ::prost::alloc::vec::Vec<SubscriberReference>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetAccountStateRequest {
                    #[prost(message, optional, tag = "1")]
                    pub entity_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(uint32, optional, tag = "2")]
                    pub program: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "3")]
                    pub region: ::core::option::Option<u32>,
                    #[prost(message, optional, tag = "10")]
                    pub options: ::core::option::Option<AccountFieldOptions>,
                    #[prost(message, optional, tag = "11")]
                    pub tags: ::core::option::Option<AccountFieldTags>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetAccountStateResponse {
                    #[prost(message, optional, tag = "1")]
                    pub state: ::core::option::Option<AccountState>,
                    #[prost(message, optional, tag = "2")]
                    pub tags: ::core::option::Option<AccountFieldTags>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetSignedAccountStateRequest {
                    #[prost(message, optional, tag = "1")]
                    pub account: ::core::option::Option<AccountId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetSignedAccountStateResponse {
                    #[prost(string, optional, tag = "1")]
                    pub token: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetGameAccountStateRequest {
                    #[deprecated]
                    #[prost(message, optional, tag = "1")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, optional, tag = "2")]
                    pub game_account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, optional, tag = "10")]
                    pub options: ::core::option::Option<GameAccountFieldOptions>,
                    #[prost(message, optional, tag = "11")]
                    pub tags: ::core::option::Option<GameAccountFieldTags>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetGameAccountStateResponse {
                    #[prost(message, optional, tag = "1")]
                    pub state: ::core::option::Option<GameAccountState>,
                    #[prost(message, optional, tag = "2")]
                    pub tags: ::core::option::Option<GameAccountFieldTags>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetLicensesRequest {
                    #[prost(message, optional, tag = "1")]
                    pub target_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(bool, optional, tag = "2")]
                    pub fetch_account_licenses: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "3")]
                    pub fetch_game_account_licenses: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "4")]
                    pub fetch_dynamic_account_licenses: ::core::option::Option<bool>,
                    #[prost(fixed32, optional, tag = "5")]
                    pub program: ::core::option::Option<u32>,
                    #[prost(bool, optional, tag = "6", default = "false")]
                    pub exclude_unknown_program: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetLicensesResponse {
                    #[prost(message, repeated, tag = "1")]
                    pub licenses: ::prost::alloc::vec::Vec<AccountLicense>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetGameSessionInfoRequest {
                    #[prost(message, optional, tag = "1")]
                    pub entity_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetGameSessionInfoResponse {
                    #[prost(message, optional, tag = "2")]
                    pub session_info: ::core::option::Option<GameSessionInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetGameTimeRemainingInfoRequest {
                    #[prost(message, optional, tag = "1")]
                    pub game_account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, optional, tag = "2")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(string, optional, tag = "3")]
                    pub benefactor_id: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetGameTimeRemainingInfoResponse {
                    #[prost(message, optional, tag = "1")]
                    pub game_time_remaining_info: ::core::option::Option<GameTimeRemainingInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetCaisInfoRequest {
                    #[prost(message, optional, tag = "1")]
                    pub entity_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetCaisInfoResponse {
                    #[prost(message, optional, tag = "1")]
                    pub cais_info: ::core::option::Option<Cais>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetAuthorizedDataRequest {
                    #[prost(message, optional, tag = "1")]
                    pub entity_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(string, repeated, tag = "2")]
                    pub tag: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                    #[prost(bool, optional, tag = "3")]
                    pub privileged_network: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetAuthorizedDataResponse {
                    #[prost(message, repeated, tag = "1")]
                    pub data: ::prost::alloc::vec::Vec<AuthorizedData>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AccountStateNotification {
                    #[prost(message, optional, tag = "1")]
                    pub account_state: ::core::option::Option<AccountState>,
                    #[deprecated]
                    #[prost(uint64, optional, tag = "2")]
                    pub subscriber_id: ::core::option::Option<u64>,
                    #[prost(message, optional, tag = "3")]
                    pub account_tags: ::core::option::Option<AccountFieldTags>,
                    #[prost(bool, optional, tag = "4")]
                    pub subscription_completed: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountStateNotification {
                    #[prost(message, optional, tag = "1")]
                    pub game_account_state: ::core::option::Option<GameAccountState>,
                    #[deprecated]
                    #[prost(uint64, optional, tag = "2")]
                    pub subscriber_id: ::core::option::Option<u64>,
                    #[prost(message, optional, tag = "3")]
                    pub game_account_tags: ::core::option::Option<GameAccountFieldTags>,
                    #[prost(bool, optional, tag = "4")]
                    pub subscription_completed: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountNotification {
                    #[prost(message, repeated, tag = "1")]
                    pub game_accounts: ::prost::alloc::vec::Vec<GameAccountList>,
                    #[prost(uint64, optional, tag = "2")]
                    pub subscriber_id: ::core::option::Option<u64>,
                    #[prost(message, optional, tag = "3")]
                    pub account_tags: ::core::option::Option<AccountFieldTags>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountSessionNotification {
                    #[prost(message, optional, tag = "1")]
                    pub game_account: ::core::option::Option<GameAccountHandle>,
                    #[prost(message, optional, tag = "2")]
                    pub session_info: ::core::option::Option<GameSessionUpdateInfo>,
                }
                #[async_trait::async_trait]
                pub trait AccountService: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 508412975u32;
                    const ORIGINAL_HASH: u32 = 1658456209u32;
                    const OUTBOUND: bool = true;
                    const DESCRIPTOR_NAME: &'static str = "bnet.protocol.account.AccountService";
                    const SHARD_NAME: &'static str = "account";
                    const RESOLVE_ACCOUNT: u8 = 13u8;
                    const SUBSCRIBE: u8 = 25u8;
                    const UNSUBSCRIBE: u8 = 26u8;
                    const GET_ACCOUNT_STATE: u8 = 30u8;
                    const GET_GAME_ACCOUNT_STATE: u8 = 31u8;
                    const GET_LICENSES: u8 = 32u8;
                    const GET_GAME_TIME_REMAINING_INFO: u8 = 33u8;
                    const GET_GAME_SESSION_INFO: u8 = 34u8;
                    const GET_CAIS_INFO: u8 = 35u8;
                    const GET_AUTHORIZED_DATA: u8 = 37u8;
                    const GET_SIGNED_ACCOUNT_STATE: u8 = 44u8;
                    async fn resolve_account(
                        &mut self,
                        _: ResolveAccountRequest,
                    ) -> Result<ResolveAccountResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn subscribe(
                        &mut self,
                        _: SubscriptionUpdateRequest,
                    ) -> Result<SubscriptionUpdateResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn unsubscribe(
                        &mut self,
                        _: SubscriptionUpdateRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn get_account_state(
                        &mut self,
                        _: GetAccountStateRequest,
                    ) -> Result<GetAccountStateResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn get_game_account_state(
                        &mut self,
                        _: GetGameAccountStateRequest,
                    ) -> Result<GetGameAccountStateResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn get_licenses(
                        &mut self,
                        _: GetLicensesRequest,
                    ) -> Result<GetLicensesResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn get_game_time_remaining_info(
                        &mut self,
                        _: GetGameTimeRemainingInfoRequest,
                    ) -> Result<GetGameTimeRemainingInfoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn get_game_session_info(
                        &mut self,
                        _: GetGameSessionInfoRequest,
                    ) -> Result<GetGameSessionInfoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn get_cais_info(
                        &mut self,
                        _: GetCaisInfoRequest,
                    ) -> Result<GetCaisInfoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn get_authorized_data(
                        &mut self,
                        _: GetAuthorizedDataRequest,
                    ) -> Result<GetAuthorizedDataResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn get_signed_account_state(
                        &mut self,
                        _: GetSignedAccountStateRequest,
                    ) -> Result<GetSignedAccountStateResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            13u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ResolveAccountRequest>::default()
                                } else {
                                    <ResolveAccountRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(resolve_account),
                                    &parsed
                                );
                                let response = self.resolve_account(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(13u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(resolve_account),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            25u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <SubscriptionUpdateRequest>::default()
                                } else {
                                    <SubscriptionUpdateRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(subscribe),
                                    &parsed
                                );
                                let response = self.subscribe(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(25u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(subscribe),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            26u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <SubscriptionUpdateRequest>::default()
                                } else {
                                    <SubscriptionUpdateRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unsubscribe),
                                    &parsed
                                );
                                let response = self.unsubscribe(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(26u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unsubscribe),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            30u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GetAccountStateRequest>::default()
                                } else {
                                    <GetAccountStateRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_account_state),
                                    &parsed
                                );
                                let response = self.get_account_state(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(30u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_account_state),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            31u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GetGameAccountStateRequest>::default()
                                } else {
                                    <GetGameAccountStateRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_game_account_state),
                                    &parsed
                                );
                                let response = self.get_game_account_state(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(31u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_game_account_state),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            32u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GetLicensesRequest>::default()
                                } else {
                                    <GetLicensesRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_licenses),
                                    &parsed
                                );
                                let response = self.get_licenses(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(32u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_licenses),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            33u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GetGameTimeRemainingInfoRequest>::default()
                                } else {
                                    <GetGameTimeRemainingInfoRequest>::decode(
                                        &mut msg.data.clone(),
                                    )?
                                };
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_game_time_remaining_info),
                                    &parsed
                                );
                                let response = self.get_game_time_remaining_info(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(33u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_game_time_remaining_info),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            34u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GetGameSessionInfoRequest>::default()
                                } else {
                                    <GetGameSessionInfoRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_game_session_info),
                                    &parsed
                                );
                                let response = self.get_game_session_info(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(34u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_game_session_info),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            35u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GetCaisInfoRequest>::default()
                                } else {
                                    <GetCaisInfoRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_cais_info),
                                    &parsed
                                );
                                let response = self.get_cais_info(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(35u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_cais_info),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            37u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GetAuthorizedDataRequest>::default()
                                } else {
                                    <GetAuthorizedDataRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_authorized_data),
                                    &parsed
                                );
                                let response = self.get_authorized_data(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(37u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_authorized_data),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            44u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GetSignedAccountStateRequest>::default()
                                } else {
                                    <GetSignedAccountStateRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_signed_account_state),
                                    &parsed
                                );
                                let response = self.get_signed_account_state(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(44u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_signed_account_state),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
                #[async_trait::async_trait]
                pub trait AccountListener: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 2013743164u32;
                    const ORIGINAL_HASH: u32 = 1423956503u32;
                    const DESCRIPTOR_NAME: &'static str = "bnet.protocol.account.AccountNotify";
                    const INBOUND: bool = true;
                    const ON_ACCOUNT_STATE_UPDATED: u8 = 1u8;
                    const ON_GAME_ACCOUNT_STATE_UPDATED: u8 = 2u8;
                    const ON_GAME_ACCOUNTS_UPDATED: u8 = 3u8;
                    const ON_GAME_SESSION_UPDATED: u8 = 4u8;
                    async fn on_account_state_updated(
                        &mut self,
                        _: AccountStateNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_game_account_state_updated(
                        &mut self,
                        _: GameAccountStateNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_game_accounts_updated(
                        &mut self,
                        _: GameAccountNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_game_session_updated(
                        &mut self,
                        _: GameAccountSessionNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <AccountStateNotification>::default()
                                } else {
                                    <AccountStateNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_account_state_updated),
                                    &parsed
                                );
                                let response = self.on_account_state_updated(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_account_state_updated),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            2u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GameAccountStateNotification>::default()
                                } else {
                                    <GameAccountStateNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_account_state_updated),
                                    &parsed
                                );
                                let response = self.on_game_account_state_updated(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(2u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_account_state_updated),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            3u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GameAccountNotification>::default()
                                } else {
                                    <GameAccountNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_accounts_updated),
                                    &parsed
                                );
                                let response = self.on_game_accounts_updated(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(3u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_accounts_updated),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            4u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GameAccountSessionNotification>::default()
                                } else {
                                    <GameAccountSessionNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AccountListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_session_updated),
                                    &parsed
                                );
                                let response = self.on_game_session_updated(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(4u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AccountListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_session_updated),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
            }
        }
        pub mod channel {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ChannelId {
                    #[prost(uint32, optional, tag = "1")]
                    pub r#type: ::core::option::Option<u32>,
                    #[prost(message, optional, tag = "2")]
                    pub host: ::core::option::Option<super::super::ProcessId>,
                    #[prost(fixed32, optional, tag = "3")]
                    pub id: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct Message {
                    #[prost(message, repeated, tag = "1")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ListChannelsOptions {
                    #[prost(uint32, optional, tag = "1", default = "0")]
                    pub start_index: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "2", default = "16")]
                    pub max_results: ::core::option::Option<u32>,
                    #[prost(string, optional, tag = "3")]
                    pub name: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(fixed32, optional, tag = "4")]
                    pub program: ::core::option::Option<u32>,
                    #[prost(fixed32, optional, tag = "5")]
                    pub locale: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "6")]
                    pub capacity_full: ::core::option::Option<u32>,
                    #[prost(message, required, tag = "7")]
                    pub attribute_filter: super::super::AttributeFilter,
                    #[prost(string, optional, tag = "8")]
                    pub channel_type: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ChannelDescription {
                    #[prost(message, required, tag = "1")]
                    pub channel_id: super::super::EntityId,
                    #[prost(uint32, optional, tag = "2")]
                    pub current_members: ::core::option::Option<u32>,
                    #[prost(message, optional, tag = "3")]
                    pub state: ::core::option::Option<ChannelState>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ChannelInfo {
                    #[prost(message, required, tag = "1")]
                    pub description: ChannelDescription,
                    #[prost(message, repeated, tag = "2")]
                    pub member: ::prost::alloc::vec::Vec<Member>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ChannelState {
                    #[prost(uint32, optional, tag = "1")]
                    pub max_members: ::core::option::Option<u32>,
                    #[prost(uint32, optional, tag = "2")]
                    pub min_members: ::core::option::Option<u32>,
                    #[prost(message, repeated, tag = "3")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                    #[prost(message, repeated, tag = "4")]
                    pub invitation: ::prost::alloc::vec::Vec<super::super::Invitation>,
                    #[prost(uint32, optional, tag = "6")]
                    pub reason: ::core::option::Option<u32>,
                    #[prost(
                        enumeration = "channel_state::PrivacyLevel",
                        optional,
                        tag = "7",
                        default = "Open"
                    )]
                    pub privacy_level: ::core::option::Option<i32>,
                    #[prost(string, optional, tag = "8")]
                    pub name: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "10", default = "default")]
                    pub channel_type: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(fixed32, optional, tag = "11")]
                    pub program: ::core::option::Option<u32>,
                    #[prost(bool, optional, tag = "13", default = "true")]
                    pub subscribe_to_presence: ::core::option::Option<bool>,
                }
                #[doc = " Nested message and enum types in `ChannelState`."]
                pub mod channel_state {
                    #[derive(
                        serde :: Serialize,
                        Clone,
                        Copy,
                        Debug,
                        PartialEq,
                        Eq,
                        Hash,
                        PartialOrd,
                        Ord,
                        :: prost :: Enumeration,
                    )]
                    #[repr(i32)]
                    pub enum PrivacyLevel {
                        Open = 1,
                        OpenInvitationAndFriend = 2,
                        OpenInvitation = 3,
                        Closed = 4,
                    }
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct MemberAccountInfo {
                    #[prost(string, optional, tag = "3")]
                    pub battle_tag: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct MemberState {
                    #[prost(message, repeated, tag = "1")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                    #[prost(uint32, repeated, tag = "2")]
                    pub role: ::prost::alloc::vec::Vec<u32>,
                    #[prost(uint64, optional, tag = "3", default = "0")]
                    pub privileges: ::core::option::Option<u64>,
                    #[prost(message, optional, tag = "4")]
                    pub info: ::core::option::Option<MemberAccountInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct Member {
                    #[prost(message, required, tag = "1")]
                    pub identity: super::super::Identity,
                    #[prost(message, required, tag = "2")]
                    pub state: MemberState,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SubscriberId {
                    #[deprecated]
                    #[prost(message, optional, tag = "1")]
                    pub account: ::core::option::Option<super::super::account::v1::AccountId>,
                    #[prost(message, optional, tag = "2")]
                    pub game_account:
                        ::core::option::Option<super::super::account::v1::GameAccountHandle>,
                    #[prost(message, optional, tag = "3")]
                    pub process: ::core::option::Option<super::super::ProcessId>,
                }
            }
        }
        pub mod authentication {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ModuleLoadRequest {
                    #[prost(message, required, tag = "1")]
                    pub module_handle: super::super::ContentHandle,
                    #[prost(bytes = "vec", optional, tag = "2")]
                    pub message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ModuleNotification {
                    #[prost(int32, optional, tag = "2")]
                    pub module_id: ::core::option::Option<i32>,
                    #[prost(uint32, optional, tag = "3")]
                    pub result: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ModuleMessageRequest {
                    #[prost(int32, required, tag = "1")]
                    pub module_id: i32,
                    #[prost(bytes = "vec", optional, tag = "2")]
                    pub message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct LogonRequest {
                    #[prost(string, optional, tag = "1")]
                    pub program: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "2")]
                    pub platform: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "3")]
                    pub locale: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "4")]
                    pub email: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "5")]
                    pub version: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(int32, optional, tag = "6")]
                    pub application_version: ::core::option::Option<i32>,
                    #[prost(bool, optional, tag = "7")]
                    pub public_computer: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "10", default = "false")]
                    pub allow_logon_queue_notifications: ::core::option::Option<bool>,
                    #[prost(bytes = "vec", optional, tag = "12")]
                    pub cached_web_credentials:
                        ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                    #[prost(string, optional, tag = "14")]
                    pub user_agent: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "15")]
                    pub device_id: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "16")]
                    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct LogonResult {
                    #[prost(uint32, required, tag = "1")]
                    pub error_code: u32,
                    #[prost(message, optional, tag = "2")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, repeated, tag = "3")]
                    pub game_account_id: ::prost::alloc::vec::Vec<super::super::EntityId>,
                    #[prost(string, optional, tag = "4")]
                    pub email: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(uint32, repeated, packed = "false", tag = "5")]
                    pub available_region: ::prost::alloc::vec::Vec<u32>,
                    #[prost(uint32, optional, tag = "6")]
                    pub connected_region: ::core::option::Option<u32>,
                    #[prost(string, optional, tag = "7")]
                    pub battle_tag: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "8")]
                    pub geoip_country: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(bytes = "vec", optional, tag = "9")]
                    pub session_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                    #[prost(bool, optional, tag = "10")]
                    pub restricted_mode: ::core::option::Option<bool>,
                    #[prost(string, optional, tag = "11")]
                    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GenerateSsoTokenRequest {
                    #[prost(fixed32, optional, tag = "1")]
                    pub program: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GenerateSsoTokenResponse {
                    #[prost(bytes = "vec", optional, tag = "1")]
                    pub sso_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                    #[prost(bytes = "vec", optional, tag = "2")]
                    pub sso_secret: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct LogonUpdateRequest {
                    #[prost(uint32, required, tag = "1")]
                    pub error_code: u32,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct LogonQueueUpdateRequest {
                    #[prost(uint32, required, tag = "1")]
                    pub position: u32,
                    #[prost(uint64, required, tag = "2")]
                    pub estimated_time: u64,
                    #[prost(uint64, required, tag = "3")]
                    pub eta_deviation_in_sec: u64,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AccountSettingsNotification {
                    #[prost(message, repeated, tag = "1")]
                    pub licenses:
                        ::prost::alloc::vec::Vec<super::super::account::v1::AccountLicense>,
                    #[prost(bool, optional, tag = "2")]
                    pub is_using_rid: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "3")]
                    pub is_playing_from_igr: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "4")]
                    pub can_receive_voice: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "5")]
                    pub can_send_voice: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ServerStateChangeRequest {
                    #[prost(uint32, required, tag = "1")]
                    pub state: u32,
                    #[prost(uint64, required, tag = "2")]
                    pub event_time: u64,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct VersionInfo {
                    #[prost(uint32, optional, tag = "1")]
                    pub number: ::core::option::Option<u32>,
                    #[prost(string, optional, tag = "2")]
                    pub patch: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(bool, optional, tag = "3")]
                    pub is_optional: ::core::option::Option<bool>,
                    #[prost(uint64, optional, tag = "4")]
                    pub kick_time: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct VersionInfoNotification {
                    #[prost(message, optional, tag = "1")]
                    pub version_info: ::core::option::Option<VersionInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct MemModuleLoadRequest {
                    #[prost(message, required, tag = "1")]
                    pub handle: super::super::ContentHandle,
                    #[prost(bytes = "vec", required, tag = "2")]
                    pub key: ::prost::alloc::vec::Vec<u8>,
                    #[prost(bytes = "vec", required, tag = "3")]
                    pub input: ::prost::alloc::vec::Vec<u8>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct MemModuleLoadResponse {
                    #[prost(bytes = "vec", required, tag = "1")]
                    pub data: ::prost::alloc::vec::Vec<u8>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SelectGameAccountRequest {
                    #[prost(message, required, tag = "1")]
                    pub game_account_id: super::super::EntityId,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountSelectedRequest {
                    #[prost(uint32, required, tag = "1")]
                    pub result: u32,
                    #[prost(message, optional, tag = "2")]
                    pub game_account_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GenerateWebCredentialsRequest {
                    #[prost(fixed32, optional, tag = "1")]
                    pub program: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GenerateWebCredentialsResponse {
                    #[prost(bytes = "vec", optional, tag = "1")]
                    pub web_credentials: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct VerifyWebCredentialsRequest {
                    #[prost(bytes = "vec", optional, tag = "1")]
                    pub web_credentials: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                }
                #[async_trait::async_trait]
                pub trait AuthenticationListener: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 1302880808u32;
                    const ORIGINAL_HASH: u32 = 1898188341u32;
                    const INBOUND: bool = true;
                    const DESCRIPTOR_NAME: &'static str =
                        "bnet.protocol.authentication.AuthenticationClient";
                    const ON_MODULE_LOAD: u8 = 1u8;
                    const ON_MODULE_MESSAGE: u8 = 2u8;
                    const ON_SERVER_STATE_CHANGE: u8 = 4u8;
                    const ON_LOGON_COMPLETE: u8 = 5u8;
                    const ON_MEM_MODULE_LOAD: u8 = 6u8;
                    const ON_LOGON_UPDATE: u8 = 10u8;
                    const ON_VERSION_INFO_UPDATED: u8 = 11u8;
                    const ON_LOGON_QUEUE_UPDATE: u8 = 12u8;
                    const ON_LOGON_QUEUE_END: u8 = 13u8;
                    const ON_GAME_ACCOUNT_SELECTED: u8 = 14u8;
                    async fn on_module_load(
                        &mut self,
                        _: ModuleLoadRequest,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_module_message(
                        &mut self,
                        _: ModuleMessageRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_server_state_change(
                        &mut self,
                        _: ServerStateChangeRequest,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_logon_complete(
                        &mut self,
                        _: LogonResult,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_mem_module_load(
                        &mut self,
                        _: MemModuleLoadRequest,
                    ) -> Result<MemModuleLoadResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_logon_update(
                        &mut self,
                        _: LogonUpdateRequest,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_version_info_updated(
                        &mut self,
                        _: VersionInfoNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_logon_queue_update(
                        &mut self,
                        _: LogonQueueUpdateRequest,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_logon_queue_end(
                        &mut self,
                        _: super::super::NoData,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_game_account_selected(
                        &mut self,
                        _: GameAccountSelectedRequest,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ModuleLoadRequest>::default()
                                } else {
                                    <ModuleLoadRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_module_load),
                                    &parsed
                                );
                                let response = self.on_module_load(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_module_load),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            2u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ModuleMessageRequest>::default()
                                } else {
                                    <ModuleMessageRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_module_message),
                                    &parsed
                                );
                                let response = self.on_module_message(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(2u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_module_message),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            4u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ServerStateChangeRequest>::default()
                                } else {
                                    <ServerStateChangeRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_server_state_change),
                                    &parsed
                                );
                                let response = self.on_server_state_change(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(4u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_server_state_change),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            5u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <LogonResult>::default()
                                } else {
                                    <LogonResult>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_logon_complete),
                                    &parsed
                                );
                                let response = self.on_logon_complete(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(5u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_logon_complete),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            6u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <MemModuleLoadRequest>::default()
                                } else {
                                    <MemModuleLoadRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_mem_module_load),
                                    &parsed
                                );
                                let response = self.on_mem_module_load(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(6u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_mem_module_load),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            10u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <LogonUpdateRequest>::default()
                                } else {
                                    <LogonUpdateRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_logon_update),
                                    &parsed
                                );
                                let response = self.on_logon_update(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(10u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_logon_update),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            11u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <VersionInfoNotification>::default()
                                } else {
                                    <VersionInfoNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_version_info_updated),
                                    &parsed
                                );
                                let response = self.on_version_info_updated(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(11u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_version_info_updated),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            12u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <LogonQueueUpdateRequest>::default()
                                } else {
                                    <LogonQueueUpdateRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_logon_queue_update),
                                    &parsed
                                );
                                let response = self.on_logon_queue_update(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(12u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_logon_queue_update),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            13u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <super::super::NoData>::default()
                                } else {
                                    <super::super::NoData>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_logon_queue_end),
                                    &parsed
                                );
                                let response = self.on_logon_queue_end(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(13u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_logon_queue_end),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            14u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GameAccountSelectedRequest>::default()
                                } else {
                                    <GameAccountSelectedRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_account_selected),
                                    &parsed
                                );
                                let response = self.on_game_account_selected(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(14u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_account_selected),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
                #[async_trait::async_trait]
                pub trait AuthenticationService: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 4284115651u32;
                    const ORIGINAL_HASH: u32 = 233634817u32;
                    const SHARD_NAME: &'static str = "authentication";
                    const OUTBOUND: bool = true;
                    const DESCRIPTOR_NAME: &'static str =
                        "bnet.protocol.authentication.AuthenticationServer";
                    const LOGON: u8 = 1u8;
                    const MODULE_NOTIFY: u8 = 2u8;
                    const MODULE_MESSAGE: u8 = 3u8;
                    const SELECT_GAME_ACCOUNT_DEPRECATED: u8 = 4u8;
                    const GENERATE_SSO_TOKEN: u8 = 5u8;
                    const SELECT_GAME_ACCOUNT: u8 = 6u8;
                    const VERIFY_WEB_CREDENTIALS: u8 = 7u8;
                    const GENERATE_WEB_CREDENTIALS: u8 = 8u8;
                    async fn logon(
                        &mut self,
                        _: LogonRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn module_notify(
                        &mut self,
                        _: ModuleNotification,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn module_message(
                        &mut self,
                        _: ModuleMessageRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn select_game_account_deprecated(
                        &mut self,
                        _: super::super::EntityId,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn generate_sso_token(
                        &mut self,
                        _: GenerateSsoTokenRequest,
                    ) -> Result<GenerateSsoTokenResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn select_game_account(
                        &mut self,
                        _: SelectGameAccountRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn verify_web_credentials(
                        &mut self,
                        _: VerifyWebCredentialsRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn generate_web_credentials(
                        &mut self,
                        _: GenerateWebCredentialsRequest,
                    ) -> Result<GenerateWebCredentialsResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <LogonRequest>::default()
                                } else {
                                    <LogonRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(logon),
                                    &parsed
                                );
                                let response = self.logon(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(logon),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            2u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ModuleNotification>::default()
                                } else {
                                    <ModuleNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(module_notify),
                                    &parsed
                                );
                                let response = self.module_notify(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(2u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(module_notify),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            3u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ModuleMessageRequest>::default()
                                } else {
                                    <ModuleMessageRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(module_message),
                                    &parsed
                                );
                                let response = self.module_message(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(3u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(module_message),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            4u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <super::super::EntityId>::default()
                                } else {
                                    <super::super::EntityId>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(select_game_account_deprecated),
                                    &parsed
                                );
                                let response = self.select_game_account_deprecated(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(4u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(select_game_account_deprecated),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            5u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GenerateSsoTokenRequest>::default()
                                } else {
                                    <GenerateSsoTokenRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(generate_sso_token),
                                    &parsed
                                );
                                let response = self.generate_sso_token(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(5u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(generate_sso_token),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            6u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <SelectGameAccountRequest>::default()
                                } else {
                                    <SelectGameAccountRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(select_game_account),
                                    &parsed
                                );
                                let response = self.select_game_account(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(6u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(select_game_account),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            7u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <VerifyWebCredentialsRequest>::default()
                                } else {
                                    <VerifyWebCredentialsRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(verify_web_credentials),
                                    &parsed
                                );
                                let response = self.verify_web_credentials(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(7u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(verify_web_credentials),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            8u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GenerateWebCredentialsRequest>::default()
                                } else {
                                    <GenerateWebCredentialsRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(generate_web_credentials),
                                    &parsed
                                );
                                let response = self.generate_web_credentials(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(8u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(AuthenticationService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(generate_web_credentials),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
            }
        }
        pub mod challenge {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ChallengeExternalRequest {
                    #[prost(string, optional, tag = "1")]
                    pub request_token: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "2")]
                    pub payload_type: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(bytes = "vec", optional, tag = "3")]
                    pub payload: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ChallengeExternalResult {
                    #[prost(string, optional, tag = "1")]
                    pub request_token: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(bool, optional, tag = "2", default = "true")]
                    pub passed: ::core::option::Option<bool>,
                }
                #[async_trait::async_trait]
                pub trait ChallengeListener: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 3336112824u32;
                    const ORIGINAL_HASH: u32 = 3151632159u32;
                    const DESCRIPTOR_NAME: &'static str = "bnet.protocol.challenge.ChallengeNotify";
                    const INBOUND: bool = true;
                    const ON_EXTERNAL_CHALLENGE: u8 = 3u8;
                    const ON_EXTERNAL_CHALLENGE_RESULT: u8 = 4u8;
                    async fn on_external_challenge(
                        &mut self,
                        _: ChallengeExternalRequest,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_external_challenge_result(
                        &mut self,
                        _: ChallengeExternalResult,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            3u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ChallengeExternalRequest>::default()
                                } else {
                                    <ChallengeExternalRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(ChallengeListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_external_challenge),
                                    &parsed
                                );
                                let response = self.on_external_challenge(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(3u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(ChallengeListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_external_challenge),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            4u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ChallengeExternalResult>::default()
                                } else {
                                    <ChallengeExternalResult>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(ChallengeListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_external_challenge_result),
                                    &parsed
                                );
                                let response = self.on_external_challenge_result(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(4u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(ChallengeListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_external_challenge_result),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
            }
        }
        pub mod club {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct MemberId {
                    #[prost(message, optional, tag = "1")]
                    pub account_id: ::core::option::Option<super::super::account::v1::AccountId>,
                    #[prost(uint64, optional, tag = "2")]
                    pub unique_id: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClubTypeRangeSet {
                    #[prost(message, optional, tag = "2")]
                    pub name_range: ::core::option::Option<super::super::UnsignedIntRange>,
                    #[prost(message, optional, tag = "3")]
                    pub description_range: ::core::option::Option<super::super::UnsignedIntRange>,
                    #[prost(message, optional, tag = "4")]
                    pub broadcast_range: ::core::option::Option<super::super::UnsignedIntRange>,
                    #[prost(message, optional, tag = "7")]
                    pub short_name_range: ::core::option::Option<super::super::UnsignedIntRange>,
                    #[prost(message, optional, tag = "25")]
                    pub member: ::core::option::Option<ClubMemberRangeSet>,
                    #[prost(message, optional, tag = "26")]
                    pub stream: ::core::option::Option<ClubStreamRangeSet>,
                    #[prost(message, optional, tag = "27")]
                    pub invitation: ::core::option::Option<ClubInvitationRangeSet>,
                    #[prost(message, optional, tag = "28")]
                    pub suggestion: ::core::option::Option<ClubSuggestionRangeSet>,
                    #[prost(message, optional, tag = "29")]
                    pub ticket: ::core::option::Option<ClubTicketRangeSet>,
                    #[prost(message, optional, tag = "30")]
                    pub ban: ::core::option::Option<ClubBanRangeSet>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClubMemberRangeSet {
                    #[prost(message, optional, tag = "1")]
                    pub count: ::core::option::Option<super::super::UnsignedIntRange>,
                    #[prost(message, optional, tag = "3")]
                    pub voice: ::core::option::Option<super::super::UnsignedIntRange>,
                    #[prost(message, optional, tag = "5")]
                    pub stream_subscriptions:
                        ::core::option::Option<super::super::UnsignedIntRange>,
                    #[prost(message, optional, tag = "7")]
                    pub note_range: ::core::option::Option<super::super::UnsignedIntRange>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClubStreamRangeSet {
                    #[prost(message, optional, tag = "1")]
                    pub count: ::core::option::Option<super::super::UnsignedIntRange>,
                    #[prost(message, optional, tag = "3")]
                    pub name_range: ::core::option::Option<super::super::UnsignedIntRange>,
                    #[prost(message, optional, tag = "4")]
                    pub subject_range: ::core::option::Option<super::super::UnsignedIntRange>,
                    #[prost(message, optional, tag = "5")]
                    pub message_range: ::core::option::Option<super::super::UnsignedIntRange>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClubInvitationRangeSet {
                    #[prost(message, optional, tag = "1")]
                    pub count: ::core::option::Option<super::super::UnsignedIntRange>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClubSuggestionRangeSet {
                    #[prost(message, optional, tag = "1")]
                    pub count: ::core::option::Option<super::super::UnsignedIntRange>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClubTicketRangeSet {
                    #[prost(message, optional, tag = "1")]
                    pub count: ::core::option::Option<super::super::UnsignedIntRange>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClubBanRangeSet {
                    #[prost(message, optional, tag = "1")]
                    pub count: ::core::option::Option<super::super::UnsignedIntRange>,
                    #[prost(message, optional, tag = "3")]
                    pub reason_range: ::core::option::Option<super::super::UnsignedIntRange>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClubPrivilegeSet {
                    #[prost(bool, optional, tag = "1")]
                    pub can_destroy: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "10")]
                    pub can_set_attribute: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "11")]
                    pub can_set_name: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "12")]
                    pub can_set_description: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "13")]
                    pub can_set_avatar: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "14")]
                    pub can_set_broadcast: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "15")]
                    pub can_set_privacy_level: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "29")]
                    pub can_add_member: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "30")]
                    pub can_kick_member: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "31")]
                    pub can_set_own_member_attribute: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "32")]
                    pub can_set_other_member_attribute: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "33")]
                    pub can_set_own_voice_state: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "34")]
                    pub can_set_own_presence_level: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "35")]
                    pub can_set_own_whisper_level: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "36")]
                    pub can_set_own_member_note: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "37")]
                    pub can_set_other_member_note: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "50")]
                    pub can_use_voice: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "51")]
                    pub can_voice_mute_member_for_all: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "70")]
                    pub can_get_invitation: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "71")]
                    pub can_send_invitation: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "72")]
                    pub can_send_guest_invitation: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "73")]
                    pub can_revoke_own_invitation: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "74")]
                    pub can_revoke_other_invitation: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "90")]
                    pub can_get_suggestion: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "91")]
                    pub can_suggest_member: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "92")]
                    pub can_approve_member: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "110")]
                    pub can_get_ticket: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "111")]
                    pub can_create_ticket: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "112")]
                    pub can_destroy_ticket: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "130")]
                    pub can_get_ban: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "131")]
                    pub can_add_ban: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "132")]
                    pub can_remove_ban: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "140")]
                    pub can_create_stream: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "141")]
                    pub can_destroy_stream: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "142")]
                    pub can_set_stream_position: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "143")]
                    pub can_set_stream_attribute: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "144")]
                    pub can_set_stream_name: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "145")]
                    pub can_set_stream_subject: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "146")]
                    pub can_set_stream_access: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "147")]
                    pub can_set_stream_voice_level: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "180")]
                    pub can_create_message: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "181")]
                    pub can_destroy_own_message: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "182")]
                    pub can_destroy_other_message: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "183")]
                    pub can_edit_own_message: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "184")]
                    pub can_pin_message: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "185")]
                    pub can_mention_all: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "186")]
                    pub can_mention_here: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "187")]
                    pub can_mention_member: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "188")]
                    pub can_mention_role: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClubRole {
                    #[prost(uint32, optional, tag = "1")]
                    pub id: ::core::option::Option<u32>,
                    #[prost(message, optional, tag = "2")]
                    pub state: ::core::option::Option<super::super::RoleState>,
                    #[prost(message, optional, tag = "3")]
                    pub privilege: ::core::option::Option<ClubPrivilegeSet>,
                    #[prost(bool, optional, tag = "4")]
                    pub always_grant_stream_access: ::core::option::Option<bool>,
                    #[prost(bool, optional, tag = "5")]
                    pub allow_in_club_slot: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClubRoleSet {
                    #[prost(message, repeated, tag = "1")]
                    pub role: ::prost::alloc::vec::Vec<ClubRole>,
                    #[prost(uint32, repeated, tag = "5")]
                    pub default_role: ::prost::alloc::vec::Vec<u32>,
                    #[prost(bool, optional, tag = "6")]
                    pub assignment_respects_relegation_chain: ::core::option::Option<bool>,
                    #[prost(string, optional, tag = "7")]
                    pub subtype: ::core::option::Option<::prost::alloc::string::String>,
                }
            }
        }
        pub mod connection {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ConnectRequest {
                    #[prost(message, optional, tag = "1")]
                    pub client_id: ::core::option::Option<super::super::ProcessId>,
                    #[prost(message, optional, tag = "2")]
                    pub bind_request: ::core::option::Option<BindRequest>,
                    #[prost(bool, optional, tag = "3", default = "true")]
                    pub use_bindless_rpc: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ConnectionMeteringContentHandles {
                    #[prost(message, repeated, tag = "1")]
                    pub content_handle: ::prost::alloc::vec::Vec<super::super::ContentHandle>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ConnectResponse {
                    #[prost(message, required, tag = "1")]
                    pub server_id: super::super::ProcessId,
                    #[prost(message, optional, tag = "2")]
                    pub client_id: ::core::option::Option<super::super::ProcessId>,
                    #[prost(uint32, optional, tag = "3")]
                    pub bind_result: ::core::option::Option<u32>,
                    #[prost(message, optional, tag = "4")]
                    pub bind_response: ::core::option::Option<BindResponse>,
                    #[prost(message, optional, tag = "5")]
                    pub content_handle_array:
                        ::core::option::Option<ConnectionMeteringContentHandles>,
                    #[prost(uint64, optional, tag = "6")]
                    pub server_time: ::core::option::Option<u64>,
                    #[prost(bool, optional, tag = "7", default = "false")]
                    pub use_bindless_rpc: ::core::option::Option<bool>,
                    #[prost(message, optional, tag = "8")]
                    pub binary_content_handle_array:
                        ::core::option::Option<ConnectionMeteringContentHandles>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct BoundService {
                    #[prost(fixed32, required, tag = "1")]
                    pub hash: u32,
                    #[prost(uint32, required, tag = "2")]
                    pub id: u32,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct BindRequest {
                    #[deprecated]
                    #[prost(fixed32, repeated, tag = "1")]
                    pub deprecated_imported_service_hash: ::prost::alloc::vec::Vec<u32>,
                    #[deprecated]
                    #[prost(message, repeated, tag = "2")]
                    pub deprecated_exported_service: ::prost::alloc::vec::Vec<BoundService>,
                    #[prost(message, repeated, tag = "3")]
                    pub exported_service: ::prost::alloc::vec::Vec<BoundService>,
                    #[prost(message, repeated, tag = "4")]
                    pub imported_service: ::prost::alloc::vec::Vec<BoundService>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct BindResponse {
                    #[deprecated]
                    #[prost(uint32, repeated, tag = "1")]
                    pub imported_service_id: ::prost::alloc::vec::Vec<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct EchoRequest {
                    #[prost(fixed64, optional, tag = "1")]
                    pub time: ::core::option::Option<u64>,
                    #[prost(bool, optional, tag = "2", default = "false")]
                    pub network_only: ::core::option::Option<bool>,
                    #[prost(bytes = "vec", optional, tag = "3")]
                    pub payload: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                    #[prost(message, optional, tag = "4")]
                    pub forward: ::core::option::Option<super::super::ProcessId>,
                    #[prost(string, optional, tag = "5")]
                    pub forward_client_id: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct EchoResponse {
                    #[prost(fixed64, optional, tag = "1")]
                    pub time: ::core::option::Option<u64>,
                    #[prost(bytes = "vec", optional, tag = "2")]
                    pub payload: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct DisconnectRequest {
                    #[prost(uint32, required, tag = "1")]
                    pub error_code: u32,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct DisconnectNotification {
                    #[prost(uint32, required, tag = "1")]
                    pub error_code: u32,
                    #[prost(string, optional, tag = "2")]
                    pub reason: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct EncryptRequest {}
                #[async_trait::async_trait]
                pub trait ConnectionService: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 662833483u32;
                    const ORIGINAL_HASH: u32 = 1698982289u32;
                    const INBOUND: bool = true;
                    const SHARD_NAME: &'static str = "connection";
                    const DESCRIPTOR_NAME: &'static str =
                        "bnet.protocol.connection.ConnectionService";
                    const OUTBOUND: bool = true;
                    const CONNECT: u8 = 1u8;
                    const BIND: u8 = 2u8;
                    const ECHO: u8 = 3u8;
                    const FORCE_DISCONNECT: u8 = 4u8;
                    const KEEP_ALIVE: u8 = 5u8;
                    const ENCRYPT: u8 = 6u8;
                    const REQUEST_DISCONNECT: u8 = 7u8;
                    async fn connect(
                        &mut self,
                        _: ConnectRequest,
                    ) -> Result<ConnectResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn bind(
                        &mut self,
                        _: BindRequest,
                    ) -> Result<BindResponse, crate::errors::WowRpcResponse> {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn echo(
                        &mut self,
                        _: EchoRequest,
                    ) -> Result<EchoResponse, crate::errors::WowRpcResponse> {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn force_disconnect(
                        &mut self,
                        _: DisconnectNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn keep_alive(
                        &mut self,
                        _: super::super::NoData,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn encrypt(
                        &mut self,
                        _: EncryptRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn request_disconnect(
                        &mut self,
                        _: DisconnectRequest,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ConnectRequest>::default()
                                } else {
                                    <ConnectRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(connect),
                                    &parsed
                                );
                                let response = self.connect(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(connect),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            2u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <BindRequest>::default()
                                } else {
                                    <BindRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(bind),
                                    &parsed
                                );
                                let response = self.bind(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(2u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(bind),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            3u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <EchoRequest>::default()
                                } else {
                                    <EchoRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(echo),
                                    &parsed
                                );
                                let response = self.echo(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(3u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(echo),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            4u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <DisconnectNotification>::default()
                                } else {
                                    <DisconnectNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(force_disconnect),
                                    &parsed
                                );
                                let response = self.force_disconnect(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(4u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(force_disconnect),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            5u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <super::super::NoData>::default()
                                } else {
                                    <super::super::NoData>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(keep_alive),
                                    &parsed
                                );
                                let response = self.keep_alive(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(5u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(keep_alive),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            6u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <EncryptRequest>::default()
                                } else {
                                    <EncryptRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(encrypt),
                                    &parsed
                                );
                                let response = self.encrypt(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(6u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(encrypt),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            7u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <DisconnectRequest>::default()
                                } else {
                                    <DisconnectRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(request_disconnect),
                                    &parsed
                                );
                                let response = self.request_disconnect(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(7u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(ConnectionService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(request_disconnect),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
            }
        }
        pub mod friends {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct Friend {
                    #[prost(message, required, tag = "1")]
                    pub account_id: super::super::EntityId,
                    #[prost(message, repeated, tag = "2")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                    #[prost(uint32, repeated, tag = "3")]
                    pub role: ::prost::alloc::vec::Vec<u32>,
                    #[prost(uint64, optional, tag = "4")]
                    pub privileges: ::core::option::Option<u64>,
                    #[deprecated]
                    #[prost(uint64, optional, tag = "5")]
                    pub attributes_epoch: ::core::option::Option<u64>,
                    #[prost(uint64, optional, tag = "6")]
                    pub creation_time: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct FriendOfFriend {
                    #[prost(message, optional, tag = "1")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(uint32, repeated, tag = "3")]
                    pub role: ::prost::alloc::vec::Vec<u32>,
                    #[prost(uint64, optional, tag = "4")]
                    pub privileges: ::core::option::Option<u64>,
                    #[prost(string, optional, tag = "6")]
                    pub full_name: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "7")]
                    pub battle_tag: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ReceivedInvitation {
                    #[prost(fixed64, required, tag = "1")]
                    pub id: u64,
                    #[prost(message, required, tag = "2")]
                    pub inviter_identity: super::super::Identity,
                    #[prost(message, required, tag = "3")]
                    pub invitee_identity: super::super::Identity,
                    #[prost(string, optional, tag = "4")]
                    pub inviter_name: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "5")]
                    pub invitee_name: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(uint64, optional, tag = "7")]
                    pub creation_time: ::core::option::Option<u64>,
                    #[prost(fixed32, optional, tag = "9")]
                    pub program: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct FriendInvitation {
                    #[prost(uint32, repeated, tag = "2")]
                    pub role: ::prost::alloc::vec::Vec<u32>,
                    #[prost(message, repeated, tag = "3")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SentInvitation {
                    #[prost(fixed64, optional, tag = "1")]
                    pub id: ::core::option::Option<u64>,
                    #[prost(string, optional, tag = "2")]
                    pub target_name: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(uint32, optional, tag = "3")]
                    pub role: ::core::option::Option<u32>,
                    #[prost(message, repeated, tag = "4")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                    #[prost(uint64, optional, tag = "5")]
                    pub creation_time: ::core::option::Option<u64>,
                    #[prost(fixed32, optional, tag = "6")]
                    pub program: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct FriendInvitationParams {
                    #[prost(string, optional, tag = "1")]
                    pub target_email: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "2")]
                    pub target_battle_tag: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(uint32, repeated, tag = "6")]
                    pub role: ::prost::alloc::vec::Vec<u32>,
                    #[prost(message, repeated, tag = "8")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                    #[prost(string, optional, tag = "9")]
                    pub target_name: ::core::option::Option<::prost::alloc::string::String>,
                    #[deprecated]
                    #[prost(fixed32, optional, tag = "10")]
                    pub program: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SubscribeResponse {
                    #[deprecated]
                    #[prost(uint32, optional, tag = "1")]
                    pub max_friends: ::core::option::Option<u32>,
                    #[deprecated]
                    #[prost(uint32, optional, tag = "2")]
                    pub max_received_invitations: ::core::option::Option<u32>,
                    #[deprecated]
                    #[prost(uint32, optional, tag = "3")]
                    pub max_sent_invitations: ::core::option::Option<u32>,
                    #[prost(message, repeated, tag = "4")]
                    pub role: ::prost::alloc::vec::Vec<super::super::Role>,
                    #[prost(message, repeated, tag = "5")]
                    pub friends: ::prost::alloc::vec::Vec<Friend>,
                    #[prost(message, repeated, tag = "7")]
                    pub received_invitations: ::prost::alloc::vec::Vec<ReceivedInvitation>,
                    #[prost(message, repeated, tag = "8")]
                    pub sent_invitations: ::prost::alloc::vec::Vec<SentInvitation>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AcceptInvitationOptions {
                    #[prost(uint32, optional, tag = "1")]
                    pub role: ::core::option::Option<u32>,
                    #[prost(fixed32, optional, tag = "2")]
                    pub program: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SubscribeRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(uint64, required, tag = "2")]
                    pub object_id: u64,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct UnsubscribeRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(uint64, optional, tag = "2")]
                    pub object_id: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SendInvitationRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_identity: ::core::option::Option<super::super::Identity>,
                    #[prost(message, required, tag = "2")]
                    pub target_id: super::super::EntityId,
                    #[prost(message, required, tag = "3")]
                    pub params: super::super::InvitationParams,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct RevokeInvitationRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(fixed64, optional, tag = "2")]
                    pub invitation_id: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AcceptInvitationRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(fixed64, required, tag = "3")]
                    pub invitation_id: u64,
                    #[prost(message, optional, tag = "4")]
                    pub options: ::core::option::Option<AcceptInvitationOptions>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct DeclineInvitationRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(fixed64, required, tag = "3")]
                    pub invitation_id: u64,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct IgnoreInvitationRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(fixed64, required, tag = "3")]
                    pub invitation_id: u64,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct RemoveFriendRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, required, tag = "2")]
                    pub target_id: super::super::EntityId,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct RevokeAllInvitationsRequest {
                    #[prost(message, optional, tag = "2")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ViewFriendsRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, required, tag = "2")]
                    pub target_id: super::super::EntityId,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ViewFriendsResponse {
                    #[prost(message, repeated, tag = "1")]
                    pub friends: ::prost::alloc::vec::Vec<FriendOfFriend>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct UpdateFriendStateRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, required, tag = "2")]
                    pub target_id: super::super::EntityId,
                    #[prost(message, repeated, tag = "3")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetFriendListRequest {
                    #[prost(message, optional, tag = "2")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetFriendListResponse {
                    #[prost(message, repeated, tag = "1")]
                    pub friends: ::prost::alloc::vec::Vec<Friend>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct CreateFriendshipRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, optional, tag = "2")]
                    pub target_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(uint32, repeated, tag = "3")]
                    pub role: ::prost::alloc::vec::Vec<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct FriendNotification {
                    #[prost(message, required, tag = "1")]
                    pub target: Friend,
                    #[prost(message, optional, tag = "5")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct UpdateFriendStateNotification {
                    #[prost(message, required, tag = "1")]
                    pub changed_friend: Friend,
                    #[prost(message, optional, tag = "5")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct InvitationNotification {
                    #[prost(message, required, tag = "1")]
                    pub invitation: ReceivedInvitation,
                    #[prost(uint32, optional, tag = "3", default = "0")]
                    pub reason: ::core::option::Option<u32>,
                    #[prost(message, optional, tag = "5")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SentInvitationAddedNotification {
                    #[prost(message, optional, tag = "1")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, optional, tag = "2")]
                    pub invitation: ::core::option::Option<SentInvitation>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SentInvitationRemovedNotification {
                    #[prost(message, optional, tag = "1")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(fixed64, optional, tag = "2")]
                    pub invitation_id: ::core::option::Option<u64>,
                    #[prost(uint32, optional, tag = "3")]
                    pub reason: ::core::option::Option<u32>,
                }
                #[async_trait::async_trait]
                pub trait FriendsService: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 2883579235u32;
                    const ORIGINAL_HASH: u32 = 2749215165u32;
                    const OUTBOUND: bool = true;
                    const SHARD_NAME: &'static str = "friends";
                    const RESOLVE_CLIENT_INSTANCE: bool = true;
                    const DESCRIPTOR_NAME: &'static str = "bnet.protocol.friends.FriendsService";
                    const SUBSCRIBE: u8 = 1u8;
                    const SEND_INVITATION: u8 = 2u8;
                    const ACCEPT_INVITATION: u8 = 3u8;
                    const REVOKE_INVITATION: u8 = 4u8;
                    const DECLINE_INVITATION: u8 = 5u8;
                    const IGNORE_INVITATION: u8 = 6u8;
                    const REMOVE_FRIEND: u8 = 8u8;
                    const VIEW_FRIENDS: u8 = 9u8;
                    const UPDATE_FRIEND_STATE: u8 = 10u8;
                    const UNSUBSCRIBE: u8 = 11u8;
                    const REVOKE_ALL_INVITATIONS: u8 = 12u8;
                    const GET_FRIEND_LIST: u8 = 13u8;
                    const CREATE_FRIENDSHIP: u8 = 14u8;
                    async fn subscribe(
                        &mut self,
                        _: SubscribeRequest,
                    ) -> Result<SubscribeResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn send_invitation(
                        &mut self,
                        _: SendInvitationRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn accept_invitation(
                        &mut self,
                        _: AcceptInvitationRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn revoke_invitation(
                        &mut self,
                        _: RevokeInvitationRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn decline_invitation(
                        &mut self,
                        _: DeclineInvitationRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn ignore_invitation(
                        &mut self,
                        _: IgnoreInvitationRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn remove_friend(
                        &mut self,
                        _: RemoveFriendRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn view_friends(
                        &mut self,
                        _: ViewFriendsRequest,
                    ) -> Result<ViewFriendsResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn update_friend_state(
                        &mut self,
                        _: UpdateFriendStateRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn unsubscribe(
                        &mut self,
                        _: UnsubscribeRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn revoke_all_invitations(
                        &mut self,
                        _: RevokeAllInvitationsRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn get_friend_list(
                        &mut self,
                        _: GetFriendListRequest,
                    ) -> Result<GetFriendListResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn create_friendship(
                        &mut self,
                        _: CreateFriendshipRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <SubscribeRequest>::default()
                                } else {
                                    <SubscribeRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(subscribe),
                                    &parsed
                                );
                                let response = self.subscribe(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(subscribe),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            2u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <SendInvitationRequest>::default()
                                } else {
                                    <SendInvitationRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(send_invitation),
                                    &parsed
                                );
                                let response = self.send_invitation(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(2u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(send_invitation),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            3u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <AcceptInvitationRequest>::default()
                                } else {
                                    <AcceptInvitationRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(accept_invitation),
                                    &parsed
                                );
                                let response = self.accept_invitation(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(3u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(accept_invitation),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            4u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <RevokeInvitationRequest>::default()
                                } else {
                                    <RevokeInvitationRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(revoke_invitation),
                                    &parsed
                                );
                                let response = self.revoke_invitation(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(4u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(revoke_invitation),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            5u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <DeclineInvitationRequest>::default()
                                } else {
                                    <DeclineInvitationRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(decline_invitation),
                                    &parsed
                                );
                                let response = self.decline_invitation(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(5u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(decline_invitation),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            6u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <IgnoreInvitationRequest>::default()
                                } else {
                                    <IgnoreInvitationRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(ignore_invitation),
                                    &parsed
                                );
                                let response = self.ignore_invitation(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(6u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(ignore_invitation),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            8u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <RemoveFriendRequest>::default()
                                } else {
                                    <RemoveFriendRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(remove_friend),
                                    &parsed
                                );
                                let response = self.remove_friend(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(8u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(remove_friend),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            9u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ViewFriendsRequest>::default()
                                } else {
                                    <ViewFriendsRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(view_friends),
                                    &parsed
                                );
                                let response = self.view_friends(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(9u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(view_friends),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            10u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <UpdateFriendStateRequest>::default()
                                } else {
                                    <UpdateFriendStateRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(update_friend_state),
                                    &parsed
                                );
                                let response = self.update_friend_state(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(10u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(update_friend_state),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            11u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <UnsubscribeRequest>::default()
                                } else {
                                    <UnsubscribeRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unsubscribe),
                                    &parsed
                                );
                                let response = self.unsubscribe(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(11u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unsubscribe),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            12u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <RevokeAllInvitationsRequest>::default()
                                } else {
                                    <RevokeAllInvitationsRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(revoke_all_invitations),
                                    &parsed
                                );
                                let response = self.revoke_all_invitations(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(12u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(revoke_all_invitations),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            13u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GetFriendListRequest>::default()
                                } else {
                                    <GetFriendListRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_friend_list),
                                    &parsed
                                );
                                let response = self.get_friend_list(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(13u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_friend_list),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            14u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <CreateFriendshipRequest>::default()
                                } else {
                                    <CreateFriendshipRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(create_friendship),
                                    &parsed
                                );
                                let response = self.create_friendship(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(14u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(create_friendship),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
                #[async_trait::async_trait]
                pub trait FriendsListener: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 2792453448u32;
                    const ORIGINAL_HASH: u32 = 1864735251u32;
                    const INBOUND: bool = true;
                    const DESCRIPTOR_NAME: &'static str = "bnet.protocol.friends.FriendsNotify";
                    const ON_FRIEND_ADDED: u8 = 1u8;
                    const ON_FRIEND_REMOVED: u8 = 2u8;
                    const ON_RECEIVED_INVITATION_ADDED: u8 = 3u8;
                    const ON_RECEIVED_INVITATION_REMOVED: u8 = 4u8;
                    const ON_SENT_INVITATION_ADDED: u8 = 5u8;
                    const ON_SENT_INVITATION_REMOVED: u8 = 6u8;
                    const ON_UPDATE_FRIEND_STATE: u8 = 7u8;
                    async fn on_friend_added(
                        &mut self,
                        _: FriendNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_friend_removed(
                        &mut self,
                        _: FriendNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_received_invitation_added(
                        &mut self,
                        _: InvitationNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_received_invitation_removed(
                        &mut self,
                        _: InvitationNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_sent_invitation_added(
                        &mut self,
                        _: SentInvitationAddedNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_sent_invitation_removed(
                        &mut self,
                        _: SentInvitationRemovedNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_update_friend_state(
                        &mut self,
                        _: UpdateFriendStateNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <FriendNotification>::default()
                                } else {
                                    <FriendNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_friend_added),
                                    &parsed
                                );
                                let response = self.on_friend_added(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_friend_added),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            2u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <FriendNotification>::default()
                                } else {
                                    <FriendNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_friend_removed),
                                    &parsed
                                );
                                let response = self.on_friend_removed(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(2u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_friend_removed),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            3u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <InvitationNotification>::default()
                                } else {
                                    <InvitationNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_received_invitation_added),
                                    &parsed
                                );
                                let response = self.on_received_invitation_added(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(3u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_received_invitation_added),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            4u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <InvitationNotification>::default()
                                } else {
                                    <InvitationNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_received_invitation_removed),
                                    &parsed
                                );
                                let response = self.on_received_invitation_removed(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(4u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_received_invitation_removed),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            5u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <SentInvitationAddedNotification>::default()
                                } else {
                                    <SentInvitationAddedNotification>::decode(
                                        &mut msg.data.clone(),
                                    )?
                                };
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_sent_invitation_added),
                                    &parsed
                                );
                                let response = self.on_sent_invitation_added(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(5u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_sent_invitation_added),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            6u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <SentInvitationRemovedNotification>::default()
                                } else {
                                    <SentInvitationRemovedNotification>::decode(
                                        &mut msg.data.clone(),
                                    )?
                                };
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_sent_invitation_removed),
                                    &parsed
                                );
                                let response = self.on_sent_invitation_removed(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(6u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_sent_invitation_removed),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            7u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <UpdateFriendStateNotification>::default()
                                } else {
                                    <UpdateFriendStateNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_update_friend_state),
                                    &parsed
                                );
                                let response = self.on_update_friend_state(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(7u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(FriendsListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_update_friend_state),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
            }
        }
        pub mod game_utilities {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct PlayerVariables {
                    #[prost(message, required, tag = "1")]
                    pub identity: super::super::Identity,
                    #[prost(double, optional, tag = "2")]
                    pub rating: ::core::option::Option<f64>,
                    #[prost(message, repeated, tag = "3")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClientInfo {
                    #[prost(string, optional, tag = "1")]
                    pub client_address: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(bool, optional, tag = "2")]
                    pub privileged_network: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClientRequest {
                    #[prost(message, repeated, tag = "1")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                    #[deprecated]
                    #[prost(message, optional, tag = "2")]
                    pub host: ::core::option::Option<super::super::ProcessId>,
                    #[prost(message, optional, tag = "3")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, optional, tag = "4")]
                    pub game_account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(fixed32, optional, tag = "5")]
                    pub program: ::core::option::Option<u32>,
                    #[prost(message, optional, tag = "6")]
                    pub client_info: ::core::option::Option<ClientInfo>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClientResponse {
                    #[prost(message, repeated, tag = "1")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ServerRequest {
                    #[prost(message, repeated, tag = "1")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                    #[prost(fixed32, required, tag = "2")]
                    pub program: u32,
                    #[deprecated]
                    #[prost(message, optional, tag = "3")]
                    pub host: ::core::option::Option<super::super::ProcessId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ServerResponse {
                    #[prost(message, repeated, tag = "1")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct PresenceChannelCreatedRequest {
                    #[prost(message, required, tag = "1")]
                    pub id: super::super::EntityId,
                    #[prost(message, optional, tag = "3")]
                    pub game_account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, optional, tag = "4")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                    #[deprecated]
                    #[prost(message, optional, tag = "5")]
                    pub host: ::core::option::Option<super::super::ProcessId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountOnlineNotification {
                    #[prost(message, required, tag = "1")]
                    pub game_account_id: super::super::EntityId,
                    #[deprecated]
                    #[prost(message, optional, tag = "2")]
                    pub host: ::core::option::Option<super::super::ProcessId>,
                    #[prost(string, optional, tag = "3")]
                    pub session_id: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GameAccountOfflineNotification {
                    #[prost(message, required, tag = "1")]
                    pub game_account_id: super::super::EntityId,
                    #[deprecated]
                    #[prost(message, optional, tag = "2")]
                    pub host: ::core::option::Option<super::super::ProcessId>,
                    #[prost(string, optional, tag = "3")]
                    pub session_id: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetAllValuesForAttributeRequest {
                    #[prost(string, optional, tag = "1")]
                    pub attribute_key: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(message, optional, tag = "2")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(fixed32, optional, tag = "5")]
                    pub program: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct GetAllValuesForAttributeResponse {
                    #[prost(message, repeated, tag = "1")]
                    pub attribute_value: ::prost::alloc::vec::Vec<super::super::Variant>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct RegisterUtilitiesRequest {
                    #[prost(message, repeated, tag = "1")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                    #[prost(fixed32, optional, tag = "2")]
                    pub program: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct RegisterUtilitiesResponse {
                    #[prost(string, optional, tag = "1")]
                    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct UnregisterUtilitiesRequest {}
                #[async_trait::async_trait]
                pub trait GameUtilitiesService: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 1368537640u32;
                    const ORIGINAL_HASH: u32 = 1069623117u32;
                    const INBOUND: bool = true;
                    const SHARD_NAME: &'static str = "game_utilities";
                    const OUTBOUND: bool = true;
                    const DESCRIPTOR_NAME: &'static str =
                        "bnet.protocol.game_utilities.GameUtilities";
                    const PROCESS_CLIENT_REQUEST: u8 = 1u8;
                    const PRESENCE_CHANNEL_CREATED: u8 = 2u8;
                    const PROCESS_SERVER_REQUEST: u8 = 6u8;
                    const ON_GAME_ACCOUNT_ONLINE: u8 = 7u8;
                    const ON_GAME_ACCOUNT_OFFLINE: u8 = 8u8;
                    const GET_ALL_VALUES_FOR_ATTRIBUTE: u8 = 10u8;
                    const REGISTER_UTILITIES: u8 = 11u8;
                    const UNREGISTER_UTILITIES: u8 = 12u8;
                    async fn process_client_request(
                        &mut self,
                        _: ClientRequest,
                    ) -> Result<ClientResponse, crate::errors::WowRpcResponse> {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn presence_channel_created(
                        &mut self,
                        _: PresenceChannelCreatedRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn process_server_request(
                        &mut self,
                        _: ServerRequest,
                    ) -> Result<ServerResponse, crate::errors::WowRpcResponse> {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_game_account_online(
                        &mut self,
                        _: GameAccountOnlineNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_game_account_offline(
                        &mut self,
                        _: GameAccountOfflineNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn get_all_values_for_attribute(
                        &mut self,
                        _: GetAllValuesForAttributeRequest,
                    ) -> Result<GetAllValuesForAttributeResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn register_utilities(
                        &mut self,
                        _: RegisterUtilitiesRequest,
                    ) -> Result<RegisterUtilitiesResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn unregister_utilities(
                        &mut self,
                        _: UnregisterUtilitiesRequest,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ClientRequest>::default()
                                } else {
                                    <ClientRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(process_client_request),
                                    &parsed
                                );
                                let response = self.process_client_request(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(process_client_request),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            2u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <PresenceChannelCreatedRequest>::default()
                                } else {
                                    <PresenceChannelCreatedRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(presence_channel_created),
                                    &parsed
                                );
                                let response = self.presence_channel_created(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(2u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(presence_channel_created),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            6u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ServerRequest>::default()
                                } else {
                                    <ServerRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(process_server_request),
                                    &parsed
                                );
                                let response = self.process_server_request(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(6u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(process_server_request),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            7u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GameAccountOnlineNotification>::default()
                                } else {
                                    <GameAccountOnlineNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_account_online),
                                    &parsed
                                );
                                let response = self.on_game_account_online(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(7u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_account_online),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            8u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GameAccountOfflineNotification>::default()
                                } else {
                                    <GameAccountOfflineNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_account_offline),
                                    &parsed
                                );
                                let response = self.on_game_account_offline(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(8u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_game_account_offline),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            10u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <GetAllValuesForAttributeRequest>::default()
                                } else {
                                    <GetAllValuesForAttributeRequest>::decode(
                                        &mut msg.data.clone(),
                                    )?
                                };
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_all_values_for_attribute),
                                    &parsed
                                );
                                let response = self.get_all_values_for_attribute(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(10u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_all_values_for_attribute),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            11u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <RegisterUtilitiesRequest>::default()
                                } else {
                                    <RegisterUtilitiesRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(register_utilities),
                                    &parsed
                                );
                                let response = self.register_utilities(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(11u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(register_utilities),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            12u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <UnregisterUtilitiesRequest>::default()
                                } else {
                                    <UnregisterUtilitiesRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unregister_utilities),
                                    &parsed
                                );
                                let response = self.unregister_utilities(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(12u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(GameUtilitiesService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unregister_utilities),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
            }
        }
        pub mod notification {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct Target {
                    #[prost(message, optional, tag = "1")]
                    pub identity: ::core::option::Option<TargetIdentity>,
                    #[prost(string, optional, tag = "2")]
                    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct TargetIdentity {
                    #[prost(message, optional, tag = "1")]
                    pub account: ::core::option::Option<super::super::account::v1::AccountId>,
                    #[prost(message, optional, tag = "2")]
                    pub game_account:
                        ::core::option::Option<super::super::account::v1::GameAccountHandle>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct Subscription {
                    #[prost(message, repeated, tag = "1")]
                    pub target: ::prost::alloc::vec::Vec<Target>,
                    #[prost(message, optional, tag = "2")]
                    pub subscriber: ::core::option::Option<super::super::account::v1::Identity>,
                    #[deprecated]
                    #[prost(bool, optional, tag = "3")]
                    pub delivery_required: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct Notification {
                    #[prost(message, optional, tag = "1")]
                    pub sender_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, required, tag = "2")]
                    pub target_id: super::super::EntityId,
                    #[prost(string, required, tag = "3")]
                    pub r#type: ::prost::alloc::string::String,
                    #[prost(message, repeated, tag = "4")]
                    pub attribute: ::prost::alloc::vec::Vec<super::super::Attribute>,
                    #[prost(message, optional, tag = "5")]
                    pub sender_account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, optional, tag = "6")]
                    pub target_account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(string, optional, tag = "7")]
                    pub sender_battle_tag: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(string, optional, tag = "8")]
                    pub target_battle_tag: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(message, optional, tag = "10")]
                    pub forwarding_identity:
                        ::core::option::Option<super::super::account::v1::Identity>,
                }
            }
        }
        pub mod presence {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct RichPresenceLocalizationKey {
                    #[prost(fixed32, required, tag = "1")]
                    pub program: u32,
                    #[prost(fixed32, required, tag = "2")]
                    pub stream: u32,
                    #[prost(uint32, required, tag = "3")]
                    pub localization_id: u32,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct FieldKey {
                    #[prost(uint32, required, tag = "1")]
                    pub program: u32,
                    #[prost(uint32, required, tag = "2")]
                    pub group: u32,
                    #[prost(uint32, required, tag = "3")]
                    pub field: u32,
                    #[prost(uint64, optional, tag = "4")]
                    pub unique_id: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct Field {
                    #[prost(message, required, tag = "1")]
                    pub key: FieldKey,
                    #[prost(message, required, tag = "2")]
                    pub value: super::super::Variant,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct FieldOperation {
                    #[prost(message, required, tag = "1")]
                    pub field: Field,
                    #[prost(
                        enumeration = "field_operation::OperationType",
                        optional,
                        tag = "2",
                        default = "Set"
                    )]
                    pub operation: ::core::option::Option<i32>,
                }
                #[doc = " Nested message and enum types in `FieldOperation`."]
                pub mod field_operation {
                    #[derive(
                        serde :: Serialize,
                        Clone,
                        Copy,
                        Debug,
                        PartialEq,
                        Eq,
                        Hash,
                        PartialOrd,
                        Ord,
                        :: prost :: Enumeration,
                    )]
                    #[repr(i32)]
                    pub enum OperationType {
                        Set = 0,
                        Clear = 1,
                    }
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct PresenceState {
                    #[prost(message, optional, tag = "1")]
                    pub entity_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, repeated, tag = "2")]
                    pub field_operation: ::prost::alloc::vec::Vec<FieldOperation>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ChannelState {
                    #[prost(message, optional, tag = "1")]
                    pub entity_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, repeated, tag = "2")]
                    pub field_operation: ::prost::alloc::vec::Vec<FieldOperation>,
                    #[prost(bool, optional, tag = "3")]
                    pub healing: ::core::option::Option<bool>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SubscribeNotification {
                    #[prost(message, optional, tag = "1")]
                    pub subscriber_id: ::core::option::Option<super::super::account::v1::AccountId>,
                    #[prost(message, repeated, tag = "2")]
                    pub state: ::prost::alloc::vec::Vec<PresenceState>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct StateChangedNotification {
                    #[prost(message, optional, tag = "1")]
                    pub subscriber_id: ::core::option::Option<super::super::account::v1::AccountId>,
                    #[prost(message, repeated, tag = "2")]
                    pub state: ::prost::alloc::vec::Vec<PresenceState>,
                }
                #[async_trait::async_trait]
                pub trait PresenceListener: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 3900927824u32;
                    const ORIGINAL_HASH: u32 = 2299181151u32;
                    const DESCRIPTOR_NAME: &'static str =
                        "bnet.protocol.presence.v1.PresenceListener";
                    const INBOUND: bool = true;
                    const ON_SUBSCRIBE: u8 = 1u8;
                    const ON_STATE_CHANGED: u8 = 2u8;
                    async fn on_subscribe(
                        &mut self,
                        _: SubscribeNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_state_changed(
                        &mut self,
                        _: StateChangedNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <SubscribeNotification>::default()
                                } else {
                                    <SubscribeNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(PresenceListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_subscribe),
                                    &parsed
                                );
                                let response = self.on_subscribe(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(PresenceListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_subscribe),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            2u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <StateChangedNotification>::default()
                                } else {
                                    <StateChangedNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(PresenceListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_state_changed),
                                    &parsed
                                );
                                let response = self.on_state_changed(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(2u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(PresenceListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_state_changed),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SubscribeRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, required, tag = "2")]
                    pub entity_id: super::super::EntityId,
                    #[prost(uint64, required, tag = "3")]
                    pub object_id: u64,
                    #[prost(fixed32, repeated, packed = "false", tag = "4")]
                    pub program: ::prost::alloc::vec::Vec<u32>,
                    #[prost(message, repeated, tag = "6")]
                    pub key: ::prost::alloc::vec::Vec<FieldKey>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct UnsubscribeRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, required, tag = "2")]
                    pub entity_id: super::super::EntityId,
                    #[prost(uint64, optional, tag = "3")]
                    pub object_id: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct UpdateRequest {
                    #[prost(message, required, tag = "1")]
                    pub entity_id: super::super::EntityId,
                    #[prost(message, repeated, tag = "2")]
                    pub field_operation: ::prost::alloc::vec::Vec<FieldOperation>,
                    #[prost(bool, optional, tag = "3")]
                    pub no_create: ::core::option::Option<bool>,
                    #[prost(message, optional, tag = "4")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct QueryRequest {
                    #[prost(message, required, tag = "1")]
                    pub entity_id: super::super::EntityId,
                    #[prost(message, repeated, tag = "2")]
                    pub key: ::prost::alloc::vec::Vec<FieldKey>,
                    #[prost(message, optional, tag = "3")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct QueryResponse {
                    #[prost(message, repeated, tag = "2")]
                    pub field: ::prost::alloc::vec::Vec<Field>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct BatchSubscribeRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, repeated, tag = "2")]
                    pub entity_id: ::prost::alloc::vec::Vec<super::super::EntityId>,
                    #[prost(fixed32, repeated, packed = "false", tag = "3")]
                    pub program: ::prost::alloc::vec::Vec<u32>,
                    #[prost(message, repeated, tag = "4")]
                    pub key: ::prost::alloc::vec::Vec<FieldKey>,
                    #[prost(uint64, optional, tag = "5")]
                    pub object_id: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SubscribeResult {
                    #[prost(message, optional, tag = "1")]
                    pub entity_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(uint32, optional, tag = "2")]
                    pub result: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct BatchSubscribeResponse {
                    #[prost(message, repeated, tag = "1")]
                    pub subscribe_failed: ::prost::alloc::vec::Vec<SubscribeResult>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct BatchUnsubscribeRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, repeated, tag = "2")]
                    pub entity_id: ::prost::alloc::vec::Vec<super::super::EntityId>,
                    #[prost(uint64, optional, tag = "3")]
                    pub object_id: ::core::option::Option<u64>,
                }
                #[async_trait::async_trait]
                pub trait PresenceService: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 3640216379u32;
                    const ORIGINAL_HASH: u32 = 4194801407u32;
                    const SHARD_NAME: &'static str = "presence_aggregator";
                    const RESOLVE_CLIENT_INSTANCE: bool = true;
                    const DESCRIPTOR_NAME: &'static str = "bnet.protocol.presence.PresenceService";
                    const OUTBOUND: bool = true;
                    const SUBSCRIBE: u8 = 1u8;
                    const UNSUBSCRIBE: u8 = 2u8;
                    const UPDATE: u8 = 3u8;
                    const QUERY: u8 = 4u8;
                    const BATCH_SUBSCRIBE: u8 = 8u8;
                    const BATCH_UNSUBSCRIBE: u8 = 9u8;
                    async fn subscribe(
                        &mut self,
                        _: SubscribeRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn unsubscribe(
                        &mut self,
                        _: UnsubscribeRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn update(
                        &mut self,
                        _: UpdateRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn query(
                        &mut self,
                        _: QueryRequest,
                    ) -> Result<QueryResponse, crate::errors::WowRpcResponse> {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn batch_subscribe(
                        &mut self,
                        _: BatchSubscribeRequest,
                    ) -> Result<BatchSubscribeResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn batch_unsubscribe(
                        &mut self,
                        _: BatchUnsubscribeRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <SubscribeRequest>::default()
                                } else {
                                    <SubscribeRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(subscribe),
                                    &parsed
                                );
                                let response = self.subscribe(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(subscribe),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            2u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <UnsubscribeRequest>::default()
                                } else {
                                    <UnsubscribeRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unsubscribe),
                                    &parsed
                                );
                                let response = self.unsubscribe(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(2u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unsubscribe),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            3u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <UpdateRequest>::default()
                                } else {
                                    <UpdateRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(update),
                                    &parsed
                                );
                                let response = self.update(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(3u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(update),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            4u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <QueryRequest>::default()
                                } else {
                                    <QueryRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(query),
                                    &parsed
                                );
                                let response = self.query(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(4u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(query),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            8u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <BatchSubscribeRequest>::default()
                                } else {
                                    <BatchSubscribeRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(batch_subscribe),
                                    &parsed
                                );
                                let response = self.batch_subscribe(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(8u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(batch_subscribe),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            9u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <BatchUnsubscribeRequest>::default()
                                } else {
                                    <BatchUnsubscribeRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(batch_unsubscribe),
                                    &parsed
                                );
                                let response = self.batch_unsubscribe(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(9u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(PresenceService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(batch_unsubscribe),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
            }
        }
        pub mod profanity {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct WordFilter {
                    #[prost(string, required, tag = "1")]
                    pub r#type: ::prost::alloc::string::String,
                    #[prost(string, required, tag = "2")]
                    pub regex: ::prost::alloc::string::String,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct WordFilters {
                    #[prost(message, repeated, tag = "1")]
                    pub filters: ::prost::alloc::vec::Vec<WordFilter>,
                }
            }
        }
        pub mod resources {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ContentHandleRequest {
                    #[prost(fixed32, required, tag = "1")]
                    pub program: u32,
                    #[prost(fixed32, required, tag = "2")]
                    pub stream: u32,
                    #[prost(fixed32, optional, tag = "3", default = "1701729619")]
                    pub version: ::core::option::Option<u32>,
                }
                #[async_trait::async_trait]
                pub trait ResourcesService: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 1259359315u32;
                    const ORIGINAL_HASH: u32 = 3971904954u32;
                    const OUTBOUND: bool = true;
                    const DESCRIPTOR_NAME: &'static str = "bnet.protocol.resources.Resources";
                    const SHARD_NAME: &'static str = "resources";
                    const GET_CONTENT_HANDLE: u8 = 1u8;
                    async fn get_content_handle(
                        &mut self,
                        _: ContentHandleRequest,
                    ) -> Result<super::super::ContentHandle, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ContentHandleRequest>::default()
                                } else {
                                    <ContentHandleRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(ResourcesService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_content_handle),
                                    &parsed
                                );
                                let response = self.get_content_handle(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(ResourcesService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(get_content_handle),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
            }
        }
        pub mod config {
            #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
            pub struct RpcMethodConfig {
                #[deprecated]
                #[prost(string, optional, tag = "1")]
                pub service_name: ::core::option::Option<::prost::alloc::string::String>,
                #[deprecated]
                #[prost(string, optional, tag = "2")]
                pub method_name: ::core::option::Option<::prost::alloc::string::String>,
                #[prost(uint32, optional, tag = "3", default = "1")]
                pub fixed_call_cost: ::core::option::Option<u32>,
                #[prost(uint32, optional, tag = "4", default = "0")]
                pub fixed_packet_size: ::core::option::Option<u32>,
                #[prost(float, optional, tag = "5", default = "0")]
                pub variable_multiplier: ::core::option::Option<f32>,
                #[prost(float, optional, tag = "6", default = "1")]
                pub multiplier: ::core::option::Option<f32>,
                #[prost(uint32, optional, tag = "7")]
                pub rate_limit_count: ::core::option::Option<u32>,
                #[prost(uint32, optional, tag = "8")]
                pub rate_limit_seconds: ::core::option::Option<u32>,
                #[prost(uint32, optional, tag = "9")]
                pub max_packet_size: ::core::option::Option<u32>,
                #[prost(uint32, optional, tag = "10")]
                pub max_encoded_size: ::core::option::Option<u32>,
                #[prost(float, optional, tag = "11")]
                pub timeout: ::core::option::Option<f32>,
                #[prost(uint32, optional, tag = "12")]
                pub cap_balance: ::core::option::Option<u32>,
                #[prost(float, optional, tag = "13", default = "0")]
                pub income_per_second: ::core::option::Option<f32>,
                #[prost(uint32, optional, tag = "14")]
                pub service_hash: ::core::option::Option<u32>,
                #[prost(uint32, optional, tag = "15")]
                pub method_id: ::core::option::Option<u32>,
            }
            #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
            pub struct RpcMeterConfig {
                #[prost(message, repeated, tag = "1")]
                pub method: ::prost::alloc::vec::Vec<RpcMethodConfig>,
                #[prost(uint32, optional, tag = "2", default = "1")]
                pub income_per_second: ::core::option::Option<u32>,
                #[prost(uint32, optional, tag = "3")]
                pub initial_balance: ::core::option::Option<u32>,
                #[prost(uint32, optional, tag = "4")]
                pub cap_balance: ::core::option::Option<u32>,
                #[prost(float, optional, tag = "5", default = "0")]
                pub startup_period: ::core::option::Option<f32>,
            }
            #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
            pub struct ProtocolAlias {
                #[prost(string, required, tag = "1")]
                pub server_service_name: ::prost::alloc::string::String,
                #[prost(string, required, tag = "2")]
                pub client_service_name: ::prost::alloc::string::String,
            }
            #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
            pub struct ServiceAliases {
                #[prost(message, repeated, tag = "1")]
                pub protocol_alias: ::prost::alloc::vec::Vec<ProtocolAlias>,
            }
        }
        pub mod user_manager {
            pub mod v1 {
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct RecentPlayer {
                    #[prost(message, required, tag = "1")]
                    pub entity_id: super::super::EntityId,
                    #[prost(string, optional, tag = "2")]
                    pub program: ::core::option::Option<::prost::alloc::string::String>,
                    #[prost(fixed64, optional, tag = "3")]
                    pub timestamp_played: ::core::option::Option<u64>,
                    #[prost(message, repeated, tag = "4")]
                    pub attributes: ::prost::alloc::vec::Vec<super::super::Attribute>,
                    #[prost(fixed32, optional, tag = "6", default = "0")]
                    pub counter: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct BlockedPlayer {
                    #[prost(message, required, tag = "1")]
                    pub account_id: super::super::EntityId,
                    #[prost(string, optional, tag = "2")]
                    pub battle_tag: ::core::option::Option<::prost::alloc::string::String>,
                    #[deprecated]
                    #[prost(uint32, repeated, packed = "false", tag = "3")]
                    pub role: ::prost::alloc::vec::Vec<u32>,
                    #[deprecated]
                    #[prost(uint64, optional, tag = "4")]
                    pub privileges: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SubscribeRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(uint64, required, tag = "2")]
                    pub object_id: u64,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct SubscribeResponse {
                    #[prost(message, repeated, tag = "1")]
                    pub blocked_players: ::prost::alloc::vec::Vec<BlockedPlayer>,
                    #[prost(message, repeated, tag = "2")]
                    pub recent_players: ::prost::alloc::vec::Vec<RecentPlayer>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct UnsubscribeRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(uint64, optional, tag = "2")]
                    pub object_id: ::core::option::Option<u64>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct AddRecentPlayersRequest {
                    #[prost(message, repeated, tag = "1")]
                    pub players: ::prost::alloc::vec::Vec<RecentPlayer>,
                    #[prost(message, optional, tag = "2")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(uint32, optional, tag = "3")]
                    pub program: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct ClearRecentPlayersRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(uint32, optional, tag = "2")]
                    pub program: ::core::option::Option<u32>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct BlockPlayerRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, required, tag = "2")]
                    pub target_id: super::super::EntityId,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct UnblockPlayerRequest {
                    #[prost(message, optional, tag = "1")]
                    pub agent_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, required, tag = "2")]
                    pub target_id: super::super::EntityId,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct BlockedPlayerAddedNotification {
                    #[prost(message, required, tag = "1")]
                    pub player: BlockedPlayer,
                    #[prost(message, optional, tag = "2")]
                    pub game_account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, optional, tag = "3")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct BlockedPlayerRemovedNotification {
                    #[prost(message, required, tag = "1")]
                    pub player: BlockedPlayer,
                    #[prost(message, optional, tag = "2")]
                    pub game_account_id: ::core::option::Option<super::super::EntityId>,
                    #[prost(message, optional, tag = "3")]
                    pub account_id: ::core::option::Option<super::super::EntityId>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct RecentPlayersAddedNotification {
                    #[prost(message, repeated, tag = "1")]
                    pub player: ::prost::alloc::vec::Vec<RecentPlayer>,
                }
                #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
                pub struct RecentPlayersRemovedNotification {
                    #[prost(message, repeated, tag = "1")]
                    pub player: ::prost::alloc::vec::Vec<RecentPlayer>,
                }
                #[async_trait::async_trait]
                pub trait UserManagerService: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 2397399374u32;
                    const ORIGINAL_HASH: u32 = 1041835658u32;
                    const DESCRIPTOR_NAME: &'static str =
                        "bnet.protocol.user_manager.UserManagerService";
                    const SHARD_NAME: &'static str = "user_manager";
                    const RESOLVE_CLIENT_INSTANCE: bool = true;
                    const OUTBOUND: bool = true;
                    const SUBSCRIBE: u8 = 1u8;
                    const ADD_RECENT_PLAYERS: u8 = 10u8;
                    const CLEAR_RECENT_PLAYERS: u8 = 11u8;
                    const BLOCK_PLAYER: u8 = 20u8;
                    const UNBLOCK_PLAYER: u8 = 21u8;
                    const BLOCK_PLAYER_FOR_SESSION: u8 = 40u8;
                    const UNSUBSCRIBE: u8 = 51u8;
                    async fn subscribe(
                        &mut self,
                        _: SubscribeRequest,
                    ) -> Result<SubscribeResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn add_recent_players(
                        &mut self,
                        _: AddRecentPlayersRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn clear_recent_players(
                        &mut self,
                        _: ClearRecentPlayersRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn block_player(
                        &mut self,
                        _: BlockPlayerRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn unblock_player(
                        &mut self,
                        _: UnblockPlayerRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn block_player_for_session(
                        &mut self,
                        _: BlockPlayerRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn unsubscribe(
                        &mut self,
                        _: UnsubscribeRequest,
                    ) -> Result<super::super::NoData, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <SubscribeRequest>::default()
                                } else {
                                    <SubscribeRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(subscribe),
                                    &parsed
                                );
                                let response = self.subscribe(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(subscribe),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            10u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <AddRecentPlayersRequest>::default()
                                } else {
                                    <AddRecentPlayersRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(add_recent_players),
                                    &parsed
                                );
                                let response = self.add_recent_players(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(10u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(add_recent_players),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            11u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <ClearRecentPlayersRequest>::default()
                                } else {
                                    <ClearRecentPlayersRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(clear_recent_players),
                                    &parsed
                                );
                                let response = self.clear_recent_players(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(11u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(clear_recent_players),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            20u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <BlockPlayerRequest>::default()
                                } else {
                                    <BlockPlayerRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(block_player),
                                    &parsed
                                );
                                let response = self.block_player(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(20u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(block_player),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            21u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <UnblockPlayerRequest>::default()
                                } else {
                                    <UnblockPlayerRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unblock_player),
                                    &parsed
                                );
                                let response = self.unblock_player(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(21u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unblock_player),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            40u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <BlockPlayerRequest>::default()
                                } else {
                                    <BlockPlayerRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(block_player_for_session),
                                    &parsed
                                );
                                let response = self.block_player_for_session(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(40u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(block_player_for_session),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            51u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <UnsubscribeRequest>::default()
                                } else {
                                    <UnsubscribeRequest>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unsubscribe),
                                    &parsed
                                );
                                let response = self.unsubscribe(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(51u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(UserManagerService),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(unsubscribe),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
                #[async_trait::async_trait]
                pub trait UserManagerListener: crate::messages::LoggingAttributes {
                    const NAME_HASH: u32 = 3007474611u32;
                    const ORIGINAL_HASH: u32 = 3162975266u32;
                    const INBOUND: bool = true;
                    const DESCRIPTOR_NAME: &'static str =
                        "bnet.protocol.user_manager.UserManagerNotify";
                    const ON_BLOCKED_PLAYER_ADDED: u8 = 1u8;
                    const ON_BLOCKED_PLAYER_REMOVED: u8 = 2u8;
                    const ON_RECENT_PLAYERS_ADDED: u8 = 11u8;
                    const ON_RECENT_PLAYERS_REMOVED: u8 = 12u8;
                    async fn on_blocked_player_added(
                        &mut self,
                        _: BlockedPlayerAddedNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_blocked_player_removed(
                        &mut self,
                        _: BlockedPlayerRemovedNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_recent_players_added(
                        &mut self,
                        _: RecentPlayersAddedNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn on_recent_players_removed(
                        &mut self,
                        _: RecentPlayersRemovedNotification,
                    ) -> Result<super::super::NoResponse, crate::errors::WowRpcResponse>
                    {
                        Err(crate::errors::WowRpcResponse::NotImplemented)
                    }
                    async fn dispatch(
                        &mut self,
                        msg: crate::messages::RawMessage,
                    ) -> Result<bytes::Bytes, crate::errors::WowRpcResponse> {
                        use prost::Message;
                        let method_id = msg
                            .headers
                            .method_id
                            .ok_or_else(|| crate::errors::WowRpcResponse::RpcMalformedRequest)?
                            as u8;
                        match method_id {
                            1u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <BlockedPlayerAddedNotification>::default()
                                } else {
                                    <BlockedPlayerAddedNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(UserManagerListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_blocked_player_added),
                                    &parsed
                                );
                                let response = self.on_blocked_player_added(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(1u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(UserManagerListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_blocked_player_added),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            2u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <BlockedPlayerRemovedNotification>::default()
                                } else {
                                    <BlockedPlayerRemovedNotification>::decode(
                                        &mut msg.data.clone(),
                                    )?
                                };
                                log::debug!(
                                    target: stringify!(UserManagerListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_blocked_player_removed),
                                    &parsed
                                );
                                let response = self.on_blocked_player_removed(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(2u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(UserManagerListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_blocked_player_removed),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            11u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <RecentPlayersAddedNotification>::default()
                                } else {
                                    <RecentPlayersAddedNotification>::decode(&mut msg.data.clone())?
                                };
                                log::debug!(
                                    target: stringify!(UserManagerListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_recent_players_added),
                                    &parsed
                                );
                                let response = self.on_recent_players_added(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(11u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(UserManagerListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_recent_players_added),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            12u8 => {
                                let parsed = if let Some(0) = msg.headers.size {
                                    <RecentPlayersRemovedNotification>::default()
                                } else {
                                    <RecentPlayersRemovedNotification>::decode(
                                        &mut msg.data.clone(),
                                    )?
                                };
                                log::debug!(
                                    target: stringify!(UserManagerListener),
                                    "[{:?}] Method `{}` called with data: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_recent_players_removed),
                                    &parsed
                                );
                                let response = self.on_recent_players_removed(parsed).await;
                                let mut headers = crate::bgs::protocol::Header::default();
                                headers.method_id = Some(12u8 as u32);
                                headers.service_hash = Some(Self::ORIGINAL_HASH);
                                headers.token = msg.headers.token;
                                log::debug!(
                                    target: stringify!(UserManagerListener),
                                    "[{:?}] Method `{}` response: {:?}",
                                    self.get_client_addr(),
                                    stringify!(on_recent_players_removed),
                                    &response
                                );
                                let mut outoing_message = crate::messages::OutgoingMessage {
                                    headers,
                                    message: Some(response?),
                                };
                                Ok(outoing_message.encode(true))
                            }
                            _ => Err(crate::errors::WowRpcResponse::RpcNotImplemented),
                        }
                    }
                }
            }
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct UnsignedIntRange {
            #[prost(uint64, optional, tag = "1")]
            pub min: ::core::option::Option<u64>,
            #[prost(uint64, optional, tag = "2")]
            pub max: ::core::option::Option<u64>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct SignedIntRange {
            #[prost(int64, optional, tag = "1")]
            pub min: ::core::option::Option<i64>,
            #[prost(int64, optional, tag = "2")]
            pub max: ::core::option::Option<i64>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct FloatRange {
            #[prost(float, optional, tag = "1")]
            pub min: ::core::option::Option<f32>,
            #[prost(float, optional, tag = "2")]
            pub max: ::core::option::Option<f32>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct BgsFieldOptions {
            #[prost(enumeration = "LogOption", optional, tag = "1")]
            pub log: ::core::option::Option<i32>,
            #[prost(bool, optional, tag = "2")]
            pub shard_key: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "3")]
            pub fanout_key: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "4")]
            pub client_instance_key: ::core::option::Option<bool>,
            #[prost(string, optional, tag = "5")]
            pub realized_enum: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct FieldRestriction {
            #[prost(oneof = "field_restriction::Type", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
            pub r#type: ::core::option::Option<field_restriction::Type>,
        }
        #[doc = " Nested message and enum types in `FieldRestriction`."]
        pub mod field_restriction {
            #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Oneof)]
            pub enum Type {
                #[prost(message, tag = "1")]
                Signed(super::SignedFieldRestriction),
                #[prost(message, tag = "2")]
                Unsigned(super::UnsignedFieldRestriction),
                #[prost(message, tag = "3")]
                Float(super::FloatFieldRestriction),
                #[prost(message, tag = "4")]
                String(super::StringFieldRestriction),
                #[prost(message, tag = "5")]
                Repeated(super::RepeatedFieldRestriction),
                #[prost(message, tag = "6")]
                Message(super::MessageFieldRestriction),
                #[prost(message, tag = "7")]
                EntityId(super::EntityIdRestriction),
                #[prost(message, tag = "8")]
                Bytes(super::StringFieldRestriction),
            }
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct RepeatedFieldRestriction {
            #[prost(message, optional, tag = "1")]
            pub size: ::core::option::Option<UnsignedIntRange>,
            #[prost(bool, optional, tag = "2")]
            pub unique: ::core::option::Option<bool>,
            #[prost(oneof = "repeated_field_restriction::Type", tags = "3, 4, 5, 6, 7, 8")]
            pub r#type: ::core::option::Option<repeated_field_restriction::Type>,
        }
        #[doc = " Nested message and enum types in `RepeatedFieldRestriction`."]
        pub mod repeated_field_restriction {
            #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Oneof)]
            pub enum Type {
                #[prost(message, tag = "3")]
                Signed(super::SignedFieldRestriction),
                #[prost(message, tag = "4")]
                Unsigned(super::UnsignedFieldRestriction),
                #[prost(message, tag = "5")]
                Float(super::FloatFieldRestriction),
                #[prost(message, tag = "6")]
                String(super::StringFieldRestriction),
                #[prost(message, tag = "7")]
                EntityId(super::EntityIdRestriction),
                #[prost(message, tag = "8")]
                Bytes(super::StringFieldRestriction),
            }
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct SignedFieldRestriction {
            #[prost(message, optional, tag = "1")]
            pub limits: ::core::option::Option<SignedIntRange>,
            #[prost(sint64, repeated, packed = "false", tag = "2")]
            pub exclude: ::prost::alloc::vec::Vec<i64>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct UnsignedFieldRestriction {
            #[prost(message, optional, tag = "1")]
            pub limits: ::core::option::Option<UnsignedIntRange>,
            #[prost(uint64, repeated, packed = "false", tag = "2")]
            pub exclude: ::prost::alloc::vec::Vec<u64>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct FloatFieldRestriction {
            #[prost(message, optional, tag = "1")]
            pub limits: ::core::option::Option<FloatRange>,
            #[prost(float, repeated, packed = "false", tag = "2")]
            pub exclude: ::prost::alloc::vec::Vec<f32>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct StringFieldRestriction {
            #[prost(message, optional, tag = "1")]
            pub size: ::core::option::Option<UnsignedIntRange>,
            #[prost(string, repeated, tag = "2")]
            pub exclude: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct EntityIdRestriction {
            #[prost(bool, optional, tag = "1")]
            pub needed: ::core::option::Option<bool>,
            #[prost(enumeration = "entity_id_restriction::Kind", optional, tag = "2")]
            pub kind: ::core::option::Option<i32>,
        }
        #[doc = " Nested message and enum types in `EntityIdRestriction`."]
        pub mod entity_id_restriction {
            #[derive(
                serde :: Serialize,
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                :: prost :: Enumeration,
            )]
            #[repr(i32)]
            pub enum Kind {
                Any = 0,
                Account = 1,
                GameAccount = 2,
                AccountOrGameAccount = 3,
                Service = 4,
                Channel = 5,
            }
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct MessageFieldRestriction {
            #[prost(bool, optional, tag = "1")]
            pub needed: ::core::option::Option<bool>,
        }
        #[derive(
            serde :: Serialize,
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            :: prost :: Enumeration,
        )]
        #[repr(i32)]
        pub enum LogOption {
            Hidden = 1,
            Hex = 2,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct BgsMessageOptions {
            #[prost(bool, optional, tag = "1")]
            pub custom_select_shard: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "2")]
            pub custom_validator: ::core::option::Option<bool>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct EntityId {
            #[prost(fixed64, required, tag = "1")]
            pub high: u64,
            #[prost(fixed64, required, tag = "2")]
            pub low: u64,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct Identity {
            #[prost(message, optional, tag = "1")]
            pub account_id: ::core::option::Option<EntityId>,
            #[prost(message, optional, tag = "2")]
            pub game_account_id: ::core::option::Option<EntityId>,
        }
        #[derive(
            serde :: Serialize,
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            :: prost :: Enumeration,
        )]
        #[repr(i32)]
        pub enum ClientIdentityRoutingType {
            ClientIdentityRoutingDisabled = 0,
            ClientIdentityRoutingBattleNetAccount = 1,
            ClientIdentityRoutingGameAccount = 2,
            ClientIdentityRoutingInstanceId = 3,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct BgsMethodOptions {
            #[prost(uint32, optional, tag = "1")]
            pub id: ::core::option::Option<u32>,
            #[prost(enumeration = "ClientIdentityRoutingType", optional, tag = "2")]
            pub client_identity_routing: ::core::option::Option<i32>,
            #[prost(bool, optional, tag = "3")]
            pub enable_fanout: ::core::option::Option<bool>,
            #[prost(string, optional, tag = "4")]
            pub legacy_fanout_replacement: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "5")]
            pub forward_key: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(bool, optional, tag = "6")]
            pub idempotent: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "7")]
            pub handle_destination_unreachable: ::core::option::Option<bool>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct BgsServiceOptions {
            #[prost(string, optional, tag = "1")]
            pub descriptor_name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(uint32, optional, tag = "4")]
            pub version: ::core::option::Option<u32>,
            #[prost(string, optional, tag = "5")]
            pub shard_name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(bool, optional, tag = "6")]
            pub resolve_client_instance: ::core::option::Option<bool>,
            #[prost(enumeration = "bgs_service_options::ServiceType", optional, tag = "7")]
            pub r#type: ::core::option::Option<i32>,
            #[prost(string, optional, tag = "8")]
            pub api_type: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[doc = " Nested message and enum types in `BGSServiceOptions`."]
        pub mod bgs_service_options {
            #[derive(
                serde :: Serialize,
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                :: prost :: Enumeration,
            )]
            #[repr(i32)]
            pub enum ServiceType {
                Rpc = 0,
                Event = 1,
                EventBroadcast = 2,
            }
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct SdkServiceOptions {
            #[prost(bool, optional, tag = "1")]
            pub inbound: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "2")]
            pub outbound: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "3")]
            pub use_client_id: ::core::option::Option<bool>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct NoResponse {}
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct Address {
            #[prost(string, required, tag = "1")]
            pub address: ::prost::alloc::string::String,
            #[prost(uint32, optional, tag = "2")]
            pub port: ::core::option::Option<u32>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct ProcessId {
            #[prost(uint32, required, tag = "1")]
            pub label: u32,
            #[prost(uint32, required, tag = "2")]
            pub epoch: u32,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct ObjectAddress {
            #[prost(message, required, tag = "1")]
            pub host: ProcessId,
            #[prost(uint64, optional, tag = "2", default = "0")]
            pub object_id: ::core::option::Option<u64>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct NoData {}
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct ErrorInfo {
            #[prost(message, required, tag = "1")]
            pub object_address: ObjectAddress,
            #[prost(uint32, required, tag = "2")]
            pub status: u32,
            #[prost(uint32, required, tag = "3")]
            pub service_hash: u32,
            #[prost(uint32, required, tag = "4")]
            pub method_id: u32,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct FanoutTarget {
            #[prost(string, optional, tag = "1")]
            pub client_id: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(bytes = "vec", optional, tag = "2")]
            pub key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            #[prost(uint64, optional, tag = "3")]
            pub object_id: ::core::option::Option<u64>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct Header {
            #[prost(uint32, required, tag = "1")]
            pub service_id: u32,
            #[prost(uint32, optional, tag = "2")]
            pub method_id: ::core::option::Option<u32>,
            #[prost(uint32, required, tag = "3")]
            pub token: u32,
            #[prost(uint64, optional, tag = "4", default = "0")]
            pub object_id: ::core::option::Option<u64>,
            #[prost(uint32, optional, tag = "5", default = "0")]
            pub size: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "6", default = "0")]
            pub status: ::core::option::Option<u32>,
            #[prost(message, repeated, tag = "7")]
            pub error: ::prost::alloc::vec::Vec<ErrorInfo>,
            #[prost(uint64, optional, tag = "8")]
            pub timeout: ::core::option::Option<u64>,
            #[prost(bool, optional, tag = "9")]
            pub is_response: ::core::option::Option<bool>,
            #[prost(message, repeated, tag = "10")]
            pub forward_targets: ::prost::alloc::vec::Vec<ProcessId>,
            #[prost(fixed32, optional, tag = "11")]
            pub service_hash: ::core::option::Option<u32>,
            #[prost(string, optional, tag = "13")]
            pub client_id: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(message, repeated, tag = "14")]
            pub fanout_target: ::prost::alloc::vec::Vec<FanoutTarget>,
            #[prost(string, repeated, tag = "15")]
            pub client_id_fanout_target: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            #[prost(bytes = "vec", optional, tag = "16")]
            pub client_record: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            #[prost(bytes = "vec", optional, tag = "17")]
            pub original_sender: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            #[prost(uint32, optional, tag = "18")]
            pub sender_token: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "19")]
            pub router_label: ::core::option::Option<u32>,
            #[prost(string, optional, tag = "20")]
            pub error_reason: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct KafkaHeader {
            #[prost(fixed32, optional, tag = "1")]
            pub service_hash: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "2")]
            pub method_id: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "3")]
            pub token: ::core::option::Option<u32>,
            #[prost(uint64, optional, tag = "4", default = "0")]
            pub object_id: ::core::option::Option<u64>,
            #[prost(uint32, optional, tag = "5", default = "0")]
            pub size: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "6", default = "0")]
            pub status: ::core::option::Option<u32>,
            #[prost(uint64, optional, tag = "7")]
            pub timeout: ::core::option::Option<u64>,
            #[prost(message, optional, tag = "8")]
            pub forward_target: ::core::option::Option<ProcessId>,
            #[prost(string, optional, tag = "9")]
            pub return_topic: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "11")]
            pub client_id: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct Variant {
            #[prost(bool, optional, tag = "2")]
            pub bool_value: ::core::option::Option<bool>,
            #[prost(int64, optional, tag = "3")]
            pub int_value: ::core::option::Option<i64>,
            #[prost(double, optional, tag = "4")]
            pub float_value: ::core::option::Option<f64>,
            #[prost(string, optional, tag = "5")]
            pub string_value: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(bytes = "vec", optional, tag = "6")]
            pub blob_value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            #[prost(bytes = "vec", optional, tag = "7")]
            pub message_value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            #[prost(string, optional, tag = "8")]
            pub fourcc_value: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(uint64, optional, tag = "9")]
            pub uint_value: ::core::option::Option<u64>,
            #[prost(message, optional, tag = "10")]
            pub entity_id_value: ::core::option::Option<EntityId>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct Attribute {
            #[prost(string, required, tag = "1")]
            pub name: ::prost::alloc::string::String,
            #[prost(message, required, tag = "2")]
            pub value: Variant,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct AttributeFilter {
            #[prost(enumeration = "attribute_filter::Operation", required, tag = "1")]
            pub op: i32,
            #[prost(message, repeated, tag = "2")]
            pub attribute: ::prost::alloc::vec::Vec<Attribute>,
        }
        #[doc = " Nested message and enum types in `AttributeFilter`."]
        pub mod attribute_filter {
            #[derive(
                serde :: Serialize,
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                :: prost :: Enumeration,
            )]
            #[repr(i32)]
            pub enum Operation {
                MatchNone = 0,
                MatchAny = 1,
                MatchAll = 2,
                MatchAllMostSpecific = 3,
            }
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct ContentHandle {
            #[prost(fixed32, required, tag = "1")]
            pub region: u32,
            #[prost(fixed32, required, tag = "2")]
            pub usage: u32,
            #[prost(bytes = "vec", required, tag = "3")]
            pub hash: ::prost::alloc::vec::Vec<u8>,
            #[prost(string, optional, tag = "4")]
            pub proto_url: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct Invitation {
            #[prost(fixed64, required, tag = "1")]
            pub id: u64,
            #[prost(message, required, tag = "2")]
            pub inviter_identity: Identity,
            #[prost(message, required, tag = "3")]
            pub invitee_identity: Identity,
            #[prost(string, optional, tag = "4")]
            pub inviter_name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "5")]
            pub invitee_name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "6")]
            pub invitation_message: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(uint64, optional, tag = "7")]
            pub creation_time: ::core::option::Option<u64>,
            #[prost(uint64, optional, tag = "8")]
            pub expiration_time: ::core::option::Option<u64>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct InvitationParams {
            #[deprecated]
            #[prost(string, optional, tag = "1")]
            pub invitation_message: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(uint64, optional, tag = "2")]
            pub expiration_time: ::core::option::Option<u64>,
        }
        #[derive(
            serde :: Serialize,
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            :: prost :: Enumeration,
        )]
        #[repr(i32)]
        pub enum InvitationRemovedReason {
            Accepted = 0,
            Declined = 1,
            Revoked = 2,
            Ignored = 3,
            Expired = 4,
            Canceled = 5,
        }
        #[derive(
            serde :: Serialize,
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            :: prost :: Enumeration,
        )]
        #[repr(i32)]
        pub enum SuggestionRemovedReason {
            Approved = 0,
            Declined = 1,
            Expired = 2,
            Canceled = 3,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct Role {
            #[prost(uint32, required, tag = "1")]
            pub id: u32,
            #[prost(string, required, tag = "2")]
            pub name: ::prost::alloc::string::String,
            #[prost(string, repeated, tag = "3")]
            pub privilege: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            #[prost(uint32, repeated, tag = "4")]
            pub assignable_role: ::prost::alloc::vec::Vec<u32>,
            #[prost(bool, optional, tag = "5")]
            pub required: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "6")]
            pub unique: ::core::option::Option<bool>,
            #[prost(uint32, optional, tag = "7")]
            pub relegation_role: ::core::option::Option<u32>,
            #[prost(uint32, repeated, tag = "9")]
            pub kickable_role: ::prost::alloc::vec::Vec<u32>,
            #[prost(uint32, repeated, tag = "10")]
            pub removable_role: ::prost::alloc::vec::Vec<u32>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct RoleState {
            #[prost(string, optional, tag = "2")]
            pub name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(uint32, repeated, tag = "4")]
            pub assignable_role: ::prost::alloc::vec::Vec<u32>,
            #[prost(bool, optional, tag = "5")]
            pub required: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "6")]
            pub unique: ::core::option::Option<bool>,
            #[prost(uint32, optional, tag = "7")]
            pub relegation_role: ::core::option::Option<u32>,
            #[prost(uint32, repeated, tag = "9")]
            pub kickable_role: ::prost::alloc::vec::Vec<u32>,
            #[prost(uint32, repeated, tag = "10")]
            pub removable_role: ::prost::alloc::vec::Vec<u32>,
            #[prost(uint32, repeated, tag = "11")]
            pub mentionable_role: ::prost::alloc::vec::Vec<u32>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct EmbedImage {
            #[prost(string, optional, tag = "1")]
            pub url: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(uint32, optional, tag = "2")]
            pub width: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "3")]
            pub height: ::core::option::Option<u32>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct Provider {
            #[prost(string, optional, tag = "1")]
            pub name: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct EmbedHtml {
            #[prost(string, optional, tag = "1")]
            pub content: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(uint32, optional, tag = "2")]
            pub width: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "3")]
            pub height: ::core::option::Option<u32>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct EmbedInfo {
            #[prost(string, optional, tag = "1")]
            pub title: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "2")]
            pub r#type: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "3")]
            pub original_url: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(message, optional, tag = "4")]
            pub thumbnail: ::core::option::Option<EmbedImage>,
            #[prost(message, optional, tag = "5")]
            pub provider: ::core::option::Option<Provider>,
            #[prost(string, optional, tag = "6")]
            pub description: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(message, optional, tag = "8")]
            pub html: ::core::option::Option<EmbedHtml>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct TimeSeriesId {
            #[prost(uint64, optional, tag = "1")]
            pub epoch: ::core::option::Option<u64>,
            #[prost(uint64, optional, tag = "2")]
            pub position: ::core::option::Option<u64>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct GetEventOptions {
            #[prost(uint64, optional, tag = "1")]
            pub fetch_from: ::core::option::Option<u64>,
            #[prost(uint64, optional, tag = "2")]
            pub fetch_until: ::core::option::Option<u64>,
            #[prost(uint32, optional, tag = "3")]
            pub max_events: ::core::option::Option<u32>,
            #[prost(enumeration = "EventOrder", optional, tag = "4")]
            pub order: ::core::option::Option<i32>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct ViewMarker {
            #[prost(uint64, optional, tag = "1")]
            pub last_read_time: ::core::option::Option<u64>,
            #[prost(uint64, optional, tag = "2")]
            pub last_message_time: ::core::option::Option<u64>,
        }
        #[derive(
            serde :: Serialize,
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            :: prost :: Enumeration,
        )]
        #[repr(i32)]
        pub enum EventOrder {
            EventDescending = 0,
            EventAscending = 1,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct MessageId {
            #[prost(uint64, optional, tag = "1")]
            pub epoch: ::core::option::Option<u64>,
            #[prost(uint64, optional, tag = "2")]
            pub position: ::core::option::Option<u64>,
        }
        #[derive(
            serde :: Serialize,
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            :: prost :: Enumeration,
        )]
        #[repr(i32)]
        pub enum TypingIndicator {
            TypingStart = 0,
            TypingStop = 1,
        }
        #[derive(serde :: Serialize, Clone, PartialEq, :: prost :: Message)]
        pub struct VoiceCredentials {
            #[prost(string, optional, tag = "1")]
            pub voice_id: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "2")]
            pub token: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "3")]
            pub url: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(enumeration = "VoiceJoinType", optional, tag = "4")]
            pub join_type: ::core::option::Option<i32>,
            #[prost(enumeration = "VoiceMuteReason", optional, tag = "5")]
            pub mute_reason: ::core::option::Option<i32>,
        }
        #[derive(
            serde :: Serialize,
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            :: prost :: Enumeration,
        )]
        #[repr(i32)]
        pub enum VoiceJoinType {
            VoiceJoinNormal = 0,
            VoiceJoinMuted = 1,
        }
        #[derive(
            serde :: Serialize,
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            :: prost :: Enumeration,
        )]
        #[repr(i32)]
        pub enum VoiceMuteReason {
            None = 0,
            ParentalControlListenOnly = 1,
            Requested = 2,
            Squelched = 3,
        }
        #[derive(
            serde :: Serialize,
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            :: prost :: Enumeration,
        )]
        #[repr(i32)]
        pub enum VoiceProviderVersion {
            VoiceProviderV4 = 0,
            VoiceProviderV5 = 1,
        }
    }
}
