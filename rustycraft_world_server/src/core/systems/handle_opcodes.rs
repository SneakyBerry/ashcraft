use crate::core::app::Connections;
use crate::core::events::packets::ClientPacketReceived;
use bevy::prelude::*;
use rustycraft_world_packets::prelude::*;

macro_rules! send_event {
    ($packet:ident, $world:ident, $entity:ident => {
        $(
            $opcode:pat => $payload:ident
        ),*
    }) => {
        match $packet.get_opcode() {
            $(
                $opcode => $world
                    .resource_mut::<Events<ClientPacketReceived<$payload>>>()
                    .send(ClientPacketReceived(
                        *$entity,
                        $packet.data_as(),
                    )),
            )*
            _ => {}
        }
    };
}

pub(crate) fn handle_opcodes(world: &mut World) {
    world.resource_scope(|world, mut connections: Mut<Connections>| {
        #[cfg(debug_assertions)]
        {
            assert!(connections.keys().all(|e| world.entities().contains(*e)))
        }
        for (entity, (connection, _)) in connections.iter_mut() {
            while let Ok(packet) = connection.receiver().try_recv() {
                send_event!(packet, world, entity => {
                    Opcode::CmsgNameQuery => CmsgNameQuery,
                    Opcode::CmsgItemQuerySingle => CmsgItemQuerySingle,
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
                    | Opcode::MsgMoveStartDescend => CMovementData
                })
            }
        }
    });
}
