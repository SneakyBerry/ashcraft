use crate::prelude::client::*;
use crate::prelude::*;
use crate::ClientPacket;
use bytes::Bytes;
use deku::DekuContainerRead;
use std::any::Any;

macro_rules! impl_parse {
    ($($opcode:pat => $to_struct:ident),*) => {
        pub fn parse(opcode: Opcode, data: Bytes) -> Result<Self, deku::DekuError> {
            let data: Box<dyn Any + Send + Sync> = match opcode {
                $(
                    $opcode => {
                        let (rest, res) = $to_struct::from_bytes((&data, 0))?;
                        if rest.0.len() != 0 {
                            tracing::error!(
                                message = "Incomplete read",
                                opcode = ?opcode,
                                packet = ?res,
                                rest = ?rest.0
                            );
                        }
                        Box::new(res)
                    },
                )*
                _ => {
                    tracing::warn!(message = "Opcode isn't parsed", opcode = ?opcode, data = ?data);
                    Box::new(())
                }
            };
            Ok(ClientPacket { opcode, data })
        }
    };
}

impl ClientPacket {
    impl_parse! {
        Opcode::CmsgCharEnum => CharEnum,
        Opcode::CmsgAuthSession => CMsgAuthSession,
        Opcode::CmsgPlayerLogin => CmsgPlayerLogin,
        Opcode::CmsgShowingCloak => ShowingCloak,
        Opcode::CmsgShowingHelm => ShowingHelm,
        Opcode::CmsgPlayedTime => PlayedTimeClient,
        Opcode::CmsgLogoutRequest => LogoutRequest,
        Opcode::CmsgPlayerLogout => PlayerLogout,
        Opcode::CmsgLogoutCancel => LogoutCancel,
        Opcode::CmsgEmote => ClientEmote,
        Opcode::CmsgAttackSwing => AttackSwing,
        Opcode::CmsgAttackStop => AttackStop,
        Opcode::CmsgSetSheathed => SetSheathed,
        Opcode::CmsgGuildQuery => QueryGuildInfo,
        Opcode::CmsgGuildCreate => GuildCreate,
        Opcode::CmsgGuildInfo => GuildGetInfo,
        Opcode::CmsgGuildRoster => GuildGetRoster,
        Opcode::CmsgGuildMotd => GuildUpdateMotdText,
        Opcode::CmsgGuildAccept => AcceptGuildInvite,
        Opcode::CmsgGuildDecline => GuildDeclineInvitation,
        Opcode::CmsgGuildInvite => GuildInviteByName,
        Opcode::MsgGuildEventLogQuery => GuildEventLogQuery,
        Opcode::MsgGuildPermissions => GuildPermissionsQuery,
        Opcode::CmsgGuildRank => GuildSetRankPermissions,
        Opcode::CmsgGuildAddRank => GuildAddRank,
        Opcode::CmsgGuildDelRank => GuildDeleteRank,
        Opcode::CmsgGuildInfoText => GuildUpdateInfoText,
        Opcode::CmsgGuildSetPublicNote | Opcode::CmsgGuildSetOfficerNote => GuildSetMemberNote,
        Opcode::CmsgGuildDisband => GuildDelete,
        Opcode::CmsgGuildDemote => GuildDemoteMember,
        Opcode::CmsgGuildPromote => GuildPromoteMember,
        Opcode::CmsgGuildRemove => GuildOfficerRemoveMember,
        Opcode::CmsgGuildLeave => GuildLeave,
        Opcode::CmsgGuildBankerActivate => GuildBankActivate,
        Opcode::CmsgGuildBankBuyTab => GuildBankBuyTab,
        Opcode::CmsgGuildBankUpdateTab => GuildBankUpdateTab,
        Opcode::CmsgGuildBankDepositMoney => GuildBankDepositMoney,
        Opcode::CmsgGuildBankQueryTab => GuildBankQueryTab,
        Opcode::MsgGuildBankMoneyWithdrawn => GuildBankRemainingWithdrawMoneyQuery,
        Opcode::CmsgGuildBankWithdrawMoney => GuildBankWithdrawMoney,
        Opcode::CmsgGuildBankSwapItems => GuildBankSwapItems,
        Opcode::MsgGuildBankLogQuery => GuildBankLogQuery,
        Opcode::CmsgSetGuildBankText => GuildBankSetTabText,
        Opcode::CmsgGuildLeader => GuildSetGuildMaster,
        Opcode::MsgSaveGuildEmblem => SaveGuildEmblem,
        Opcode::CmsgLfgLeave => LFGLeave,
        Opcode::CmsgLfgJoin => LFGJoin,
        Opcode::CmsgQuestQuery => QueryQuestInfo,
        Opcode::CmsgCreatureQuery => QueryCreature,
        Opcode::CmsgGameObjectQuery => QueryGameObject,
        Opcode::CmsgNameQuery => NameQuery,
        Opcode::CmsgItemQuerySingle => QueryItemSingle,
        Opcode::CmsgCancelCast => CancelCast,
        Opcode::CmsgCancelAura => CancelAura,
        Opcode::CmsgPetCancelAura => PetCancelAura,
        Opcode::CmsgCancelGrowthAura => CancelGrowthAura,
        Opcode::CmsgCancelMountAura => CancelMountAura,
        Opcode::CmsgCancelAutoRepeatSpell => CancelAutoRepeatSpell,
        Opcode::CmsgCancelChannelling => CancelChannelling,
        Opcode::CmsgTotemDestroyed => TotemDestroyed,
        Opcode::MsgMoveStartForward
        | Opcode::MsgMoveStartBackward
        | Opcode::MsgMoveStop
        | Opcode::MsgMoveStartStrafeLeft
        | Opcode::MsgMoveStartStrafeRight
        | Opcode::MsgMoveStopStrafe
        | Opcode::MsgMoveJump
        | Opcode::MsgMoveStartTurnLeft
        | Opcode::MsgMoveStartTurnRight
        | Opcode::MsgMoveStopTurn
        | Opcode::MsgMoveStartPitchUp
        | Opcode::MsgMoveStartPitchDown
        | Opcode::MsgMoveStopPitch
        | Opcode::MsgMoveSetRunMode
        | Opcode::MsgMoveSetWalkMode
        | Opcode::MsgMoveFallLand
        | Opcode::MsgMoveStartSwim
        | Opcode::MsgMoveStopSwim
        | Opcode::MsgMoveSetFacing
        | Opcode::MsgMoveSetPitch
        | Opcode::MsgMoveHeartbeat
        | Opcode::CmsgMoveFallReset
        | Opcode::CmsgMoveSetFly
        | Opcode::MsgMoveStartAscend
        | Opcode::MsgMoveStopAscend
        | Opcode::CmsgMoveChngTransport
        | Opcode::MsgMoveStartDescend
        => CMovementData
    }
}
