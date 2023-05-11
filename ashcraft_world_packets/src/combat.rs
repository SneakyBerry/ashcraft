use crate::prelude::Guid;
use deku::prelude::*;

pub mod client {
    use super::*;

    #[derive(Debug, Clone, DekuRead)]
    pub struct AttackSwing {
        pub victim: Guid,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct AttackStop;

    #[derive(Debug, Clone, DekuRead)]
    pub struct SetSheathed {
        pub sheath_state: u32,
    }
}

pub mod server {
    use super::*;
    use crate::opcodes::Opcode;
    use ashcraft_derive::ServerPacket;

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgAttackSwingNotInRange)]
    pub struct AttackSwingNotInRange();

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgAttackSwingBadFacing)]
    pub struct AttackSwingBadFacing();

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgAttackSwingDeadTarget)]
    pub struct AttackSwingDeadTarget();

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgAttackSwingCantAttack)]
    pub struct AttackSwingCantAttack();

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgAttackStart)]
    pub struct AttackStart {
        pub attacker: Guid,
        pub victim: Guid,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgAttackStop)]
    pub struct ServerAttackStop {
        pub attacker: Guid,
        pub victim: Option<AttackStopPayload>,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgCancelCombat)]
    pub struct CancelCombat();

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgCancelAutoRepeat)]
    pub struct CancelAutoRepeat {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, DekuWrite)]
    pub struct AttackStopPayload {
        pub victim: Guid,
        pub is_dead: bool,
    }
}
