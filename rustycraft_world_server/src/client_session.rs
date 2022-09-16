use anyhow::anyhow;
use bytes::{Bytes, BytesMut};
use deku::prelude::*;
use rustycraft_common::Account;
use rustycraft_database::redis::RedisClient;
use rustycraft_world_packets::area::Area;
use rustycraft_world_packets::auth::{
    AuthChallengeServer, AuthOk, AuthResponseServer, AuthSessionClient,
};
use rustycraft_world_packets::characters::{Character, CharacterEnumServer};
use rustycraft_world_packets::class::Class;
use rustycraft_world_packets::expansion::Expansion;
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
use rustycraft_world_packets::object::{Object, ObjectType, ObjectUpdateType, SmsgUpdateObject};
use rustycraft_world_packets::opcodes::Opcode;
use rustycraft_world_packets::position::Vector3d;
use rustycraft_world_packets::power::Power;
use rustycraft_world_packets::race::Race;
use rustycraft_world_packets::response_code::ResponseCode;
use rustycraft_world_packets::time_sync::SmsgTimeSyncReq;
use rustycraft_world_packets::tutorial::SmsgTutorialFlags;
use rustycraft_world_packets::update_mask::{UpdateMask, UpdateType};
use rustycraft_world_packets::ServerPacket;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, MutexGuard, OnceCell};
use wow_srp::normalized_string::NormalizedString;
use wow_srp::wrath_header::{ProofSeed, ServerCrypto};

pub struct ClientSession {
    socket_reader: Mutex<OwnedReadHalf>,
    socket_writer: Mutex<OwnedWriteHalf>,
    peer_addr: SocketAddr,
    encryption: Mutex<OnceCell<ServerCrypto>>,
    redis: Arc<RedisClient>,
}

fn char_data() -> CharacterEnumServer {
    CharacterEnumServer::new(vec![Character {
        guid: Guid {
            high_guid: HighGuid::Player,
            guid: 0,
        },
        name: "Abobus".to_string(),
        race: Race::Human,
        class: Class::Warrior,
        gender: Gender::Male,
        skin: 0,
        face: 0,
        hair_style: 0,
        hair_color: 0,
        facial_hair: 0,
        level: 0,
        area: Area::DunMorogh,
        map: Map::EasternKingdoms,
        position: Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        guild_id: 0,
        flags: 0,
        recustomization_flags: 0,
        first_login: false,
        pet_display_id: 0,
        pet_level: 0,
        pet_family: 0,
        equipment: [CharacterGear {
            equipment_display_id: 0,
            inventory_type: InventoryType::NonEquip,
            enchantment: 0,
        }; 23],
    }])
}

impl ClientSession {
    pub fn new(socket: TcpStream, redis: Arc<RedisClient>) -> Self {
        let peer_addr = socket
            .peer_addr()
            .expect("Incoming connection without remote address? o_O");
        let (socket_reader, socket_writer) = socket.into_split();
        ClientSession {
            peer_addr,
            socket_reader: Mutex::new(socket_reader),
            socket_writer: Mutex::new(socket_writer),
            encryption: Default::default(),
            redis,
        }
    }

    pub async fn serve(self: Arc<Self>) -> anyhow::Result<()> {
        let mut socket_reader = self.socket_reader.lock().await;
        self.init_connection(&mut socket_reader).await?;

        loop {
            let res = self.read_socket(&mut socket_reader).await;
            if let Ok(Some((opcode, data))) = res {
                match opcode {
                    Opcode::CmsgReadyForAccountDataTimes => {}
                    Opcode::CmsgCharEnum => self.send_packet(char_data()).await?,
                    Opcode::CmsgRealmSplit
                    | Opcode::CmsgPing
                    | Opcode::CmsgSetActiveVoiceChannel => {}
                    Opcode::CmsgPlayerLogin => {
                        self.handle_player_login(CmsgPlayerLogin::from_bytes((&data, 0))?.1)
                            .await?
                    }
                    opcode => todo!("{:?}: {:?}", opcode, data),
                }
            } else {
                let first = res.unwrap();
                first.unwrap();
            }
        }
    }

    async fn read_socket(
        self: &Arc<Self>,
        socket_reader: &mut MutexGuard<'_, OwnedReadHalf>,
    ) -> anyhow::Result<Option<(Opcode, Bytes)>> {
        // TODO: Reuse buffers
        let mut headers = [0; 6];
        let mut data = BytesMut::new();

        if socket_reader.read_exact(&mut headers).await? > 0 {
            {
                if let Some(encryption) = self.encryption.lock().await.get_mut() {
                    encryption.decrypt(&mut headers);
                }
            }
            // Size includes opcode with len 4 that already been parsed
            let pkt_size = u16::from_be_bytes(headers[0..2].try_into()?).saturating_sub(4);
            let (_, opcode) = Opcode::from_bytes((&headers[2..], 0))?;
            data.resize(pkt_size as usize, 0);
            socket_reader.read_exact(&mut data).await?;
            trace!(
                "[{:?}] Packet: {opcode:?} with size: {pkt_size} body: {data:?}",
                self.peer_addr
            );
            Ok(Some((opcode, data.freeze())))
        } else {
            Ok(None)
        }
    }

    async fn init_connection(
        self: &Arc<Self>,
        socket_reader: &mut MutexGuard<'_, OwnedReadHalf>,
    ) -> anyhow::Result<()> {
        let seed = ProofSeed::new();
        let auth_challenge = AuthChallengeServer {
            server_seed: seed.seed(),
        };
        self.send_packet(auth_challenge).await?;

        let (opcode, data) = self
            .read_socket(socket_reader)
            .await?
            .ok_or_else(|| anyhow!("Socket closed"))?;

        if let Opcode::CmsgAuthSession = opcode {
            let (_, packet) = AuthSessionClient::from_bytes((&data, 0))?;
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
                    self.encryption
                        .lock()
                        .await
                        .set(encryption)
                        .map_err(|_| ())
                        .expect("OnceCell already set");
                }
                self.send_packet(AuthResponseServer {
                    result: ResponseCode::AuthOk,
                    ok: Some(AuthOk {
                        billing_flags: 0,
                        billing_rested: 0,
                        billing_time: 0,
                        expansion: Expansion::WrathOfTheLichLing,
                    }),
                    wait: None,
                })
                .await?;
            } else {
                self.send_packet(AuthResponseServer {
                    result: ResponseCode::AuthSystemError,
                    ok: None,
                    wait: None,
                })
                .await?;
            }
            Ok(())
        } else {
            Err(anyhow!("Unexpected packet"))
        }
    }

    pub(crate) async fn send_packet(
        self: &Arc<Self>,
        data: impl ServerPacket,
    ) -> anyhow::Result<()> {
        let mut writer = self.socket_writer.lock().await;
        let payload = data.encode(
            self.encryption
                .lock()
                .await
                .get_mut()
                .map(|e| e.encrypter()),
        )?;
        writer
            .write_all(&payload)
            .await
            .map_err(anyhow::Error::from)
    }

    async fn handle_player_login(self: &Arc<Self>, _player: CmsgPlayerLogin) -> anyhow::Result<()> {
        self.send_packet(SmsgLoginVerifyWorld {
            map: Map::EasternKingdoms,
            position: Vector3d {
                x: 200.0,
                y: 200.0,
                z: 200.0,
            },
            orientation: 0.0,
        })
        .await
        .unwrap();
        self.send_packet(SmsgTutorialFlags::default())
            .await
            .unwrap();
        // let update_mask = UpdatePlayer::new()
        //     .set_object_GUID(Guid::new(4))
        //     .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
        //     .set_object_SCALE_X(1.0)
        //     .set_unit_health(100)
        //     .set_unit_MAXHEALTH(100)
        //     .set_unit_LEVEL(1)
        //     .set_unit_FACTIONTEMPLATE(1)
        //     .set_unit_DISPLAYID(50)
        //     .set_unit_NATIVEDISPLAYID(50);

        let mut update_mask = UpdateMask::new(UpdateType::Player);
        let guid = Guid {
            high_guid: HighGuid::Player,
            guid: 4,
        };
        update_mask
            .set_value(0, guid.as_u64() as u32)
            .set_value(1, (guid.as_u64() >> 32 as u32) as u32)
            .set_value(
                23,
                u32::from_le_bytes([
                    Race::Human as u8,
                    Class::Warrior as u8,
                    Gender::Female as u8,
                    Power::Rage as u8,
                ]),
            )
            .set_value(4, 1)
            .set_value(24, 100)
            .set_value(32, 100)
            .set_value(54, 1)
            .set_value(55, 1)
            .set_value(67, 50)
            .set_value(68, 50);

        let update_flag = MovementBlockBuilder::default()
            .living(MovementBlockLivingVariants::Living(
                LivingBuilder::default()
                    .backwards_running_speed(4.5)
                    .backwards_swimming_speed(0.0)
                    .extra_flags(ExtraMovementFlags::new(0))
                    .fall_time(0.0)
                    .flags(MovementFlags::new(0))
                    .flight_speed(0.0)
                    .backwards_flight_speed(0.0)
                    .living_orientation(0.0)
                    .living_position(Vector3d {
                        x: -8949.95,
                        y: -132.493,
                        z: 83.5312,
                    })
                    .pitch_rate(0.0)
                    .running_speed(7.0)
                    .swimming_speed(0.0)
                    .timestamp(0)
                    .turn_rate(std::f32::consts::PI)
                    .walking_speed(1.0)
                    .build()
                    .unwrap(),
            ))
            .set_self()
            .build()
            .unwrap();
        let update_object = SmsgUpdateObject::new(vec![Object {
            update_type: ObjectUpdateType::CreateObject2 {
                guid3: Guid {
                    high_guid: HighGuid::Player,
                    guid: 4,
                }
                .into(),
                object_type: ObjectType::Player,
                mask2: update_mask,
                movement2: update_flag,
            },
        }]);
        self.send_packet(update_object).await.unwrap();
        self.send_packet(SmsgTimeSyncReq { time_sync: 0 })
            .await
            .unwrap();
        Ok(())
    }
}
