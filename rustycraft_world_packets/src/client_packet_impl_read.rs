use crate::auth::CMsgAuthSession;
use crate::login::CmsgPlayerLogin;
use crate::opcodes::Opcode;
use crate::position::MovementInfo;
use crate::ClientPacket;
use bytes::Bytes;
use deku::DekuContainerRead;
use std::any::Any;

macro_rules! impl_parse {
    ($($opcode:pat => $to_struct:ident),*) => {
        pub fn parse(opcode: Opcode, data: Bytes) -> Result<Self, deku::DekuError> {
            let data: Box<dyn Any + Send + Sync> = match opcode {
                $(
                    $opcode => Box::new($to_struct::from_bytes((&data, 0))?.1),
                )*
                _ => Box::new(())
            };
            Ok(ClientPacket { opcode, data })
        }
    };
}

impl ClientPacket {
    impl_parse!(
        Opcode::CmsgAuthSession => CMsgAuthSession,
        Opcode::CmsgPlayerLogin => CmsgPlayerLogin,
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
        => MovementInfo
    );
}
