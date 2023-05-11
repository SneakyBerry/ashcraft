use crate::core::app::Connections;
use crate::core::events::packets::ClientPacketReceived;
use bevy::prelude::*;
use ashcraft_world_packets::prelude::client::*;
use ashcraft_world_packets::prelude::*;

macro_rules! send_event {
    (
        $(
            $opcode:pat => $payload:ident
        ),*
    ) => {
        pub(crate) fn populate_with_events(app: &mut App) {
            $(
                app.add_event::<ClientPacketReceived<$payload>>();
            )*
        }
        pub(crate) fn handle_opcodes(world: &mut World) {
            world.resource_scope(|world, mut connections: Mut<Connections>| {
            #[cfg(debug_assertions)]
            {
                assert!(connections.keys().all(|e| world.entities().contains(*e)))
            }
            for (entity, (connection, _)) in connections.iter_mut() {
                while let Ok(packet) = connection.receiver().try_recv() {
                    match packet.get_opcode() {
                        $(
                            $opcode => world
                                .resource_mut::<Events<ClientPacketReceived<$payload>>>()
                                .send(ClientPacketReceived(
                                    *entity,
                                    packet.get_opcode(),
                                    packet.data_as(),
                                )),
                        )*
                        _ => {}
                        }
                    }
                }
            })
        }
    }
}

send_event! {
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
