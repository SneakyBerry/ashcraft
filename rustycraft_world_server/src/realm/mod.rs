use crate::session_handler::{Connection, ConnectionState};
use anyhow::anyhow;
use rustycraft_common::Account;
use rustycraft_database::redis::RedisClient;
use rustycraft_world_packets::auth::{
    AuthChallengeServer, AuthOk, AuthResponseServer, CMsgAuthSession,
};
use rustycraft_world_packets::characters::{Character, CharacterEnumServer};
use rustycraft_world_packets::common::area::Area;
use rustycraft_world_packets::common::class::Class;
use rustycraft_world_packets::common::expansion::Expansion;
use rustycraft_world_packets::common::gender::Gender;
use rustycraft_world_packets::gear::CharacterGear;
use rustycraft_world_packets::guid::Guid;
use rustycraft_world_packets::inventory::InventoryType;
use rustycraft_world_packets::map::Map;
use rustycraft_world_packets::objects::prelude::EquipmentSlots;
use rustycraft_world_packets::opcodes::Opcode;
use rustycraft_world_packets::position::Vector3d;
use rustycraft_world_packets::race::Race;
use rustycraft_world_packets::response_code::ResponseCode;
use rustycraft_world_packets::{guid, ClientPacket};
use std::mem;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use wow_srp::normalized_string::NormalizedString;
use wow_srp::wrath_header::ProofSeed;

const TICK_TIME: Duration = Duration::from_millis(1000 / 60);
const AUTH_SERVER_RESPONSE_OK: AuthResponseServer = AuthResponseServer {
    result: ResponseCode::AuthOk,
    ok: Some(AuthOk {
        billing_flags: 0,
        billing_rested: 0,
        billing_time: 0,
        expansion: Expansion::WrathOfTheLichLing,
    }),
    wait: None,
};

fn char_data() -> CharacterEnumServer {
    let mut equipment = [CharacterGear {
        equipment_display_id: 0,
        inventory_type: InventoryType::NonEquip,
        enchantment: 0,
    }; 23];
    // equipment[EquipmentSlots::MainHand as usize] = CharacterGear {
    //     equipment_display_id: 2,
    //     inventory_type: InventoryType::WeaponMainHand,
    //     enchantment: 0,
    // };
    CharacterEnumServer::new(vec![Character {
        guid: Guid::Player(guid::Player::new(0, 4)),
        name: "Warr".to_string(),
        race: Race::Draenei,
        class: Class::Warrior,
        gender: Gender::Female,
        skin: 0,
        face: 0,
        hair_style: 0,
        hair_color: 0,
        facial_hair: 0,
        level: 1,
        area: Area::HowlingFjord,
        map: Map::Northrend,
        position: Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rotation: None,
        },
        guild_id: 0,
        flags: 0,
        recustomization_flags: 0,
        first_login: false,
        pet_display_id: 0,
        pet_level: 0,
        pet_family: 0,
        equipment,
    }])
}

type Handler<T> = fn(&RealmHandler, &mut Connection, T) -> anyhow::Result<()>;

pub struct RealmHandler {
    incoming_connections: mpsc::UnboundedReceiver<Connection>,
    world_server_sender: mpsc::UnboundedSender<Connection>,
    connections: Vec<Connection>,
    redis: Arc<RedisClient>,
}

impl RealmHandler {
    pub fn new(
        incoming_connections: mpsc::UnboundedReceiver<Connection>,
        world_server_sender: mpsc::UnboundedSender<Connection>,
        redis: Arc<RedisClient>,
    ) -> Self {
        Self {
            incoming_connections,
            world_server_sender,
            connections: vec![],
            redis,
        }
    }

    pub async fn run(mut self) {
        info!("Realm handler started");
        let mut sleep_time = Duration::default();
        loop {
            tokio::time::sleep(sleep_time).await;
            let t = Instant::now();
            let mut uninitialized_connections = vec![];
            while let Ok(connection) = self.incoming_connections.try_recv() {
                uninitialized_connections.push(connection);
            }
            for mut connection in uninitialized_connections {
                if let Ok(_) = self.init_connection(&mut connection).await {
                    info!("Connection handled");
                    connection.state = ConnectionState::Auth;
                    self.connections.push(connection);
                } else {
                    info!("Connection failed");
                }
            }
            let conn_len = self.connections.len();
            let connections = mem::replace(&mut self.connections, Vec::with_capacity(conn_len));
            for mut conn in connections {
                if let Ok(data) = conn.receiver().try_recv() {
                    match data.get_opcode() {
                        Opcode::CmsgCharEnum => conn.sender().send(Box::new(char_data())).unwrap(),
                        Opcode::CmsgCharCreate => todo!(),
                        Opcode::CmsgPlayerLogin => {
                            conn.state = ConnectionState::WorldLogin(data.data_as());
                            self.world_server_sender.send(conn).unwrap();
                            continue;
                        }
                        _ => {}
                    }
                }
                self.connections.push(conn)
            }
            sleep_time = TICK_TIME.saturating_sub(t.elapsed());
        }
    }

    async fn init_connection(&self, connection: &mut Connection) -> anyhow::Result<()> {
        let seed = ProofSeed::new();
        let auth_challenge = AuthChallengeServer {
            server_seed: seed.seed(),
        };
        connection.sender().send(Box::new(auth_challenge))?;

        let client_packet = connection
            .receiver()
            .recv()
            .await
            .ok_or_else(|| anyhow!("Socket closed"))?;

        if let Opcode::CmsgAuthSession = client_packet.get_opcode() {
            let packet = client_packet.data_as::<CMsgAuthSession>();
            let account = self.redis.get::<Account>(&packet.username).await?;
            if let Ok(encryption) = seed.into_header_crypto(
                &NormalizedString::new(&packet.username)?,
                account
                    .session_key
                    .try_into()
                    .map_err(|vect| anyhow!("Can't convert {vect:?} to [u8; 40]"))?,
                packet.client_proof,
                packet.client_seed,
            ) {
                {
                    connection
                        .encryption()
                        .lock()
                        .await
                        .set(encryption)
                        .map_err(|_| ())
                        .expect("OnceCell already set");
                }
                connection
                    .sender()
                    .send(Box::new(AUTH_SERVER_RESPONSE_OK))?;
            } else {
                connection.sender().send(Box::new(AuthResponseServer {
                    result: ResponseCode::AuthSystemError,
                    ok: None,
                    wait: None,
                }))?;
            }
            Ok(())
        } else {
            Err(anyhow!("Unexpected packet"))
        }
    }
}
