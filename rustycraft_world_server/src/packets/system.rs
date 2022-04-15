use crate::packets::IntoServerPacket;
use crate::OpcodeServer;
use deku::prelude::*;
use rustycraft_protocol::expansions::Expansions;

#[derive(Debug, DekuWrite)]
pub struct SetTimeZoneInformation {
    #[deku(bits = "7")]
    server_tz_len: u8,
    #[deku(bits = "7")]
    #[deku(pad_bits_after = "2")]
    game_tz_len: u8,
    #[deku(count = "server_time_tz_len")]
    #[deku(writer = "crate::utils::string_writer(deku::output, &self.server_tz)")]
    server_tz: String,
    #[deku(count = "game_time_tz_len")]
    #[deku(writer = "crate::utils::string_writer(deku::output, &self.game_tz)")]
    game_tz: String,
}

impl SetTimeZoneInformation {
    pub fn default() -> SetTimeZoneInformation {
        let server_tz = "Europe/Paris".to_owned();
        let game_tz = "Europe/Paris".to_owned();
        SetTimeZoneInformation {
            server_tz_len: server_tz.len() as u8,
            game_tz_len: game_tz.len() as u8,
            server_tz,
            game_tz,
        }
    }
}

impl IntoServerPacket for SetTimeZoneInformation {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::SetTimeZoneInformation
    }
}

#[derive(Debug, DekuWrite)]
#[deku(endian = "little")]
pub struct SavedThrottleObjectState {
    max_tries: u32,
    per_milliseconds: u32,
    try_count: u32,
    last_reset_time_before_now: u32,
}

#[derive(Debug, DekuWrite)]
pub struct EuropaTicketConfig {
    #[deku(bits = "1")]
    tickets_enabled: bool,
    #[deku(bits = "1")]
    bugs_enabled: bool,
    #[deku(bits = "1")]
    complaints_enabled: bool,
    #[deku(bits = "1")]
    #[deku(pad_bits_after = "4")]
    suggestions_enabled: bool,
    throttle_state: SavedThrottleObjectState,
}

#[derive(Debug, DekuWrite)]
#[deku(endian = "little")]
pub struct GameRuleValuePair {
    rule: i32,
    value: i32,
}

#[derive(Debug, DekuWrite)]
pub struct FeatureSystemStatusGlueScreen {
    #[deku(bits = "1")]
    blizz_pay_store_enabled: bool,
    #[deku(bits = "1")]
    blizz_pay_store_available: bool,
    #[deku(bits = "1")]
    blizz_pay_store_disabled_by_parental_control: bool,
    #[deku(bits = "1")]
    char_undelete_enabled: bool,
    #[deku(bits = "1")]
    commerce_system_enabled: bool,
    #[deku(bits = "1")]
    unk: bool,
    #[deku(bits = "1")]
    will_kick_from_world: bool,
    #[deku(bits = "1")]
    is_expansion_preorder_in_store: bool,
    #[deku(bits = "1")]
    kiosk_mode_enabled: bool,
    #[deku(bits = "1")]
    competitive_mode_enabled: bool,
    #[deku(bits = "1")]
    trial_boost_enabled: bool,
    #[deku(bits = "1")]
    token_balance_enabled: bool,
    #[deku(bits = "1")]
    live_region_character_list_enabled: bool,
    #[deku(bits = "1")]
    live_region_character_copy_enabled: bool,
    #[deku(bits = "1")]
    live_region_account_copy_enabled: bool,
    #[deku(bits = "1")]
    live_region_key_bindings_copy_enabled: bool,
    #[deku(bits = "1")]
    unk_901: bool,
    #[deku(bits = "1")]
    #[deku(pad_bits_after = "6")]
    europa_ticket_status: bool,

    europa_ticket_config: Option<EuropaTicketConfig>,

    #[deku(endian = "little")]
    token_poll_time_seconds: u32,
    #[deku(endian = "little")]
    kiosk_session_minutes: u32,
    #[deku(endian = "little")]
    token_balance_amount: i64,
    #[deku(endian = "little")]
    max_chars_per_realm: i32,

    #[deku(endian = "little")]
    vec_govna_size: u32, // TODO: Rename
    #[deku(endian = "little")]
    blizz_pay_store_products_delivery_delay: i32,
    #[deku(endian = "little")]
    active_chars_upgrade_boost_size: i32,
    #[deku(endian = "little")]
    min_expansion_level: i32,
    #[deku(endian = "little")]
    max_expansion_level: i32,
    #[deku(endian = "little")]
    game_rule_unk: i32,
    #[deku(endian = "little")]
    game_rule_vals_size: u32,
    #[deku(endian = "little")]
    max_player_name_queries_per_packet: i16,
    vec_govna: Vec<i32>,
    game_rule_vals: Vec<GameRuleValuePair>,
}

impl IntoServerPacket for FeatureSystemStatusGlueScreen {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::FeatureSystemStatusGlueScreen
    }
}

impl FeatureSystemStatusGlueScreen {
    pub fn new() -> FeatureSystemStatusGlueScreen {
        FeatureSystemStatusGlueScreen {
            blizz_pay_store_enabled: false,
            blizz_pay_store_available: false,
            blizz_pay_store_disabled_by_parental_control: false,
            char_undelete_enabled: false,
            commerce_system_enabled: false,
            unk: false,
            will_kick_from_world: false,
            is_expansion_preorder_in_store: false,
            kiosk_mode_enabled: false,
            competitive_mode_enabled: false,
            trial_boost_enabled: false,
            token_balance_enabled: false,
            live_region_character_list_enabled: false,
            live_region_character_copy_enabled: false,
            live_region_account_copy_enabled: false,
            live_region_key_bindings_copy_enabled: false,
            unk_901: false,
            europa_ticket_status: false,
            europa_ticket_config: None,
            token_poll_time_seconds: 5,
            kiosk_session_minutes: 5,
            token_balance_amount: 5,
            max_chars_per_realm: 5,
            vec_govna_size: 1000,
            blizz_pay_store_products_delivery_delay: 5,
            active_chars_upgrade_boost_size: 5,
            min_expansion_level: Expansions::ExpansionClassic as i32,
            max_expansion_level: Expansions::ExpansionShadowlands as i32,
            game_rule_unk: 5,
            game_rule_vals_size: 0,
            max_player_name_queries_per_packet: 50,
            vec_govna: vec![],
            game_rule_vals: vec![],
        }
    }
}
