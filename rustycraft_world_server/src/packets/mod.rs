use crate::opcodes::OpcodeClient;
use crate::packets::auth::{AuthContinuedSession, AuthSession, Ping};
use crate::packets::characters::{
    CheckCharacterNameAvailability, CreateCharacterRequest, ReorderCharacters,
};
use crate::OpcodeServer;
use bytes::{Bytes, BytesMut};
use deku::bitvec::{BitVec, Msb0};
use deku::prelude::*;
use std::fmt::Debug;
use std::mem::size_of_val;

pub mod auth;
pub mod characters;
pub mod client_config;
pub mod objects;
pub mod system;

fn write(output: &mut BitVec<Msb0, u8>, packet_size: u32) -> Result<(), DekuError> {
    packet_size.write(output, ())
}

#[derive(Debug, DekuWrite, DekuRead)]
pub struct PacketHeader {
    pub size: u32,
    #[deku(count = "12")]
    pub tag: Vec<u8>,
}

#[derive(Debug, DekuWrite)]
pub struct ServerPacket {
    header: PacketHeader,
    payload: Vec<u8>,
}

impl ServerPacket {
    pub fn new(aes_tag: Vec<u8>, payload: Bytes) -> ServerPacket {
        assert_eq!(aes_tag.len(), 12);
        ServerPacket {
            header: PacketHeader {
                size: payload.len() as u32,
                tag: aes_tag,
            },
            payload: payload.to_vec(),
        }
    }
    pub fn serialize(&self) -> Result<Bytes, deku::DekuError> {
        self.to_bytes().map(Bytes::from)
    }
}

#[derive(Debug)]
pub struct RawClientPacket {
    pub header: PacketHeader,
    pub payload: Bytes,
}

impl TryFrom<Vec<u8>> for ClientPacket {
    type Error = anyhow::Error;
    fn try_from(data: Vec<u8>) -> anyhow::Result<Self> {
        let (_, opcode): (_, OpcodeClient) = OpcodeClient::from_bytes((&data, 0))?;
        let rest_data = (data.as_slice(), 16);
        Ok(match opcode {
            OpcodeClient::Ping => ClientPacket::Ping(Ping::from_bytes(rest_data)?.1),
            OpcodeClient::AuthSession => {
                ClientPacket::AuthSession(AuthSession::from_bytes(rest_data)?.1)
            }
            OpcodeClient::EnterEncryptedModeAck => ClientPacket::EnterEncryptedModeAck,
            OpcodeClient::LogDisconnect => ClientPacket::LogDisconnect,
            OpcodeClient::EnumCharacters => ClientPacket::EnumCharacters,
            OpcodeClient::GetUndeleteCharacterCooldownStatus => {
                ClientPacket::GetUndeleteCharacterCooldownStatus
            }
            OpcodeClient::BattlePayGetPurchaseList => ClientPacket::BattlePayGetPurchaseList,
            OpcodeClient::BattlePayGetProductList => ClientPacket::BattlePayGetProductList,
            OpcodeClient::UpdateVasPurchaseStates => ClientPacket::UpdateVasPurchaseStates,
            OpcodeClient::ServerTimeOffsetRequest => ClientPacket::ServerTimeOffsetRequest,
            OpcodeClient::PlayerLogin => ClientPacket::PlayerLogin,
            OpcodeClient::ReorderCharacters => {
                ClientPacket::ReorderCharacters(ReorderCharacters::from_bytes(rest_data)?.1)
            }
            OpcodeClient::KeepAlive => ClientPacket::KeepAlive,
            OpcodeClient::CheckCharacterNameAvailability => {
                ClientPacket::CheckCharacterNameAvailability(
                    CheckCharacterNameAvailability::from_bytes(rest_data)?.1,
                )
            }
            OpcodeClient::CreateCharacter => {
                ClientPacket::CreateCharacter(CreateCharacterRequest::from_bytes(rest_data)?.1)
            }
            OpcodeClient::AuthContinuedSession => {
                ClientPacket::AuthContinuedSession(AuthContinuedSession::from_bytes(rest_data)?.1)
            }
            _ => {
                error!(
                    "Unhandled opcode: {:?}, payload: {:X}",
                    opcode,
                    &Bytes::from(data)
                );
                Err(anyhow!("Unhandled opcode"))?
            }
        })
    }
}

pub trait IntoServerPacket: DekuContainerWrite + Debug + Send + Sync {
    fn get_opcode(&self) -> OpcodeServer;
    fn serialize(&self) -> Result<Bytes, DekuError> {
        let mut buf = BytesMut::with_capacity(2 + size_of_val(&self));
        buf.extend(self.get_opcode().to_bytes()?);
        buf.extend(self.to_bytes()?);
        Ok(buf.into())
    }
}

#[derive(Debug)]
pub enum ClientPacket {
    AbandonNpeResponse,
    AcceptGuildInvite,
    AcceptTrade,
    AcceptWargameInvite,
    AccountNotificationAcknowledged,
    ActivateSoulbind,
    ActivateTaxi,
    AddonList,
    AddAccountCosmetic,
    AddBattlenetFriend,
    AddFriend,
    AddIgnore,
    AddToy,
    AdventureJournalOpenQuest,
    AdventureJournalUpdateSuggestions,
    AdventureMapStartQuest,
    AlterAppearance,
    AreaSpiritHealerQuery,
    AreaSpiritHealerQueue,
    AreaTrigger,
    ArtifactAddPower,
    ArtifactSetAppearance,
    AssignEquipmentSetSpec,
    AttackStop,
    AttackSwing,
    AuctionableTokenSell,
    AuctionableTokenSellAtMarketPrice,
    AuctionBrowseQuery,
    AuctionCancelCommoditiesPurchase,
    AuctionConfirmCommoditiesPurchase,
    AuctionGetCommodityQuote,
    AuctionHelloRequest,
    AuctionListBiddedItems,
    AuctionListBucketsByBucketKeys,
    AuctionListItemsByBucketKey,
    AuctionListItemsByItemId,
    AuctionListOwnedItems,
    AuctionPlaceBid,
    AuctionRemoveItem,
    AuctionReplicateItems,
    AuctionSellCommodity,
    AuctionSellItem,
    AuctionSetFavoriteItem,
    AuthContinuedSession(AuthContinuedSession),
    AuthSession(AuthSession),
    AutobankItem,
    AutobankReagent,
    AutostoreBankItem,
    AutostoreBankReagent,
    AutoEquipItem,
    AutoEquipItemSlot,
    AutoGuildBankItem,
    AutoStoreBagItem,
    AutoStoreGuildBankItem,
    AzeriteEmpoweredItemSelectPower,
    AzeriteEmpoweredItemViewed,
    AzeriteEssenceActivateEssence,
    AzeriteEssenceUnlockMilestone,
    BankerActivate,
    BattlefieldLeave,
    BattlefieldList,
    BattlefieldPort,
    BattlemasterHello,
    BattlemasterJoin,
    BattlemasterJoinArena,
    BattlemasterJoinBrawl,
    BattlemasterJoinSkirmish,
    BattlenetChallengeResponse,
    BattlenetRequest,
    BattlePayAckFailedResponse,
    BattlePayCancelOpenCheckout,
    BattlePayConfirmPurchaseResponse,
    BattlePayDistributionAssignToTarget,
    BattlePayDistributionAssignVas,
    BattlePayGetProductList,
    BattlePayGetPurchaseList,
    BattlePayOpenCheckout,
    BattlePayRequestPriceInfo,
    BattlePayStartPurchase,
    BattlePayStartVasPurchase,
    BattlePetClearFanfare,
    BattlePetDeletePet,
    BattlePetDeletePetCheat,
    BattlePetModifyName,
    BattlePetRequestJournal,
    BattlePetRequestJournalLock,
    BattlePetSetBattleSlot,
    BattlePetSetFlags,
    BattlePetSummon,
    BattlePetUpdateDisplayNotify,
    BattlePetUpdateNotify,
    BeginTrade,
    BinderActivate,
    BlackMarketBidOnItem,
    BlackMarketOpen,
    BlackMarketRequestItems,
    BonusRoll,
    BugReport,
    BusyTrade,
    BuyBackItem,
    BuyBankSlot,
    BuyItem,
    BuyReagentBank,
    CageBattlePet,
    CalendarAddEvent,
    CalendarCommunityInvite,
    CalendarComplain,
    CalendarCopyEvent,
    CalendarEventSignUp,
    CalendarGet,
    CalendarGetEvent,
    CalendarGetNumPending,
    CalendarInvite,
    CalendarModeratorStatus,
    CalendarRemoveEvent,
    CalendarRemoveInvite,
    CalendarRsvp,
    CalendarStatus,
    CalendarUpdateEvent,
    CancelAura,
    CancelAutoRepeatSpell,
    CancelCast,
    CancelChannelling,
    CancelGrowthAura,
    CancelMasterLootRoll,
    CancelModSpeedNoControlAuras,
    CancelMountAura,
    CancelQueuedSpell,
    CancelTempEnchantment,
    CancelTrade,
    CanDuel,
    CanRedeemTokenForBalance,
    CastSpell,
    ChallengeModeRequestLeaders,
    ChangeBagSlotFlag,
    ChangeBankBagSlotFlag,
    ChangeMonumentAppearance,
    ChangeRealmTicket,
    ChangeSubGroup,
    CharacterCheckUpgrade,
    CharacterRenameRequest,
    CharacterUpgradeManualUnrevokeRequest,
    CharacterUpgradeStart,
    CharCustomize,
    CharDelete,
    CharRaceOrFactionChange,
    ChatAddonMessage,
    ChatAddonMessageTargeted,
    ChatChannelAnnouncements,
    ChatChannelBan,
    ChatChannelDeclineInvite,
    ChatChannelDisplayList,
    ChatChannelInvite,
    ChatChannelKick,
    ChatChannelList,
    ChatChannelModerator,
    ChatChannelOwner,
    ChatChannelPassword,
    ChatChannelSetOwner,
    ChatChannelSilenceAll,
    ChatChannelUnban,
    ChatChannelUnmoderator,
    ChatChannelUnsilenceAll,
    ChatJoinChannel,
    ChatLeaveChannel,
    ChatMessageAfk,
    ChatMessageChannel,
    ChatMessageDnd,
    ChatMessageEmote,
    ChatMessageGuild,
    ChatMessageInstanceChat,
    ChatMessageOfficer,
    ChatMessageParty,
    ChatMessageRaid,
    ChatMessageRaidWarning,
    ChatMessageSay,
    ChatMessageWhisper,
    ChatMessageYell,
    ChatRegisterAddonPrefixes,
    ChatReportFiltered,
    ChatReportIgnored,
    ChatUnregisterAllAddonPrefixes,
    CheckCharacterNameAvailability(CheckCharacterNameAvailability),
    CheckIsAdventureMapPoiValid,
    ChoiceResponse,
    ChromieTimeSelectExpansion,
    ClaimWeeklyReward,
    ClearNewAppearance,
    ClearRaidMarker,
    ClearTradeItem,
    ClientPortGraveyard,
    CloseInteraction,
    CloseQuestChoice,
    CloseRuneforgeInteraction,
    ClubFinderApplicationResponse,
    ClubFinderGetApplicantsList,
    ClubFinderPost,
    ClubFinderRequestClubsData,
    ClubFinderRequestClubsList,
    ClubFinderRequestMembershipToClub,
    ClubFinderRequestPendingClubsList,
    ClubFinderRequestSubscribedClubPostingIds,
    ClubFinderRespondToApplicant,
    ClubPresenceSubscribe,
    CollectionItemSetFavorite,
    CommentatorEnable,
    CommentatorEnterInstance,
    CommentatorExitInstance,
    CommentatorGetMapInfo,
    CommentatorGetPlayerCooldowns,
    CommentatorGetPlayerInfo,
    CommentatorSpectate,
    CommentatorStartWargame,
    CommerceTokenGetCount,
    CommerceTokenGetLog,
    CommerceTokenGetMarketPrice,
    Complaint,
    CompleteCinematic,
    CompleteMovie,
    ConfirmArtifactRespec,
    ConfirmRespecWipe,
    ConnectToFailed,
    ConsumableTokenBuy,
    ConsumableTokenBuyAtMarketPrice,
    ConsumableTokenCanVeteranBuy,
    ConsumableTokenRedeem,
    ConsumableTokenRedeemConfirmation,
    ContributionContribute,
    ContributionLastUpdateRequest,
    ConversationCinematicReady,
    ConversationLineStarted,
    ConvertRaid,
    CovenantRenownRequestCatchupState,
    CreateCharacter(CreateCharacterRequest),
    CreateShipment,
    DbQueryBulk,
    DeclineGuildInvites,
    DeclinePetition,
    DeleteEquipmentSet,
    DelFriend,
    DelIgnore,
    DepositReagentBank,
    DestroyItem,
    DfBootPlayerVote,
    DfConfirmExpandSearch,
    DfGetJoinStatus,
    DfGetSystemInfo,
    DfJoin,
    DfLeave,
    DfProposalResponse,
    DfReadyCheckResponse,
    DfSetRoles,
    DfTeleport,
    DiscardedTimeSyncAcks,
    DismissCritter,
    DoCountdown,
    DoMasterLootRoll,
    DoReadyCheck,
    DuelResponse,
    EjectPassenger,
    Emote,
    EnableNagle,
    EnableTaxiNode,
    EngineSurvey,
    EnterEncryptedModeAck,
    EnumCharacters,
    EnumCharactersDeletedByClient,
    FarSight,
    GameEventDebugDisable,
    GameEventDebugEnable,
    GameObjReportUse,
    GameObjUse,
    GarrisonAddFollowerHealth,
    GarrisonAssignFollowerToBuilding,
    GarrisonCancelConstruction,
    GarrisonCheckUpgradeable,
    GarrisonCompleteMission,
    GarrisonFullyHealAllFollowers,
    GarrisonGenerateRecruits,
    GarrisonGetClassSpecCategoryInfo,
    GarrisonGetMapData,
    GarrisonGetMissionReward,
    GarrisonLearnTalent,
    GarrisonMissionBonusRoll,
    GarrisonPurchaseBuilding,
    GarrisonRecruitFollower,
    GarrisonRemoveFollower,
    GarrisonRemoveFollowerFromBuilding,
    GarrisonRenameFollower,
    GarrisonRequestBlueprintAndSpecializationData,
    GarrisonRequestShipmentInfo,
    GarrisonResearchTalent,
    GarrisonSetBuildingActive,
    GarrisonSetFollowerFavorite,
    GarrisonSetFollowerInactive,
    GarrisonSetRecruitmentPreferences,
    GarrisonSocketTalent,
    GarrisonStartMission,
    GarrisonSwapBuildings,
    GenerateRandomCharacterName,
    GetAccountCharacterList,
    GetAccountNotifications,
    GetGarrisonInfo,
    GetItemPurchaseData,
    GetLandingPageShipments,
    GetMirrorImageData,
    GetPvpOptionsEnabled,
    GetRafAccountInfo,
    GetRemainingGameTime,
    GetTrophyList,
    GetUndeleteCharacterCooldownStatus,
    GetVasAccountCharacterList,
    GetVasTransferTargetRealmList,
    GmTicketAcknowledgeSurvey,
    GmTicketGetCaseStatus,
    GmTicketGetSystemStatus,
    GossipRefreshOptions,
    GossipSelectOption,
    GuildAddBattlenetFriend,
    GuildAddRank,
    GuildAssignMemberRank,
    GuildAutoDeclineInvitation,
    GuildBankActivate,
    GuildBankBuyTab,
    GuildBankDepositMoney,
    GuildBankLogQuery,
    GuildBankQueryTab,
    GuildBankRemainingWithdrawMoneyQuery,
    GuildBankSetTabText,
    GuildBankTextQuery,
    GuildBankUpdateTab,
    GuildBankWithdrawMoney,
    GuildChallengeUpdateRequest,
    GuildChangeNameRequest,
    GuildDeclineInvitation,
    GuildDelete,
    GuildDeleteRank,
    GuildDemoteMember,
    GuildEventLogQuery,
    GuildGetAchievementMembers,
    GuildGetRanks,
    GuildGetRoster,
    GuildInviteByName,
    GuildLeave,
    GuildNewsUpdateSticky,
    GuildOfficerRemoveMember,
    GuildPermissionsQuery,
    GuildPromoteMember,
    GuildQueryMembersForRecipe,
    GuildQueryMemberRecipes,
    GuildQueryNews,
    GuildQueryRecipes,
    GuildReplaceGuildMaster,
    GuildSetAchievementTracking,
    GuildSetFocusedAchievement,
    GuildSetGuildMaster,
    GuildSetMemberNote,
    GuildSetRankPermissions,
    GuildShiftRank,
    GuildUpdateInfoText,
    GuildUpdateMotdText,
    HearthAndResurrect,
    HideQuestChoice,
    HotfixRequest,
    IgnoreTrade,
    InitiateRolePoll,
    InitiateTrade,
    Inspect,
    InstanceLockResponse,
    IslandQueue,
    ItemPurchaseRefund,
    ItemTextQuery,
    JoinPetBattleQueue,
    JoinRatedBattleground,
    KeepAlive,
    KeyboundOverride,
    LatencyReport,
    LearnPvpTalents,
    LearnTalents,
    LeaveGroup,
    LeavePetBattleQueue,
    LfgListApplyToGroup,
    LfgListCancelApplication,
    LfgListDeclineApplicant,
    LfgListGetStatus,
    LfgListInviteApplicant,
    LfgListInviteResponse,
    LfgListJoin,
    LfgListLeave,
    LfgListSearch,
    LfgListUpdateRequest,
    ListInventory,
    LiveRegionAccountRestore,
    LiveRegionCharacterCopy,
    LiveRegionGetAccountCharacterList,
    LiveRegionKeyBindingsCopy,
    LoadingScreenNotify,
    LoadSelectedTrophy,
    LogoutCancel,
    LogoutInstant,
    LogoutRequest,
    LogDisconnect,
    LogStreamingError,
    LootItem,
    LootMoney,
    LootRelease,
    LootRoll,
    LootUnit,
    LowLevelRaid1,
    LowLevelRaid2,
    MailCreateTextItem,
    MailDelete,
    MailGetList,
    MailMarkAsRead,
    MailReturnToSender,
    MailTakeItem,
    MailTakeMoney,
    MakeContitionalAppearancePermanent,
    MasterLootItem,
    MergeGuildBankItemWithGuildBankItem,
    MergeGuildBankItemWithItem,
    MergeItemWithGuildBankItem,
    MinimapPing,
    MissileTrajectoryCollision,
    MountClearFanfare,
    MountSetFavorite,
    MountSpecialAnim,
    MoveApplyInertiaAck,
    MoveApplyMovementForceAck,
    MoveChangeTransport,
    MoveChangeVehicleSeats,
    MoveCollisionDisableAck,
    MoveCollisionEnableAck,
    MoveDismissVehicle,
    MoveDoubleJump,
    MoveEnableDoubleJumpAck,
    MoveEnableSwimToFlyTransAck,
    MoveFallLand,
    MoveFallReset,
    MoveFeatherFallAck,
    MoveForceFlightBackSpeedChangeAck,
    MoveForceFlightSpeedChangeAck,
    MoveForcePitchRateChangeAck,
    MoveForceRootAck,
    MoveForceRunBackSpeedChangeAck,
    MoveForceRunSpeedChangeAck,
    MoveForceSwimBackSpeedChangeAck,
    MoveForceSwimSpeedChangeAck,
    MoveForceTurnRateChangeAck,
    MoveForceUnrootAck,
    MoveForceWalkSpeedChangeAck,
    MoveGravityDisableAck,
    MoveGravityEnableAck,
    MoveGuildBankItem,
    MoveHeartbeat,
    MoveHoverAck,
    MoveInertiaDisableAck,
    MoveInertiaEnableAck,
    MoveInitActiveMoverComplete,
    MoveJump,
    MoveKnockBackAck,
    MoveRemoveInertiaAck,
    MoveRemoveMovementForces,
    MoveRemoveMovementForceAck,
    MoveSeamlessTransferComplete,
    MoveSetCanFlyAck,
    MoveSetCanTurnWhileFallingAck,
    MoveSetCollisionHeightAck,
    MoveSetFacing,
    MoveSetFacingHeartbeat,
    MoveSetFly,
    MoveSetIgnoreMovementForcesAck,
    MoveSetModMovementForceMagnitudeAck,
    MoveSetPitch,
    MoveSetRunMode,
    MoveSetTurnRateCheat,
    MoveSetVehicleRecIdAck,
    MoveSetWalkMode,
    MoveSplineDone,
    MoveStartAscend,
    MoveStartBackward,
    MoveStartDescend,
    MoveStartForward,
    MoveStartPitchDown,
    MoveStartPitchUp,
    MoveStartStrafeLeft,
    MoveStartStrafeRight,
    MoveStartSwim,
    MoveStartTurnLeft,
    MoveStartTurnRight,
    MoveStop,
    MoveStopAscend,
    MoveStopPitch,
    MoveStopStrafe,
    MoveStopSwim,
    MoveStopTurn,
    MoveTeleportAck,
    MoveTimeSkipped,
    MoveUpdateFallSpeed,
    MoveWaterWalkAck,
    MythicPlusRequestMapStats,
    NeutralPlayerSelectFaction,
    NextCinematicCamera,
    ObjectUpdateFailed,
    ObjectUpdateRescued,
    OfferPetition,
    OpeningCinematic,
    OpenItem,
    OpenMissionNpc,
    OpenShipmentNpc,
    OpenTradeskillNpc,
    OptOutOfLoot,
    OverrideScreenFlash,
    PartyInvite,
    PartyInviteResponse,
    PartyUninvite,
    PerformItemInteraction,
    PetitionBuy,
    PetitionRenameGuild,
    PetitionShowList,
    PetitionShowSignatures,
    PetAbandon,
    PetAction,
    PetBattleFinalNotify,
    PetBattleInput,
    PetBattleQueueProposeMatchResult,
    PetBattleQuitNotify,
    PetBattleReplaceFrontPet,
    PetBattleRequestPvp,
    PetBattleRequestUpdate,
    PetBattleRequestWild,
    PetBattleScriptErrorNotify,
    PetBattleWildLocationFail,
    PetCancelAura,
    PetCastSpell,
    PetRename,
    PetSetAction,
    PetSpellAutocast,
    PetStopAttack,
    Ping(Ping),
    PlayerLogin,
    PushQuestToParty,
    PvpLogData,
    QueryBattlePetName,
    QueryCorpseLocationFromClient,
    QueryCorpseTransport,
    QueryCountdownTimer,
    QueryCreature,
    QueryGameObject,
    QueryGarrisonPetName,
    QueryGuildInfo,
    QueryInspectAchievements,
    QueryNextMailTime,
    QueryNpcText,
    QueryPageText,
    QueryPetition,
    QueryPetName,
    QueryPlayerNames,
    QueryPlayerNamesForCommunity,
    QueryPlayerNameByCommunityId,
    QueryQuestCompletionNpcs,
    QueryQuestInfo,
    QueryRealmName,
    QueryScenarioPoi,
    QueryTime,
    QueryTreasurePicker,
    QueryVoidStorage,
    QuestConfirmAccept,
    QuestGiverAcceptQuest,
    QuestGiverChooseReward,
    QuestGiverCloseQuest,
    QuestGiverCompleteQuest,
    QuestGiverHello,
    QuestGiverQueryQuest,
    QuestGiverRequestReward,
    QuestGiverStatusMultipleQuery,
    QuestGiverStatusQuery,
    QuestLogRemoveQuest,
    QuestPoiQuery,
    QuestPushResult,
    QuestSessionBeginResponse,
    QuestSessionRequestStart,
    QuestSessionRequestStop,
    QueuedMessagesEnd,
    QuickJoinAutoAcceptRequests,
    QuickJoinRequestInvite,
    QuickJoinRequestInviteWithConfirmation,
    QuickJoinRespondToInvite,
    QuickJoinSignalToastDisplayed,
    RafClaimActivityReward,
    RafClaimNextReward,
    RafGenerateRecruitmentLink,
    RafUpdateRecruitmentInfo,
    RandomRoll,
    ReadyCheckResponse,
    ReadItem,
    ReclaimCorpse,
    RemoveNewItem,
    RemoveRafRecruit,
    ReorderCharacters(ReorderCharacters),
    RepairItem,
    ReplaceTrophy,
    RepopRequest,
    ReportClientVariables,
    ReportEnabledAddons,
    ReportKeybindingExecutionCounts,
    ReportPvpPlayerAfk,
    ReportServerLag,
    ReportStuckInCombat,
    RequestAccountData,
    RequestAreaPoiUpdate,
    RequestBattlefieldStatus,
    RequestCategoryCooldowns,
    RequestCemeteryList,
    RequestCharacterGuildFollowInfo,
    RequestConquestFormulaConstants,
    RequestCovenantCallings,
    RequestCrowdControlSpell,
    RequestForcedReactions,
    RequestGarrisonTalentWorldQuestUnlocks,
    RequestGuildPartyState,
    RequestGuildRewardsList,
    RequestLatestSplashScreen,
    RequestLfgListBlacklist,
    RequestMythicPlusAffixes,
    RequestMythicPlusSeasonData,
    RequestPartyJoinUpdates,
    RequestPartyMemberStats,
    RequestPetInfo,
    RequestPlayedTime,
    RequestPvpRewards,
    RequestRaidInfo,
    RequestRatedPvpInfo,
    RequestRealmGuildMasterInfo,
    RequestResearchHistory,
    RequestScheduledPvpInfo,
    RequestStabledPets,
    RequestVehicleExit,
    RequestVehicleNextSeat,
    RequestVehiclePrevSeat,
    RequestVehicleSwitchSeat,
    RequestWeeklyRewards,
    RequestWorldQuestUpdate,
    ResetChallengeMode,
    ResetChallengeModeCheat,
    ResetInstances,
    ResurrectResponse,
    RevertMonumentAppearance,
    RideVehicleInteract,
    SaveCufProfiles,
    SaveEquipmentSet,
    SaveGuildEmblem,
    ScenePlaybackCanceled,
    ScenePlaybackComplete,
    SceneTriggerEvent,
    SelfRes,
    SellItem,
    SendCharacterClubInvitation,
    SendContactList,
    SendMail,
    SendTextEmote,
    ServerTimeOffsetRequest,
    SetAchievementsHidden,
    SetActionBarToggles,
    SetActionButton,
    SetActiveMover,
    SetAdvancedCombatLogging,
    SetAssistantLeader,
    SetBackpackAutosortDisabled,
    SetBankAutosortDisabled,
    SetChatDisabled,
    SetContactNotes,
    SetCurrencyFlags,
    SetDifficultyId,
    SetDungeonDifficulty,
    SetEveryoneIsAssistant,
    SetFactionAtWar,
    SetFactionInactive,
    SetFactionNotAtWar,
    SetGameEventDebugViewState,
    SetInsertItemsLeftToRight,
    SetLootMethod,
    SetLootSpecialization,
    SetPartyAssignment,
    SetPartyLeader,
    SetPetSlot,
    SetPlayerDeclinedNames,
    SetPreferredCemetery,
    SetPvp,
    SetRaidDifficulty,
    SetRole,
    SetSavedInstanceExtend,
    SetSelection,
    SetSheathed,
    SetSortBagsRightToLeft,
    SetTaxiBenchmarkMode,
    SetTitle,
    SetTradeCurrency,
    SetTradeGold,
    SetTradeItem,
    SetUsingPartyGarrison,
    SetWarMode,
    SetWatchedFaction,
    ShowTradeSkill,
    SignPetition,
    SilencePartyTalker,
    SocketGems,
    SortBags,
    SortBankBags,
    SortReagentBankBags,
    SpellClick,
    SpiritHealerActivate,
    SplitGuildBankItem,
    SplitGuildBankItemToInventory,
    SplitItem,
    SplitItemToGuildBank,
    StandStateChange,
    StartChallengeMode,
    StartSpectatorWarGame,
    StartWarGame,
    StoreGuildBankItem,
    SubmitUserFeedback,
    SubscriptionInterstitialResponse,
    SummonResponse,
    SupportTicketSubmitComplaint,
    SurrenderArena,
    SuspendCommsAck,
    SuspendTokenResponse,
    SwapGuildBankItemWithGuildBankItem,
    SwapInvItem,
    SwapItem,
    SwapItemWithGuildBankItem,
    SwapSubGroups,
    SwapVoidItem,
    TabardVendorActivate,
    TalkToGossip,
    TaxiNodeStatusQuery,
    TaxiQueryAvailableNodes,
    TaxiRequestEarlyLanding,
    TimeAdjustmentResponse,
    TimeSyncResponse,
    TimeSyncResponseDropped,
    TimeSyncResponseFailed,
    ToggleDifficulty,
    TogglePvp,
    TotemDestroyed,
    ToyClearFanfare,
    TradeSkillSetFavorite,
    TrainerBuySpell,
    TrainerList,
    TransmogrifyItems,
    TurnInPetition,
    Tutorial,
    TwitterCheckStatus,
    TwitterConnect,
    TwitterDisconnect,
    UiMapQuestLinesRequest,
    UnacceptTrade,
    UndeleteCharacter,
    UnlearnSkill,
    UnlearnSpecialization,
    UnlockVoidStorage,
    UpdateAccountData,
    UpdateAreaTriggerVisual,
    UpdateClientSettings,
    UpdateMissileTrajectory,
    UpdateRaidTarget,
    UpdateSpellVisual,
    UpdateVasPurchaseStates,
    UpgradeGarrison,
    UpgradeRuneforgeLegendary,
    UsedFollow,
    UseCritterItem,
    UseEquipmentSet,
    UseItem,
    UseToy,
    VasCheckTransferOk,
    VasGetQueueMinutes,
    VasGetServiceStatus,
    ViolenceLevel,
    VoiceChannelSttTokenRequest,
    VoiceChatJoinChannel,
    VoiceChatLogin,
    VoidStorageTransfer,
    Warden3Data,
    Who,
    WhoIs,
    WorldPortResponse,
    WrapItem,
}
