use crate::session_handler::{Connection, ConnectionState};
use bevy_ecs::entity::EntityLocation;
use bevy_ecs::prelude::*;
use bevy_time::{FixedTimestep, Time};
use rustycraft_ecs::common::Health;
use rustycraft_ecs::player::PlayerBundle;
use rustycraft_ecs::prelude::prelude::*;
use rustycraft_ecs::prelude::{Quat, Vec3};
use rustycraft_world_packets::common::class::Class;
use rustycraft_world_packets::common::emotes::Emote;
use rustycraft_world_packets::common::gender::Gender;
use rustycraft_world_packets::common::school::DamageSchool;
use rustycraft_world_packets::guid::{Guid, Player};
use rustycraft_world_packets::login::{CmsgPlayerLogin, SmsgLoginVerifyWorld};
use rustycraft_world_packets::map::Map;
use rustycraft_world_packets::movement_block::{
    LivingBuilder, MovementBlock, MovementBlockBuilder, MovementBlockLivingVariants,
};
use rustycraft_world_packets::movement_flags::{ExtraMovementFlags, MovementFlags};
use rustycraft_world_packets::object::{ObjectType, ObjectUpdateType, SmsgUpdateObject};
use rustycraft_world_packets::objects::prelude::*;
use rustycraft_world_packets::opcodes::Opcode;
use rustycraft_world_packets::position::{CMovementData, MovementInfo, Vector3d};
use rustycraft_world_packets::power::Power;
use rustycraft_world_packets::query::{
    CmsgItemQuerySingle, CmsgNameQuery, InventoryType, ItemBondingType, ItemClass, ItemDamageData,
    ItemFlags, ItemFlags2, ItemMod, ItemQuality, ItemSocketData, ItemSpellData, ItemStats,
    KnownName, NameQueryResponse, ReputationRank, Sheath, SmsgItemQuerySingle,
    SmsgNameQueryResponse, SocketColor, Weapon,
};
use rustycraft_world_packets::race::Race;
use rustycraft_world_packets::time_sync::SmsgTimeSyncReq;
use rustycraft_world_packets::transport::TransportInfo;
use rustycraft_world_packets::tutorial::SmsgTutorialFlags;
use rustycraft_world_packets::{guid, object, ClientPacket};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

const TICK_TIME: Duration = Duration::from_millis(1000 / 60);

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

fn ashbringer_data() -> SmsgItemQuerySingle {
    let mut item_flags2 = ItemFlags2::default();
    item_flags2
        .set_item_flag2_bnet_account_trade_ok()
        .set_item_flag2_faction_alliance()
        .set_item_flag2_no_durability();
    SmsgItemQuerySingle {
        item_id: 13262,
        response: Some(ItemStats {
            class: ItemClass::Weapon(Weapon::Sword2),
            sound_override_subclass: -1,
            name: "Ashbringer SOBOROBON".to_string(),
            display_info_id: 23875,
            quality: ItemQuality::Legendary,
            flags: ItemFlags::default(),
            flags2: ItemFlags2::default(),
            buy_price: 732658,
            sell_price: 146531,
            inventory_type: InventoryType::WeaponTwoHands,
            allowable_class: 0,
            allowable_race: 0,
            item_level: 76,
            required_level: 60,
            required_skill: 0,
            required_skill_rank: 0,
            required_spell: 0,
            required_honor_rank: 0,
            required_city_rank: 0,
            required_reputation_faction: 0,
            required_reputation_rank: ReputationRank::Hated,
            max_count: 1,
            stackable: 1,
            container_slots: 0,
            stats_count: 0,
            item_stat: vec![],
            scaling_stat_distribution: 0,
            scaling_stat_value: 0,
            damage: [
                ItemDamageData {
                    min: 201.0,
                    max: 247.0,
                    damage_type: DamageSchool::Physical,
                },
                ItemDamageData {
                    min: 30.0,
                    max: 50.0,
                    damage_type: DamageSchool::Holy,
                },
            ],
            resistance: Default::default(),
            delay: 3000,
            ammo_type: 0,
            ranged_mod_range: 0.0,
            spells: [
                ItemSpellData {
                    spell_id: 18112,
                    spell_trigger: 2,
                    spell_charges: 0,
                    spell_cooldown: -1,
                    spell_category: 0,
                    spell_category_cooldown: -1,
                },
                ItemSpellData {
                    spell_id: 0,
                    spell_trigger: 0,
                    spell_charges: 0,
                    spell_cooldown: -1,
                    spell_category: 0,
                    spell_category_cooldown: -1,
                },
                ItemSpellData {
                    spell_id: 0,
                    spell_trigger: 0,
                    spell_charges: 0,
                    spell_cooldown: -1,
                    spell_category: 0,
                    spell_category_cooldown: -1,
                },
                ItemSpellData {
                    spell_id: 0,
                    spell_trigger: 0,
                    spell_charges: 0,
                    spell_cooldown: -1,
                    spell_category: 0,
                    spell_category_cooldown: -1,
                },
                ItemSpellData {
                    spell_id: 0,
                    spell_trigger: 0,
                    spell_charges: 0,
                    spell_cooldown: -1,
                    spell_category: 0,
                    spell_category_cooldown: -1,
                },
            ],
            bonding: ItemBondingType::WhenPickedUp,
            description: "abbobus".to_string(),
            page_text: 0,
            language_id: 0,
            page_material: 0,
            start_quest: 0,
            lock_id: 0,
            material: 1,
            sheath: Sheath::MainHand,
            random_property: 0,
            random_suffix: 0,
            block: 0,
            item_set: 0,
            max_durability: 145,
            area: 0,
            map: 0,
            bag_family: 0,
            totem_category: 0,
            socket: [ItemSocketData {
                color: SocketColor::Unknown,
                content: 0,
            }; 3],
            socket_bonus: 0,
            gem_properties: 0,
            required_disenchant_skill: 0,
            armor_damage_modifier: 0.0,
            duration: 0,
            item_limit_category: 0,
            holiday_id: 0,
        }),
    }
}

pub struct WorldHandler {
    schedule: Schedule,
    incoming_connections: IncomingConnections,
    realm_server_sender: mpsc::UnboundedSender<Connection>,
}

#[derive(Debug, StageLabel)]
enum Stages {
    GetConnections,
    BeforeUpdate,
    Update,
    AfterUpdate,
}

#[derive(Debug, Component)]
struct InGame {
    guid: Guid,
    time_sync: Instant,
}
#[derive(Debug, Component)]
struct MovementData(MovementInfo);
#[derive(Debug, Component)]
struct Updates(Vec<object::Object>);

type IncomingConnections = mpsc::UnboundedReceiver<Connection>;

impl WorldHandler {
    pub fn new(
        incoming_connections: IncomingConnections,
        realm_server_sender: mpsc::UnboundedSender<Connection>,
    ) -> Self {
        let mut schedule = Schedule::default();
        schedule.add_stage(Stages::GetConnections, SystemStage::parallel());
        schedule.add_stage_after(
            Stages::GetConnections,
            Stages::BeforeUpdate,
            SystemStage::single_threaded(),
        );
        schedule.add_stage_after(
            Stages::BeforeUpdate,
            Stages::Update,
            SystemStage::parallel(),
        );
        schedule.add_stage_after(Stages::Update, Stages::AfterUpdate, SystemStage::parallel());
        schedule.add_system_to_stage(
            Stages::GetConnections,
            handle_incoming_connections.at_start(),
        );
        schedule.add_system_to_stage(Stages::BeforeUpdate, handle_player_login);
        schedule.add_system_to_stage(Stages::Update, handle_opcodes);
        schedule.add_system_to_stage(Stages::AfterUpdate, send_updates);
        schedule.add_system_to_stage(
            Stages::AfterUpdate,
            send_position_update.before(send_updates),
        );
        schedule.add_system_to_stage(Stages::AfterUpdate, send_player_update.before(send_updates));
        schedule.add_system_to_stage(Stages::AfterUpdate, sync_time.after(send_updates));
        Self {
            schedule,
            incoming_connections,
            realm_server_sender,
        }
    }

    pub fn run(mut self) {
        let mut world = World::new();
        world.insert_non_send_resource(self.incoming_connections);
        world.insert_non_send_resource(self.realm_server_sender);
        world.init_resource::<bevy_time::Time>();

        info!("World server started");
        let mut sleep_time = Duration::default();
        loop {
            std::thread::sleep(sleep_time);
            let t = Instant::now();
            self.schedule.run(&mut world);
            sleep_time = TICK_TIME.saturating_sub(t.elapsed());
        }
    }
}

fn handle_incoming_connections(
    mut commands: Commands,
    mut incoming_connections: NonSendMut<IncomingConnections>,
) {
    while let Ok(connection) = incoming_connections.try_recv() {
        assert!(
            matches!(connection.state, ConnectionState::WorldLogin(..)),
            "Invalid connection state"
        );
        commands.spawn(connection);
    }
}

fn handle_opcodes(
    mut commands: Commands,
    mut connections: Query<(Entity, &mut Connection, &mut Transform), With<InGame>>,
) {
    for (entity, mut connection, mut transform) in &mut connections.iter_mut() {
        while let Ok(packet) = (*connection).receiver().try_recv() {
            match packet.get_opcode() {
                Opcode::CmsgNameQuery => {
                    let data = packet.data_as::<CmsgNameQuery>();
                    try_send_box!(commands, entity, connection => SmsgNameQueryResponse {
                        guid: data.guid.into(),
                        response: NameQueryResponse::Known(KnownName {
                            name: "Abobus".to_string(),
                            realm_name: 0,
                            race: Race::Human,
                            sex: Gender::Female,
                            class: Class::Warrior,
                            declined: false,
                        }),
                    });
                }
                Opcode::CmsgItemQuerySingle => {
                    let data = packet.data_as::<CmsgItemQuerySingle>();
                    if data.item_id != 13262 {
                        continue;
                    }
                    try_send_box!(commands, entity, connection => ashbringer_data());
                }
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
                    let data = packet.data_as::<CMovementData>();
                    let pos = &data.movement_info.pos;
                    transform.translation.x = pos.x;
                    transform.translation.y = pos.y;
                    transform.translation.z = pos.z;
                    transform.rotation = Quat::from_rotation_x(pos.rotation.unwrap());
                    commands
                        .entity(entity)
                        .insert(MovementData(data.movement_info));
                }
                _ => {}
            }
        }
    }
}

fn handle_player_login(
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
        let mut player = rustycraft_ecs::player::Player::new();
        let mut player_update = player.get_for_update();
        player_update.unit = UnitUpdate {
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
        player_update.visible_items = visible_items.into();

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
            },
            Updates(vec![]),
        ));

        connection.as_mut().state = ConnectionState::InGame;
    }
}

fn send_player_update(
    mut updates: Query<(Entity, &mut Updates)>,
    mut changed_player: Query<
        (Entity, &mut rustycraft_ecs::player::Player, &Transform),
        Added<rustycraft_ecs::player::Player>,
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
                updates.0.push(object::Object {
                    update_type: ObjectUpdateType::CreateObject {
                        guid: player.get().unit.object.guid.into(),
                        object_type: ObjectType::Player,
                        movement: MovementBlockBuilder::default()
                            .living(movement.clone())
                            .build()
                            .expect("And this is correct too."),
                        update_fields: player.render_update(),
                    },
                })
            } else {
                updates.0.push(object::Object {
                    update_type: ObjectUpdateType::CreateObject2 {
                        guid: player.get().unit.object.guid.into(),
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

fn send_position_update(
    mut updates: Query<(Entity, &mut Updates)>,
    changed_transform: Query<
        (Entity, &rustycraft_ecs::common::Guid, &MovementData),
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
                updates.0.push(object::Object {
                    update_type: update_movement_block.clone(),
                })
            }
        })
    }
}

fn sync_time(
    mut commands: Commands,
    mut connections: Query<(Entity, &mut Connection, &mut InGame)>,
) {
    for (entity, connection, mut in_game) in connections.iter_mut() {
        if in_game.time_sync.elapsed() > Duration::from_secs(5) {
            try_send_box!(commands, entity, connection => SmsgTimeSyncReq { time_sync: 0 });
            in_game.time_sync = Instant::now();
        }
    }
}

fn send_updates(
    mut commands: Commands,
    mut connections: Query<(Entity, &mut Connection, &mut Updates), Changed<Updates>>,
) {
    for (entity, mut connection, mut updates) in &mut connections.iter_mut() {
        if updates.0.is_empty() {
            continue;
        }
        let update_data = std::mem::replace(&mut (*updates).0, Vec::new());
        try_send_box!(commands, entity, connection => SmsgUpdateObject::new(update_data));
    }
}
