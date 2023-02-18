use crate::core::commands::packet_event::EmitClientPacketEvent;
use crate::core::components::common::Health;
use crate::core::components::player::PlayerBundle;
use crate::core::events::packets::ClientPacketReceived;
use crate::session_handler::{Connection, ConnectionState};
use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_ecs::system::SystemState;
use rustycraft_world_packets::prelude::*;
use std::time::Duration;
use tokio::sync::mpsc;
macro_rules! try_send {
    ($commands:ident, $entity:ident, $connection:ident => $data:expr) => {
        // If we can't send any data to channel that means connection was closed.
        if let Err(_) = (*$connection).sender().send($data) {
            $commands.entity($entity).despawn();
            continue;
        }
    };
}
macro_rules! try_send_box {
    ($commands:tt, $entity:ident, $connection:ident => $data:expr) => {
        // If we can't send any data to channel that means connection was closed.
        try_send!($commands, $entity, $connection => Box::new($data))
    };
}

#[derive(Debug, Component)]
pub(crate) struct InGame {
    guid: Guid,
    time_sync: Instant,
    time_sync_counter: u32,
}
#[derive(Debug, Component)]
pub(crate) struct MovementData(MovementInfo);
#[derive(Debug, Component)]
pub(crate) struct Updates(Vec<Object>);

pub(crate) fn handle_opcodes(
    mut commands: Commands,
    mut connections: Query<(Entity, &mut Connection, &mut Transform), With<InGame>>,
) {
    for (entity, mut connection, mut transform) in connections.iter_mut() {
        while let Ok(packet) = (*connection).receiver().try_recv() {
            match packet.get_opcode() {
                Opcode::CmsgNameQuery => commands.add(EmitClientPacketEvent(
                    entity,
                    packet.data_as::<CmsgNameQuery>(),
                )),
                Opcode::CmsgItemQuerySingle => commands.add(EmitClientPacketEvent(
                    entity,
                    packet.data_as::<CmsgItemQuerySingle>(),
                )),
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
                | Opcode::MsgMoveStartDescend => {
                    commands.add(EmitClientPacketEvent(
                        entity,
                        packet.data_as::<CMovementData>(),
                    ));
                }
                _ => {}
            }
        }
    }
}

pub(crate) fn handle_player_login(
    mut commands: Commands,
    mut connections: Query<(Entity, &mut Connection), Added<Connection>>,
) {
    for (entity, mut connection) in connections.iter_mut() {
        let guid = if let ConnectionState::WorldLogin(packet) = &connection.state {
            packet.guid
        } else {
            unreachable!()
        };
        try_send_box!(commands, entity, connection => SmsgLoginVerifyWorld {
            map: Map::EasternKingdoms,
            position: Vector3d {
                x: -8940.,
                y: -132.,
                z: 83.,
                rotation: Some(180.0),
            },
        });
        try_send_box!(commands, entity, connection => SmsgTutorialFlags::default());

        // Create player component
        let mut visible_items: [_; 19] = Default::default();
        visible_items[EquipmentSlots::MainHand as usize] = EquipedItem {
            id: 13262,
            permanent: 0,
            temporary: 0,
        };
        let mut player = super::components::player::Player::new();
        player.unit = UnitUpdate {
            object: ObjectUpdate {
                guid,
                scale: 1.0,
                ..Default::default()
            },
            data: UnitData {
                race: Race::Human,
                class: Class::Warrior,
                gender: Gender::Female,
                power: Power::Rage,
            },
            health: 100,
            max_health: 100,
            level: 1,
            faction_template: 1,
            display_id: 50,
            native_display_id: 50,
            ..Default::default()
        };
        *player.visible_items = visible_items.into();

        commands.entity(entity).insert((
            PlayerBundle {
                player: player.clone(),
                position: Transform {
                    translation: Vec3::new(-8940.34, -132.009, 83.6564),
                    rotation: Quat::from_rotation_x(180.0),
                    scale: Default::default(),
                },
                health: Health::new(100.0),
            },
            InGame {
                guid,
                time_sync: Instant::now(),
                time_sync_counter: 0,
            },
            Updates(vec![]),
        ));

        connection.as_mut().state = ConnectionState::InGame;
    }
}

pub(crate) fn send_player_update(
    mut updates: Query<(Entity, &mut Updates)>,
    mut changed_player: Query<
        (Entity, &mut super::components::player::Player, &Transform),
        Added<super::components::player::Player>,
    >,
) {
    for (entity, mut player, transform) in &mut changed_player.iter_mut() {
        let movement = MovementBlockLivingVariants::Living(Box::new(
            LivingBuilder::default()
                .backwards_running_speed(4.5)
                .backwards_swimming_speed(0.0)
                .movement_data(MovementInfo {
                    pos: Vector3d {
                        x: transform.translation.x,
                        y: transform.translation.y,
                        z: transform.translation.z,
                        rotation: Some(transform.rotation.x),
                    },
                    ..Default::default()
                })
                .flight_speed(10.0)
                .backwards_flight_speed(10.0)
                .pitch_rate(10.0)
                .running_speed(10.0)
                .swimming_speed(10.0)
                .turn_rate(std::f32::consts::PI)
                .walking_speed(1.0)
                .build()
                .expect("I am sure that this is correct."),
        ));

        updates.iter_mut().for_each(|(e, mut updates)| {
            if e != entity {
                updates.0.push(Object {
                    update_type: ObjectUpdateType::CreateObject {
                        guid: player.unit.object.guid.into(),
                        object_type: ObjectType::Player,
                        movement: MovementBlockBuilder::default()
                            .living(movement.clone())
                            .build()
                            .expect("And this is correct too."),
                        update_fields: player.render_update(),
                    },
                })
            } else {
                updates.0.push(Object {
                    update_type: ObjectUpdateType::CreateObject2 {
                        guid: player.unit.object.guid.into(),
                        object_type: ObjectType::Player,
                        movement: MovementBlockBuilder::default()
                            .living(movement.clone())
                            .set_self()
                            .build()
                            .expect("And this is correct too."),
                        update_fields: player.render_update(),
                    },
                })
            }
        });
        player.swap()
    }
}

pub(crate) fn send_position_update(
    mut updates: Query<(Entity, &mut Updates)>,
    changed_transform: Query<
        (Entity, &super::components::common::Guid, &MovementData),
        Changed<Transform>,
    >,
) {
    for (entity, guid, movement) in &mut changed_transform.iter() {
        let data = movement.0.clone();
        let movement = if data.flags.is_mask_moving() {
            MovementBlockLivingVariants::Living(Box::new(
                LivingBuilder::default()
                    .backwards_running_speed(4.5)
                    .backwards_swimming_speed(0.0)
                    .movement_data(data)
                    .flight_speed(10.0)
                    .backwards_flight_speed(10.0)
                    .pitch_rate(10.0)
                    .running_speed(10.0)
                    .swimming_speed(10.0)
                    .turn_rate(std::f32::consts::PI)
                    .walking_speed(1.0)
                    .build()
                    .expect("I am sure that this is correct."),
            ))
        } else {
            MovementBlockLivingVariants::Stationary(data.pos)
        };
        let update_movement_block = ObjectUpdateType::Movement {
            guid: guid.guid().into(),
            movement: MovementBlockBuilder::default()
                .living(movement)
                .build()
                .expect("And this is correct too."),
        };
        updates.iter_mut().for_each(|(e, mut updates)| {
            if e != entity {
                updates.0.push(Object {
                    update_type: update_movement_block.clone(),
                })
            }
        })
    }
}

pub(crate) fn sync_time(
    mut commands: Commands,
    mut connections: Query<(Entity, &mut Connection, &mut InGame)>,
) {
    for (entity, connection, mut in_game) in connections.iter_mut() {
        if in_game.time_sync.elapsed() > Duration::from_secs(5) {
            try_send_box!(commands, entity, connection => SmsgTimeSyncReq { time_sync: in_game.time_sync_counter });
            in_game.time_sync = Instant::now();
            in_game.time_sync_counter = in_game.time_sync_counter.wrapping_add(1);
        }
    }
}

pub(crate) fn send_updates(
    mut commands: Commands,
    mut connections: Query<(Entity, &mut Connection, &mut Updates), Changed<Updates>>,
) {
    for (entity, connection, mut updates) in &mut connections.iter_mut() {
        if updates.0.is_empty() {
            continue;
        }
        let update_data = std::mem::replace(&mut (*updates).0, Vec::new());
        try_send_box!(commands, entity, connection => SmsgUpdateObject::new(update_data));
    }
}

pub(crate) fn handle_incoming_connections(
    mut commands: Commands,
    mut incoming_connections: NonSendMut<mpsc::UnboundedReceiver<Connection>>,
) {
    while let Ok(connection) = incoming_connections.try_recv() {
        assert!(
            matches!(connection.state, ConnectionState::WorldLogin(..)),
            "Invalid connection state"
        );
        commands.spawn(connection);
    }
}
