use crate::session_handler::Connection;
use bevy_ecs::entity::EntityLocation;
use bevy_ecs::prelude::*;
use rustycraft_world_packets::common::class::Class;
use rustycraft_world_packets::common::emotes::Emote;
use rustycraft_world_packets::common::gender::Gender;
use rustycraft_world_packets::common::school::DamageSchool;
use rustycraft_world_packets::guid::Guid;
use rustycraft_world_packets::login::{CmsgPlayerLogin, SmsgLoginVerifyWorld};
use rustycraft_world_packets::map::Map;
use rustycraft_world_packets::movement_block::{
    LivingBuilder, MovementBlockBuilder, MovementBlockLivingVariants,
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

pub struct WorldHandler {
    incoming_connections: mpsc::UnboundedReceiver<(ClientPacket, Connection)>,
    realm_server_sender: mpsc::UnboundedSender<Connection>,
    connections: Vec<Connection>,
    schedule: Schedule,
    world: World,
}

impl WorldHandler {
    pub fn new(
        incoming_connections: mpsc::UnboundedReceiver<(ClientPacket, Connection)>,
        realm_server_sender: mpsc::UnboundedSender<Connection>,
    ) -> Self {
        let mut world = World::new();
        Self {
            incoming_connections,
            realm_server_sender,
            connections: vec![],
            schedule: Schedule::default(),
            world,
        }
    }

    pub fn run(mut self) {
        info!("World server started");
        let mut sleep_time = Duration::default();
        loop {
            std::thread::sleep(sleep_time);
            let t = Instant::now();
            while let Ok((packet, mut connection)) = self.incoming_connections.try_recv() {
                if packet.get_opcode() == Opcode::CmsgPlayerLogin {
                    Self::handle_player_login(&mut connection, packet.data_as::<CmsgPlayerLogin>())
                        .unwrap();
                }
                self.connections.push(connection);
            }
            for connection in &mut self.connections {
                while let Ok(packet) = connection.receiver().try_recv() {
                    match packet.get_opcode() {
                        Opcode::CmsgPlayerLogin => {
                            Self::handle_player_login(
                                connection,
                                packet.data_as::<CmsgPlayerLogin>(),
                            )
                            .unwrap();
                        }
                        Opcode::CmsgNameQuery => {
                            let data = packet.data_as::<CmsgNameQuery>();
                            connection
                                .sender()
                                .send(Box::new(SmsgNameQueryResponse {
                                    guid: data.guid.into(),
                                    response: NameQueryResponse::Known(KnownName {
                                        name: "Abobus".to_string(),
                                        realm_name: 0,
                                        race: Race::Human,
                                        sex: Gender::Female,
                                        class: Class::Warrior,
                                        declined: false,
                                    }),
                                }))
                                .unwrap();
                        }
                        Opcode::CmsgItemQuerySingle => {
                            let data = packet.data_as::<CmsgItemQuerySingle>();
                            println!("{:?}", &data);
                            if data.item_id != 13262 {
                                continue;
                            }
                            let mut item_flags = ItemFlags::default();
                            let mut item_flags2 = ItemFlags2::default();
                            item_flags2
                                .set_item_flag2_bnet_account_trade_ok()
                                .set_item_flag2_faction_alliance()
                                .set_item_flag2_no_durability();
                            connection
                                .sender()
                                .send(Box::new(SmsgItemQuerySingle {
                                    item_id: data.item_id,
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
                                        description: "".to_string(),
                                        page_text: 0,
                                        language_id: 0,
                                        page_material: 0,
                                        start_quest: 0,
                                        lock_id: 0,
                                        material: 1,
                                        sheath: Sheath::OffHand,
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
                                }))
                                .unwrap();
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
                        | Opcode::MsgMoveStartDescend => {}
                        _ => {}
                    }
                }
            }
            sleep_time = TICK_TIME.saturating_sub(t.elapsed());
        }
    }

    fn handle_player_login(
        connection: &mut Connection,
        _player: CmsgPlayerLogin,
    ) -> anyhow::Result<()> {
        connection.sender().send(Box::new(SmsgLoginVerifyWorld {
            map: Map::EasternKingdoms,
            position: Vector3d {
                x: -8940.,
                y: -132.,
                z: 83.,
                rotation: Some(180.0),
            },
        }))?;
        connection
            .sender()
            .send(Box::new(SmsgTutorialFlags::default()))?;
        let mut visible_items: [_; 19] = Default::default();
        visible_items[EquipmentSlots::OffHand as usize] = Some(EquipedItem {
            id: 13262,
            permanent: 13262,
            temporary: 13262,
        });
        let mut player = Player {
            unit: Unit {
                object: Object {
                    guid: Some(Guid::Player(guid::Player::new(0, 4))),
                    scale_x: Some(1.0),
                    ..Default::default()
                },
                data: Some(UnitData {
                    race: Race::Human,
                    class: Class::Warrior,
                    gender: Gender::Female,
                    power: Power::Rage,
                }),
                health: Some(100),
                max_health: Some(100),
                level: Some(1),
                faction_template: Some(1),
                display_id: Some(50),
                native_display_id: Some(50),
                ..Default::default()
            },
            visible_items,
            ..Default::default()
        };
        let mut mf = MovementFlags::default();
        mf.set_can_fly();
        let mut mfe = ExtraMovementFlags::default();
        let movement = MovementBlockBuilder::default()
            .living(MovementBlockLivingVariants::Living(Box::new(
                LivingBuilder::default()
                    .backwards_running_speed(4.5)
                    .backwards_swimming_speed(0.0)
                    .movement_data(MovementInfo {
                        flags: mf,
                        flags_extra: mfe,
                        time: 0,
                        pos: Vector3d {
                            x: -8940.34,
                            y: -132.009,
                            z: 83.6564,
                            rotation: Some(0.5),
                        },
                        transport_info: None,
                        pitch: None,
                        fall_time: 0,
                        falling: None,
                        spline_elevation: None,
                    })
                    .flight_speed(90.0)
                    .backwards_flight_speed(90.0)
                    .pitch_rate(10.0)
                    .running_speed(10.0)
                    .swimming_speed(10.0)
                    .turn_rate(std::f32::consts::PI)
                    .walking_speed(1.0)
                    .build()
                    .expect("I am sure that this is correct."),
            )))
            .set_self()
            .build()
            .expect("And this is correct too.");

        let mut npc_flags = NPCFlags::default();
        npc_flags.set_repair();
        let unit = Unit {
            object: Object {
                guid: Some(Guid::MapSpecific(guid::MapSpecific::Unit {
                    unk: 0,
                    npc_id: 0,
                    counter: 2,
                })),
                scale_x: Some(1.0),
                ..Default::default()
            },
            data: Some(UnitData {
                race: Race::Human,
                class: Class::Warrior,
                gender: Gender::Female,
                power: Power::Rage,
            }),
            health: Some(100),
            max_health: Some(100),
            level: Some(1),
            faction_template: Some(1),
            display_id: Some(25337),
            native_display_id: Some(50),
            npc_flags: Some(npc_flags),
            npc_emote_state: Some(Emote::StateTalk),
            ..Default::default()
        };
        let unit_movement = MovementBlockBuilder::default()
            .living(MovementBlockLivingVariants::Stationary(Vector3d {
                x: -8940.34,
                y: -132.009,
                z: 83.6564,
                rotation: Some(0.),
            }))
            .build()
            .expect("And this is correct too.");

        connection
            .sender()
            .send(Box::new(SmsgUpdateObject::new(vec![
                object::Object {
                    update_type: ObjectUpdateType::CreateObject2 {
                        guid: Guid::MapSpecific(guid::MapSpecific::Unit {
                            unk: 0,
                            npc_id: 0,
                            counter: 2,
                        })
                        .into(),
                        object_type: ObjectType::Unit,
                        update_fields: unit.into(),
                        movement: unit_movement,
                    },
                },
                object::Object {
                    update_type: ObjectUpdateType::CreateObject2 {
                        guid: Guid::Player(guid::Player::new(0, 4)).into(),
                        object_type: ObjectType::Player,
                        update_fields: player.clone().into(),
                        movement: movement.clone(),
                    },
                },
            ])))?;
        connection
            .sender()
            .send(Box::new(SmsgTimeSyncReq { time_sync: 0 }))?;

        Ok(())
    }
}
