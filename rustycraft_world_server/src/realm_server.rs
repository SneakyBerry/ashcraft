use crate::classes::Classes;
use crate::crypt::{AES128Companion, INITIALIZED_RSA, RSA};
use crate::opcodes::OpcodeClient;
use crate::packets::auth::{
    AddrType, AuthContinuedSession, AuthResponse, AuthSuccessInfo, Class, ConnectTo,
    ConnectToSerial, GameTime, Ping, Pong, RaceClassAvailability,
};
use crate::packets::characters::{
    CharResponse, CharacterInfo, CreateCharacterRequest, CreateCharacterResponse,
    EnumCharactersResult, UndeleteCooldownStatusResponse,
};
use crate::packets::client_config::ClientCacheVersion;
use crate::packets::system::{FeatureSystemStatusGlueScreen, SetTimeZoneInformation};
use crate::packets::{ClientPacket, IntoServerPacket, PacketHeader, RawClientPacket, ServerPacket};
use crate::races::Races;
use crate::session_modules::crypt::{EncryptedModeTrait, SessionKey};
use crate::world_listener::{SessionHandler, Void};
use crate::world_server::ServerEventEnum;
use crate::{ObjectEvent, ObjectGuid, OpcodeServer, ServerObject};
use bytes::{Bytes, BytesMut};
use deku::prelude::*;
use enum_iterator::IntoEnumIterator;
use rand::Rng;
use rustycraft_common::Character;
use rustycraft_database::redis::RedisClient;
use rustycraft_protocol::expansions::Expansions;
use rustycraft_protocol::rpc_responses::WowRpcResponse;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, RwLock};

#[derive(Debug)]
pub struct RealmServer {
    pub(crate) addr: SocketAddr,
    pub(crate) redis: RedisClient,
    pub(crate) aes_companion: AES128Companion,
    pub(crate) client_socket_reader: ReadHalf<TcpStream>,
    pub(crate) client_socket_writer: WriteHalf<TcpStream>,
    pub(crate) encryption_key: [u8; 16],
    pub(crate) session_key: [u8; 40],
    pub(crate) server_challenge: [u8; 16],
    characters: Vec<CharacterInfo>,
}

#[derive(Debug)]
pub enum Error {
    SocketError(tokio::io::Error),
    SendError(String),
    Anyhow(anyhow::Error),
}

impl From<tokio::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::SocketError(e)
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Anyhow(e)
    }
}

impl<T> From<mpsc::error::SendError<T>> for Error {
    fn from(e: mpsc::error::SendError<T>) -> Self {
        Error::SendError(e.to_string())
    }
}

impl RealmServer {

    pub(crate) async fn init_connection(&mut self) -> Result<(), Error> {
        self.initial_packets().await?;
        self.init_session().await?;
        Ok(())
    }

    async fn init_session(&mut self) -> anyhow::Result<()> {
        let mut available_classes = vec![];
        for race in Races::into_enum_iter() {
            for class in Classes::into_enum_iter() {
                available_classes.push(RaceClassAvailability::new(
                    race.clone(),
                    vec![Class::new(
                        class,
                        Expansions::ExpansionClassic,
                        Expansions::ExpansionShadowlands,
                    )],
                ))
            }
        }
        let auth_response = AuthResponse::new(
            WowRpcResponse::Ok,
            Some(AuthSuccessInfo::new(
                1024,
                0,
                Expansions::ExpansionShadowlands,
                Expansions::ExpansionShadowlands,
                0,
                0,
                chrono::Local::now().timestamp() as u64,
                available_classes,
                false,
                false,
                GameTime::new(0, u32::MAX, false),
                None,
                None,
                None,
                vec![],
                vec![],
            )),
            None,
        );
        self.write_to_socket(Box::new(auth_response)).await?;
        // let tz_info = SetTimeZoneInformation::default();
        // self.write_to_socket(Box::new(tz_info)).await?;
        // let features_glue_screen = FeatureSystemStatusGlueScreen::new();
        // self.write_to_socket(Box::new(features_glue_screen)).await?;
        // let cache_version = ClientCacheVersion::new();
        // self.write_to_socket(Box::new(cache_version)).await?;
        Ok(())
    }

    async fn handle_enum_characters_request(&mut self) -> anyhow::Result<()> {
        let res = EnumCharactersResult::new(self.characters.clone());
        self.write_to_socket(Box::new(res)).await
    }

    async fn handle_get_undelete_character_cooldown_status_request(
        &mut self,
    ) -> anyhow::Result<()> {
        let res = UndeleteCooldownStatusResponse::new();
        self.write_to_socket(Box::new(res)).await
    }

    async fn handle_create_character_request(
        &mut self,
        request: CreateCharacterRequest,
    ) -> anyhow::Result<()> {
        self.characters.push(CharacterInfo::from_create_request(
            request,
            self.characters.len() as u8,
        ));
        let res =
            CreateCharacterResponse::new(CharResponse::CreateSuccess, Some(ObjectGuid::new()));
        self.write_to_socket(Box::new(res)).await
    }

    async fn handle_player_login_request(
        &mut self,
        // request: PlayerLogin
    ) -> anyhow::Result<()> {
        let res = ConnectTo::new(
            0,
            ConnectToSerial::WorldAttempt1,
            &[127, 0, 0, 1],
            9901,
            AddrType::IPv4,
        );
        self.write_to_socket(Box::new(res)).await
    }
}

#[async_trait::async_trait]
impl SessionHandler for RealmServer {
    fn new(socket: TcpStream, _: Option<mpsc::Sender<ServerEventEnum>>) -> anyhow::Result<Self> {
        let peer_addr = socket.peer_addr()?;
        let (reader, writer) = split(socket);
        Ok(RealmServer {
            redis: RedisClient::new().unwrap(),
            addr: peer_addr,
            client_socket_reader: reader,
            client_socket_writer: writer,
            encryption_key: [0; 16],
            session_key: [0; 40],
            aes_companion: AES128Companion::new(),
            characters: vec![],
            server_challenge: rand::thread_rng().gen(),
        })
    }

    async fn handle(self) -> Result<(), Error> {
        let addr = self.addr.clone();
        let rw_locked_self = Arc::new(RwLock::new(self));
        rw_locked_self.write().await.init_connection().await?;
        loop {
            let client_event = rw_locked_self.write().await.read_client_packet().await;
            debug!(target: "WorldSession", "[{:?}] New packet received from client: {:?}", &addr, client_event);
            match client_event {
                Ok(ClientPacket::Ping(ping)) => {
                    let pong = Pong::from(ping);
                    let _ = rw_locked_self
                        .write()
                        .await
                        .write_to_socket(Box::new(pong))
                        .await?;
                }
                Ok(ClientPacket::EnumCharacters) => {
                    let _ = rw_locked_self
                        .write()
                        .await
                        .handle_enum_characters_request()
                        .await?;
                }
                Ok(ClientPacket::GetUndeleteCharacterCooldownStatus) => {
                    let _ = rw_locked_self
                        .write()
                        .await
                        .handle_get_undelete_character_cooldown_status_request()
                        .await?;
                }
                Ok(ClientPacket::BattlePayGetPurchaseList) => {}
                Ok(ClientPacket::BattlePayGetProductList) => {}
                Ok(ClientPacket::UpdateVasPurchaseStates) => {}
                Ok(ClientPacket::ServerTimeOffsetRequest) => {}
                Ok(ClientPacket::KeepAlive) => {}
                Ok(ClientPacket::ReorderCharacters(data)) => {}
                Ok(ClientPacket::CreateCharacter(char_request)) => {
                    let _ = rw_locked_self
                        .write()
                        .await
                        .handle_create_character_request(char_request)
                        .await?;
                }
                Ok(ClientPacket::LogDisconnect) => {
                    break;
                }
                Ok(ClientPacket::PlayerLogin) => {
                    let mut write_self = rw_locked_self.write().await;
                    write_self.handle_player_login_request().await?;
                }
                Err(Error::SocketError(er)) => Err(er)?,
                _ => {
                    error!(
                        "[ {:?} ] Unhandled packet: {:?}",
                        rw_locked_self.read().await.addr,
                        client_event
                    );
                }
            };
        }
        Ok(())
    }
}
