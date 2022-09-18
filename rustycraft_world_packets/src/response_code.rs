use deku::prelude::*;

#[derive(Debug, DekuWrite)]
#[deku(type = "u8")]
pub enum ResponseCode {
    ResponseSuccess = 0,
    ResponseFailure = 1,
    ResponseCancelled = 2,
    ResponseDisconnected = 3,
    ResponseFailedToConnect = 4,
    ResponseConnected = 5,
    ResponseVersionMismatch = 6,
    ClientStatusConnecting = 7,
    ClientStatusNegotiatingSecurity = 8,
    ClientStatusNegotiationComplete = 9,
    ClientStatusNegotiationFailed = 10,
    ClientStatusAuthenticating = 11,
    AuthOk = 12,
    AuthFailed = 13,
    AuthReject = 14,
    AuthBadServerProof = 15,
    AuthUnavailable = 16,
    AuthSystemError = 17,
    AuthBillingError = 18,
    AuthBillingExpired = 19,
    AuthVersionMismatch = 20,
    AuthUnknownAccount = 21,
    AuthIncorrectPassword = 22,
    AuthSessionExpired = 23,
    AuthServerShuttingDown = 24,
    AuthAlreadyLoggingIn = 25,
    AuthLoginServerNotFound = 26,
    AuthWaitQueue = 27,
    AuthBanned = 28,
    AuthAlreadyOnline = 29,
    AuthNoTime = 30,
    AuthDbBusy = 31,
    AuthSuspended = 32,
    AuthParentalControl = 33,
    AuthLockedEnforced = 34,
    RealmListInProgress = 35,
    RealmListSuccess = 36,
    RealmListFailed = 37,
    RealmListInvalid = 38,
    RealmListRealmNotFound = 39,
    AccountCreateInProgress = 40,
    AccountCreateSuccess = 41,
    AccountCreateFailed = 42,
    CharListRetrieving = 43,
    CharListRetrieved = 44,
    CharListFailed = 45,
    CharCreateInProgress = 46,
    CharCreateSuccess = 47,
    CharCreateError = 48,
    CharCreateFailed = 49,
    CharCreateNameInUse = 50,
    CharCreateDisabled = 51,
    CharCreatePvpTeamsViolation = 52,
    CharCreateServerLimit = 53,
    CharCreateAccountLimit = 54,
    CharCreateServerQueue = 55,
    CharCreateOnlyExisting = 56,
    CharCreateExpansion = 57,
    CharCreateExpansionClass = 58,
    CharCreateLevelRequirement = 59,
    CharCreateUniqueClassLimit = 60,
    CharCreateCharacterInGuild = 61,
    CharCreateRestrictedRaceClass = 62,
    CharCreateCharacterChooseRace = 63,
    CharCreateCharacterArenaLeader = 64,
    CharCreateCharacterDeleteMail = 65,
    CharCreateCharacterSwapFaction = 66,
    CharCreateCharacterRaceOnly = 67,
    CharCreateCharacterGoldLimit = 68,
    CharCreateForceLogin = 69,
    CharDeleteInProgress = 70,
    CharDeleteSuccess = 71,
    CharDeleteFailed = 72,
    CharDeleteFailedLockedForTransfer = 73,
    CharDeleteFailedGuildLeader = 74,
    CharDeleteFailedArenaCaptain = 75,
    CharLoginInProgress = 76,
    CharLoginSuccess = 77,
    CharLoginNoWorld = 78,
    CharLoginDuplicateCharacter = 79,
    CharLoginNoInstances = 80,
    CharLoginFailed = 81,
    CharLoginDisabled = 82,
    CharLoginNoCharacter = 83,
    CharLoginLockedForTransfer = 84,
    CharLoginLockedByBilling = 85,
    CharLoginLockedByMobileAh = 86,
    CharNameSuccess = 87,
    CharNameFailure = 88,
    CharNameNoName = 89,
    CharNameTooShort = 90,
    CharNameTooLong = 91,
    CharNameInvalidCharacter = 92,
    CharNameMixedLanguages = 93,
    CharNameProfane = 94,
    CharNameReserved = 95,
    CharNameInvalidApostrophe = 96,
    CharNameMultipleApostrophes = 97,
    CharNameThreeConsecutive = 98,
    CharNameInvalidSpace = 99,
    CharNameConsecutiveSpaces = 100,
    CharNameRussianConsecutiveSilentCharacters = 101,
    CharNameRussianSilentCharacterAtBeginningOrEnd = 102,
    CharNameDeclensionDoesntMatchBaseName = 103,
}
