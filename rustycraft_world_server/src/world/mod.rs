use crate::session_handler::Connection;
use deku::DekuContainerRead;
use rustycraft_world_packets::area::Area;
use rustycraft_world_packets::characters::{Character, CharacterEnumServer};
use rustycraft_world_packets::class::Class;
use rustycraft_world_packets::gear::CharacterGear;
use rustycraft_world_packets::gender::Gender;
use rustycraft_world_packets::guid::{Guid, HighGuid};
use rustycraft_world_packets::inventory::InventoryType;
use rustycraft_world_packets::login::{CmsgPlayerLogin, SmsgLoginVerifyWorld};
use rustycraft_world_packets::map::Map;
use rustycraft_world_packets::movement_block::{
    LivingBuilder, MovementBlockBuilder, MovementBlockLivingVariants,
};
use rustycraft_world_packets::movement_flags::{ExtraMovementFlags, MovementFlags};
use rustycraft_world_packets::object::{ObjectType, ObjectUpdateType, SmsgUpdateObject};
use rustycraft_world_packets::objects::prelude::*;
use rustycraft_world_packets::opcodes::Opcode;
use rustycraft_world_packets::position::Vector3d;
use rustycraft_world_packets::power::Power;
use rustycraft_world_packets::race::Race;
use rustycraft_world_packets::time_sync::SmsgTimeSyncReq;
use rustycraft_world_packets::tutorial::SmsgTutorialFlags;
use rustycraft_world_packets::{object, ClientPacket};
use tokio::sync::mpsc;

pub struct WorldHandler {
    incoming_connections: mpsc::UnboundedReceiver<(ClientPacket, Connection)>,
    realm_server_sender: mpsc::UnboundedSender<Connection>,
    connections: Vec<Connection>,
}

impl WorldHandler {
    pub fn new(
        incoming_connections: mpsc::UnboundedReceiver<(ClientPacket, Connection)>,
        realm_server_sender: mpsc::UnboundedSender<Connection>,
    ) -> Self {
        Self {
            incoming_connections,
            realm_server_sender,
            connections: vec![],
        }
    }

    pub fn run(mut self) {
        println!("World server started");
        loop {
            while let Ok((packet, mut connection)) = self.incoming_connections.try_recv() {
                match packet.opcode {
                    Opcode::CmsgPlayerLogin => {
                        Self::handle_player_login(
                            &mut connection,
                            CmsgPlayerLogin::from_bytes((&packet.data, 0)).unwrap().1,
                        )
                        .unwrap();
                    }
                    _ => {}
                }
                self.connections.push(connection);
            }
            for connection in &mut self.connections {
                while let Ok(packet) = connection.receiver().try_recv() {
                    match packet.opcode {
                        Opcode::CmsgPlayerLogin => {
                            Self::handle_player_login(
                                connection,
                                CmsgPlayerLogin::from_bytes((&packet.data, 0)).unwrap().1,
                            )
                            .unwrap();
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn handle_player_login(
        connection: &mut Connection,
        _player: CmsgPlayerLogin,
    ) -> anyhow::Result<()> {
        println!("Player login request");
        connection.sender().send(Box::new(SmsgLoginVerifyWorld {
            map: Map::EasternKingdoms,
            position: Vector3d {
                x: 200.0,
                y: 200.0,
                z: 200.0,
                rotation: Some(0.0),
            },
        }))?;
        connection
            .sender()
            .send(Box::new(SmsgTutorialFlags::default()))?;
        let player = Player {
            unit: Unit {
                object: Object {
                    guid: Some(Guid::new(HighGuid::Player, 4)),
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
            ..Default::default()
        };
        let movement = MovementBlockBuilder::default()
            .living(MovementBlockLivingVariants::Living(Box::new(
                LivingBuilder::default()
                    .backwards_running_speed(4.5)
                    .backwards_swimming_speed(0.0)
                    .extra_flags(ExtraMovementFlags::new(0))
                    .fall_time(0.0)
                    .flags(MovementFlags::new(0))
                    .flight_speed(0.0)
                    .backwards_flight_speed(0.0)
                    .living_position(Vector3d {
                        x: -8949.95,
                        y: -132.493,
                        z: 83.5312,
                        rotation: Some(0.0),
                    })
                    .pitch_rate(0.0)
                    .running_speed(60.0)
                    .swimming_speed(0.0)
                    .timestamp(0)
                    .turn_rate(std::f32::consts::PI)
                    .walking_speed(1.0)
                    .build()
                    .expect("I am sure that this is correct."),
            )))
            .set_self()
            .build()
            .expect("And this is correct too.");
        let update_object = SmsgUpdateObject::new(vec![object::Object {
            update_type: ObjectUpdateType::CreateObject2 {
                guid: Guid::new(HighGuid::Player, 4).into(),
                object_type: ObjectType::Player,
                update_fields: player.into(),
                movement,
            },
        }]);
        connection.sender().send(Box::new(update_object))?;

        connection
            .sender()
            .send(Box::new(SmsgTimeSyncReq { time_sync: 0 }))?;
        println!("All data was sent");
        Ok(())
    }
}
