pub mod json {
    pub mod realm_list {
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RealmListTicketIdentity {
            pub game_account_id: u32,
            pub game_account_region: u32,
        }
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClientVersion {
            pub version_major: u32,
            pub version_minor: u32,
            pub version_revision: u32,
            pub version_build: u32,
        }
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClientInformation {
            pub platform: u32,
            pub build_variant: String,
            pub r#type: u32,
            pub time_zone: String,
            pub current_time: u32,
            pub text_locale: u32,
            pub audio_locale: u32,
            pub version_data_build: u32,
            pub version: ClientVersion,
            pub secret: Vec<u8>,
            pub client_arch: u32,
            pub system_version: String,
            pub platform_type: u32,
            pub system_arch: u32,
        }
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RealmListTicketClientInformation {
            pub info: ClientInformation,
        }
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RealmCharacterCountEntry {
            pub wow_realm_address: u32,
            pub count: u32,
        }
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RealmCharacterCountList {
            pub counts: Vec<RealmCharacterCountEntry>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RealmEntry {
            pub wow_realm_address: u32,
            pub cfg_timezones_id: u32,
            pub population_state: u32,
            pub cfg_categories_id: u32,
            pub version: ClientVersion,
            pub cfg_realms_id: u32,
            pub flags: u32,
            pub name: String,
            pub cfg_configs_id: u32,
            pub cfg_languages_id: u32,
        }
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RealmState {
            pub update: Option<RealmEntry>,
            pub deleting: bool,
        }
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RealmListUpdates {
            pub updates: Vec<RealmState>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct IpAddress {
            pub ip: String,
            pub port: u32,
        }
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RealmIpAddressFamily {
            pub family: u32,
            pub addresses: Vec<IpAddress>,
        }
        #[derive(serde :: Serialize, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RealmListServerIpAddresses {
            pub families: Vec<RealmIpAddressFamily>,
        }
    }
}
