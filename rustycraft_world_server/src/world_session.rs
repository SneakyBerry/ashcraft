use crate::crypt::{AES128Companion, INITIALIZED_RSA, RSA};
use crate::opcodes::OpcodeClient;
use crate::packets::auth::{AuthResponse, Ping, Pong};
use crate::packets::client_config::ClientCacheVersion;
use crate::packets::system::{FeatureSystemStatusGlueScreen, SetTimeZoneInformation};
use crate::packets::{ClientPacket, IntoServerPacket, PacketHeader, RawClientPacket, ServerPacket};
use crate::world_listener::WorldSessionHandler;
use crate::world_server::{NewSession, ServerEventEnum};
use crate::OpcodeServer;
use bytes::{Bytes, BytesMut};
use deku::prelude::*;
use rand::Rng;
use rustycraft_database::redis::RedisClient;
use rustycraft_protocol::rpc_responses::WowRpcResponse;
use std::net::SocketAddr;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

pub struct WorldClientSession {
    pub(crate) addr: SocketAddr,
    pub(crate) redis: RedisClient,
    pub(crate) rsa: &'static RSA,
    pub(crate) aes_companion: AES128Companion,
    pub(crate) client_socket_reader: ReadHalf<TcpStream>,
    pub(crate) client_socket_writer: WriteHalf<TcpStream>,
    pub(crate) world_server_events: Sender<ServerEventEnum>,
    pub(crate) server_challenge: [u8; 16],
    pub(crate) encryption_key: [u8; 16],
    pub(crate) session_key: [u8; 40],
}

impl WorldClientSession {
    pub(crate) async fn read_client_packet(&mut self) -> anyhow::Result<ClientPacket> {
        let pkt_size = self.client_socket_reader.read_u32_le().await?;
        let mut aes_token = Vec::with_capacity(12);
        self.client_socket_reader.read_buf(&mut aes_token).await?;
        let mut packet = BytesMut::with_capacity(pkt_size as usize);
        self.client_socket_reader.read_buf(&mut packet).await?;

        let decrypted = self.aes_companion.decrypt(&packet, &aes_token)?;
        let result = ClientPacket::try_from(decrypted)?;
        trace!("New packet from client: {:?}", &result);
        Ok(result)
    }

    pub(crate) async fn write_to_socket(
        &mut self,
        data: Box<dyn IntoServerPacket>,
    ) -> anyhow::Result<()> {
        trace!("Plain packet: {:?}", &data);
        let result = data.serialize()?;
        let encrypted = self.aes_companion.encrypt(&result)?;
        let pkt = ServerPacket::new(encrypted.aes_tag, encrypted.cipher_text);
        self.client_socket_writer.write(&pkt.serialize()?).await?;
        Ok(())
    }

    pub(crate) async fn init_connection(&mut self) -> anyhow::Result<()> {
        self.initial_packets().await?;
        self.init_encryption_state().await?;
        self.enter_encrypted_mode().await?;
        self.init_session().await?;

        Ok(())
    }

    async fn init_session(&mut self) -> anyhow::Result<()> {
        let auth_response = AuthResponse::new(WowRpcResponse::Ok, None, None);
        self.write_to_socket(Box::new(auth_response)).await?;
        let tz_info = SetTimeZoneInformation::default();
        self.write_to_socket(Box::new(tz_info)).await?;
        let features_glue_screen = FeatureSystemStatusGlueScreen::new();
        self.write_to_socket(Box::new(features_glue_screen)).await?;
        let cache_version = ClientCacheVersion::new();
        self.write_to_socket(Box::new(cache_version)).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl WorldSessionHandler for WorldClientSession {
    fn new(socket: TcpStream, world_server_tx: Sender<ServerEventEnum>) -> anyhow::Result<Self> {
        let peer_addr = socket.peer_addr()?;
        let (reader, writer) = split(socket);
        Ok(WorldClientSession {
            rsa: &INITIALIZED_RSA,
            redis: RedisClient::new().unwrap(),
            addr: peer_addr,
            client_socket_reader: reader,
            client_socket_writer: writer,
            world_server_events: world_server_tx,
            server_challenge: rand::thread_rng().gen(),
            encryption_key: [0; 16],
            session_key: [0; 40],
            aes_companion: AES128Companion::new(),
        })
    }

    async fn handle(mut self) -> anyhow::Result<()> {
        self.init_connection().await?;
        let (world_tx, mut world_rx) = mpsc::channel(2048);
        self.world_server_events
            .send(ServerEventEnum::NewSession(NewSession {
                addr: self.addr,
                sender: world_tx,
            }))
            .await?;
        loop {
            tokio::select! {
                Some(server_event) = world_rx.recv() => {
                    debug!(target: "WorldSession", "[{:?}] New packet received from server: {:?}", self.addr, server_event);
                    self.write_to_socket(server_event).await?;
                },
                Ok(client_event) = self.read_client_packet() => {
                    debug!(target: "WorldSession", "[{:?}] New packet received from client: {:?}", self.addr, client_event);
                    self.world_server_events.send(ServerEventEnum::NewClientPacket(self.addr, client_event)).await?;
                }
            };
        }
        Ok(())
    }
}
