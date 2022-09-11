use crate::classes::Classes;
use crate::guid::ObjectGuid;
use crate::packets::IntoServerPacket;
use crate::races::Races;
use crate::OpcodeServer;
use bytes::Bytes;
use deku::prelude::*;
use enum_iterator::IntoEnumIterator;

#[derive(Debug, DekuWrite)]
#[deku(type = "u8")]
#[repr(u8)]
pub enum CharResponse {
    ListRetrieving = 20,
    ListRetrieved = 21,
    ListFailed = 22,

    CreateInProgress = 23,
    CreateSuccess = 24,
    CreateError = 25,
    CreateFailed = 26,
    CreateNameInUse = 27,
    CreateDisabled = 28,
    CreatePvpTeamsViolation = 29,
    CreateServerLimit = 30,
    CreateAccountLimit = 31,
    CreateServerQueue = 32,
    CreateOnlyExisting = 33,
    CreateExpansion = 34,
    CreateExpansionClass = 35,
    CreateCharacterInGuild = 36,
    CreateRestrictedRaceclass = 37,
    CreateCharacterChooseRace = 38,
    CreateCharacterArenaLeader = 39,
    CreateCharacterDeleteMail = 40,
    CreateCharacterSwapFaction = 41,
    CreateCharacterRaceOnly = 42,
    CreateCharacterGoldLimit = 43,
    CreateForceLogin = 44,
    CreateTrial = 45,
    CreateTimeout = 46,
    CreateThrottle = 47,
    CreateAlliedRaceAchievement = 48,
    CreateCharacterInCommunity = 49,
    CreateNewPlayer = 50,
    CreateNameReservationFull = 51,

    DeleteInProgress = 52,
    DeleteSuccess = 53,
    DeleteFailed = 54,
    DeleteFailedLockedForTransfer = 55,
    DeleteFailedGuildLeader = 56,
    DeleteFailedArenaCaptain = 57,
    DeleteFailedHasHeirloomOrMail = 58,
    DeleteFailedUpgradeInProgress = 59,
    DeleteFailedHasWowToken = 60,
    DeleteFailedVasTransactionInProgress = 61,
    DeleteFailedCommunityOwner = 62,

    LoginInProgress = 63,
    LoginSuccess = 64,
    LoginNoWorld = 65,
    LoginDuplicateCharacter = 66,
    LoginNoInstances = 67,
    LoginFailed = 68,
    LoginDisabled = 69,
    LoginNoCharacter = 70,
    LoginLockedForTransfer = 71,
    LoginLockedByBilling = 72,
    LoginLockedByMobileAh = 73,
    LoginTemporaryGmLock = 74,
    LoginLockedByCharacterUpgrade = 75,
    LoginLockedByRevokedCharacterUpgrade = 76,
    LoginLockedByRevokedVasTransaction = 77,
    LoginLockedByRestriction = 78,
    LoginLockedForRealmPlaytype = 79,

    NameSuccess = 80,
    NameFailure = 81,
    NameNoName = 82,
    NameTooShort = 83,
    NameTooLong = 84,
    NameInvalidCharacter = 85,
    NameMixedLanguages = 86,
    NameProfane = 87,
    NameReserved = 88,
    NameInvalidApostrophe = 89,
    NameMultipleApostrophes = 90,
    NameThreeConsecutive = 91,
    NameInvalidSpace = 92,
    NameConsecutiveSpaces = 93,
    NameRussianConsecutiveSilentCharacters = 94,
    NameRussianSilentCharacterAtBeginningOrEnd = 95,
    NameDeclensionDoesntMatchBaseName = 96,
    NameSpacesDisallowed = 97,
}

#[derive(Debug, DekuWrite, Clone)]
#[deku(endian = "little")]
pub struct VisualItemInfo {
    display_id: u32,
    display_enchant_id: u32,
    secondary_item_modified_appearance_id: i32,
    inv_type: u8,
    subclass: u8,
}

impl VisualItemInfo {
    pub fn new() -> VisualItemInfo {
        VisualItemInfo {
            display_id: 0,
            display_enchant_id: 0,
            secondary_item_modified_appearance_id: 0,
            inv_type: 0,
            subclass: 0,
        }
    }
}

#[derive(Debug, DekuWrite, Clone)]
pub struct CharacterInfo {
    guid: ObjectGuid,
    #[deku(endian = "little")]
    guild_club_member_id: u64,
    list_position: u8,
    race: Races,
    class: Classes,
    is_female: bool,
    #[deku(endian = "little")]
    customizations_size: u32,
    experience_leve: u8,
    #[deku(endian = "little")]
    zone_id: i32,
    #[deku(endian = "little")]
    map_id: i32,
    #[deku(endian = "little")]
    x: f32,
    #[deku(endian = "little")]
    y: f32,
    #[deku(endian = "little")]
    z: f32,
    guild_guid: ObjectGuid,
    #[deku(endian = "little")]
    flags_1: u32,
    #[deku(endian = "little")]
    flags_2: u32,
    #[deku(endian = "little")]
    flags_3: u32,
    #[deku(endian = "little")]
    pet_creature_display_id: u32,
    #[deku(endian = "little")]
    pet_experience_level: u32,
    #[deku(endian = "little")]
    pet_creature_family_id: u32,
    #[deku(endian = "little")]
    primary_profession: u32,
    #[deku(endian = "little")]
    secondary_profession: u32,
    #[deku(count = "23")]
    visual_items: Vec<VisualItemInfo>,
    #[deku(endian = "little")]
    last_played_time: i64,
    #[deku(endian = "little")]
    spec_id: u16,
    #[deku(endian = "little")]
    unk_703: u32,
    #[deku(endian = "little")]
    last_login_version: u32,
    #[deku(endian = "little")]
    flags_4: u32,
    #[deku(endian = "little")]
    mail_senders_size: u32,
    #[deku(endian = "little")]
    mail_sender_types_size: u32,
    #[deku(endian = "little")]
    override_select_screen_file_data_id: u32,
    customizations: Vec<CustomizationChoice>,
    // mail_sender_types: Vec<MailSenderType>,
    #[deku(bits = "6")]
    char_name_len: u8,
    #[deku(bits = "1")]
    first_login: bool,
    #[deku(bits = "1")]
    boost_in_progress: bool,
    #[deku(bits = "5")]
    #[deku(pad_bits_after = "3")]
    unk_wod61x: u8,
    // mail senders names len
    // mail senders
    #[deku(count = "char_name_len")]
    #[deku(writer = "crate::utils::string_writer(deku::output, &self.name)")]
    name: String,
}

impl CharacterInfo {
    pub(crate) fn from_create_request(
        create_request: CreateCharacterRequest,
        list_position: u8,
    ) -> Self {
        CharacterInfo {
            guid: ObjectGuid::new().into(),
            guild_club_member_id: 0,
            list_position,
            race: create_request.race,
            class: create_request.class,
            is_female: create_request.is_female,
            customizations_size: create_request.customizations.len() as u32,
            experience_leve: 0,
            zone_id: 6451,
            map_id: 1,
            x: -662.388,
            y: -4308.26,
            z: 44.1715,
            guild_guid: ObjectGuid::new().into(),
            flags_1: 0,
            flags_2: 0,
            flags_3: 0,
            pet_creature_display_id: 0,
            pet_experience_level: 0,
            pet_creature_family_id: 0,
            primary_profession: 0,
            secondary_profession: 0,
            visual_items: vec![VisualItemInfo::new(); 23],
            last_played_time: 0,
            spec_id: 0,
            unk_703: 0,
            last_login_version: 0,
            flags_4: 0,
            mail_senders_size: 0,
            mail_sender_types_size: 0,
            override_select_screen_file_data_id: 0,
            customizations: create_request.customizations,
            char_name_len: create_request.name.len() as u8,
            first_login: false,
            boost_in_progress: false,
            unk_wod61x: 0,
            name: create_request.name,
        }
    }
}

#[derive(Debug, DekuWrite)]
#[deku(endian = "little")]
pub struct RaceUnlock {
    race_id: i32,
    #[deku(bits = "1")]
    has_expansion: bool,
    #[deku(bits = "1")]
    has_achievement: bool,
    #[deku(bits = "1")]
    #[deku(pad_bits_after = "5")]
    has_heritage_armor: bool,
}

#[derive(Debug, DekuWrite)]
pub struct EnumCharactersResult {
    #[deku(bits = "1")]
    success: bool,
    #[deku(bits = "1")]
    is_deleted_characters: bool,
    #[deku(bits = "1")]
    is_new_player_restriction_skipped: bool,
    #[deku(bits = "1")]
    is_new_player_restricted: bool,
    #[deku(bits = "1")]
    is_new_player: bool,
    #[deku(bits = "1")]
    disabled_classes_mask_has_value: bool,
    #[deku(bits = "1")]
    #[deku(pad_bits_after = "1")]
    is_allied_races_creation_allowed: bool,
    #[deku(endian = "little")]
    characters_len: u32,
    #[deku(endian = "little")]
    max_character_level: i32,
    #[deku(endian = "little")]
    race_unlock_data_len: u32,
    #[deku(endian = "little")]
    unlocked_conditional_appearances_len: u32,
    #[deku(count = "characters_len")]
    characters: Vec<CharacterInfo>,
    #[deku(count = "race_unlock_data_len")]
    race_unlock_data: Vec<RaceUnlock>,
}

impl IntoServerPacket for EnumCharactersResult {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::EnumCharactersResult
    }
}

impl EnumCharactersResult {
    pub fn new(characters: Vec<CharacterInfo>) -> EnumCharactersResult {
        let mut race_unlock_data = vec![];
        for race in Races::into_enum_iter() {
            let race_unlock = RaceUnlock {
                race_id: race as i32,
                has_expansion: true,
                has_achievement: true,
                has_heritage_armor: true,
            };
            race_unlock_data.push(race_unlock);
        }
        EnumCharactersResult {
            success: true,
            is_deleted_characters: false,
            is_new_player_restriction_skipped: false,
            is_new_player_restricted: false,
            is_new_player: true,
            disabled_classes_mask_has_value: false,
            is_allied_races_creation_allowed: true,
            characters_len: characters.len() as u32,
            max_character_level: 0,
            race_unlock_data_len: race_unlock_data.len() as u32,
            unlocked_conditional_appearances_len: 0,
            characters,
            race_unlock_data,
        }
    }
}

#[derive(Debug, DekuWrite)]
#[deku(endian = "little")]
pub struct UndeleteCooldownStatusResponse {
    #[deku(bits = "1")]
    #[deku(pad_bits_after = "7")]
    on_cooldown: bool,
    max_cooldown: u32,
    curr_cooldown: u32,
}

impl UndeleteCooldownStatusResponse {
    pub fn new() -> UndeleteCooldownStatusResponse {
        UndeleteCooldownStatusResponse {
            on_cooldown: false,
            max_cooldown: u32::MAX,
            curr_cooldown: 0,
        }
    }
}

impl IntoServerPacket for UndeleteCooldownStatusResponse {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::UndeleteCooldownStatusResponse
    }
}

#[derive(Debug, DekuRead)]
#[deku(endian = "little")]
pub struct CheckCharacterNameAvailability {
    pub sequence_idx: u32,
    #[deku(bits = "6")]
    #[deku(pad_bits_after = "2")]
    _name_len: u8,
    #[deku(count = "_name_len")]
    #[deku(map = "crate::utils::parse_string")]
    pub name: String,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct CustomizationChoice {
    option_id: u32,
    choice_id: u32,
}

#[derive(Debug, DekuRead)]
#[deku(endian = "little")]
pub struct CreateInfo {
    template_set: Option<u32>,
}

#[derive(Debug, DekuRead)]
pub struct CreateCharacterRequest {
    #[deku(bits = "6")]
    _name_len: u8,
    #[deku(bits = "1")]
    has_template_set: bool,
    #[deku(bits = "1")]
    pub is_trial_boost: bool,
    #[deku(bits = "1")]
    #[deku(pad_bits_after = "7")]
    pub is_use_npe: bool,
    pub race: Races,
    pub class: Classes,
    pub is_female: bool,

    #[deku(endian = "little")]
    _customization_count: u32,

    #[deku(count = "_name_len")]
    #[deku(map = "crate::utils::parse_string")]
    pub name: String,

    #[deku(endian = "little")]
    #[deku(cond = "*has_template_set")]
    pub template_set: Option<u32>,

    #[deku(count = "_customization_count")]
    pub customizations: Vec<CustomizationChoice>,
}

#[derive(Debug, DekuWrite)]
pub struct CreateCharacterResponse {
    result: CharResponse,
    guid: Option<ObjectGuid>,
}

impl CreateCharacterResponse {
    pub fn new(result: CharResponse, guid: Option<ObjectGuid>) -> CreateCharacterResponse {
        CreateCharacterResponse {
            result,
            guid: guid.map(|guid| guid.into()),
        }
    }
}

impl IntoServerPacket for CreateCharacterResponse {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::CreateChar
    }
}

#[derive(Debug, DekuRead)]
pub struct ReorderCharacters {
    // guid: ObjectGuid,
    // new_pos: u8,
}

#[derive(Debug, DekuWrite)]
#[deku(endian = "little")]
pub struct LoginVerifyWorld {
    map_id: i32,
    x: f32,
    y: f32,
    z: f32,
    reason_id: u32,
}

impl LoginVerifyWorld {
    pub fn new() -> LoginVerifyWorld {
        LoginVerifyWorld {
            map_id: 1,
            x: -662.388,
            y: -4308.26,
            z: 44.1715,
            reason_id: 0,
        }
    }
}

impl IntoServerPacket for LoginVerifyWorld {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::LoginVerifyWorld
    }
}

#[derive(Debug, DekuWrite)]
pub struct AccountDataTimes {
    player_guid: ObjectGuid,
    #[deku(endian = "little")]
    last_played_time: i64,
}

impl AccountDataTimes {
    pub fn new(player_guid: ObjectGuid) -> AccountDataTimes {
        AccountDataTimes {
            player_guid: player_guid.into(),
            last_played_time: 0,
        }
    }
}

impl IntoServerPacket for AccountDataTimes {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::AccountDataTimes
    }
}

#[derive(Debug, DekuWrite)]
#[deku(endian = "little")]
pub struct ServerTimeOffset {
    time: i64,
}

impl ServerTimeOffset {
    pub fn new() -> ServerTimeOffset {
        ServerTimeOffset { time: 0 }
    }
}

impl IntoServerPacket for ServerTimeOffset {
    fn get_opcode(&self) -> OpcodeServer {
        OpcodeServer::ServerTimeOffset
    }
}
