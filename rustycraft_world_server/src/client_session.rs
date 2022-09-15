use crate::packets::area::Area;
use crate::packets::auth::{AuthChallengeServer, AuthOk, AuthResponseServer, AuthSessionClient};
use crate::packets::characters::{Character, CharacterEnumServer};
use crate::packets::class::Class;
use crate::packets::expansion::Expansion;
use crate::packets::gear::CharacterGear;
use crate::packets::gender::Gender;
use crate::packets::guid::Guid;
use crate::packets::inventory::InventoryType;
use crate::packets::login::{CmsgPlayerLogin, SmsgLoginVerifyWorld};
use crate::packets::map::Map;
use crate::packets::opcodes::Opcode;
use crate::packets::race::Race;
use crate::packets::response_code::ResponseCode;
use crate::packets::tutorial::SmsgTutorialFlags;
use crate::packets::vector3d::Vector3d;
use crate::packets::ServerPacket;
use anyhow::anyhow;
use bytes::{Bytes, BytesMut};
use deku::prelude::*;
use rustycraft_common::Account;
use rustycraft_database::redis::RedisClient;
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
        guid: Guid { guid: 0 },
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
        .await?;
        self.send_packet(SmsgTutorialFlags::default()).await?;
        Ok(())
    }
}
