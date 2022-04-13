use crate::constants::{
    AUTH_CHECK_SEED, CLIENT_TO_SERVER_CONNECTION, ENCRYPTION_KEY_SEED, REALM_WIN_AUTH_SEED,
    SERVER_TO_CLIENT_CONNECTION, SESSION_KEY_SEED,
};
use crate::crypt::{AES128Companion, INITIALIZED_RSA, RSA};
use crate::opcodes::OpcodeClient;
use crate::packets::auth::Ping;
use crate::packets::auth::{AuthChallenge, AuthSession, EncryptedMode, Pong};
use crate::packets::{PacketHeader, RawClientPacket, ServerPacket};
use crate::utils::generate_session_key;
use crate::world_listener::WorldSessionHandler;
use crate::world_server::{NewSession, ServerEventEnum};
use crate::OpcodeServer;
use anyhow::anyhow;
use bytes::BytesMut;
use deku::prelude::*;
use hmac::{Mac, SimpleHmac};
use rand::Rng;
use rustycraft_common::Account;
use rustycraft_database::redis::RedisClient;
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

pub struct WorldClientSession {
    addr: SocketAddr,
    redis: RedisClient,
    rsa: &'static RSA,
    aes_companion: AES128Companion,
    client_socket_reader: ReadHalf<TcpStream>,
    client_socket_writer: WriteHalf<TcpStream>,
    world_server_events: Sender<ServerEventEnum>,
    server_challenge: [u8; 16],
    encryption_key: [u8; 16],
    session_key: [u8; 40],
}

type HmacSha256 = SimpleHmac<Sha256>;

impl WorldClientSession {
    async fn read_client_packet(&mut self) -> anyhow::Result<RawClientPacket> {
        let pkt_size = self.client_socket_reader.read_u32_le().await?;
        let mut aes_token = Vec::with_capacity(12);
        self.client_socket_reader.read_buf(&mut aes_token).await?;
        let mut packet = BytesMut::with_capacity(pkt_size as usize);
        self.client_socket_reader.read_buf(&mut packet).await?;

        let decrypted = self.aes_companion.decrypt(&packet, &aes_token)?;
        trace!(
            "Payload size: {}, aes_token: {:?}, packet: {:?}",
            &pkt_size,
            &aes_token,
            &decrypted
        );

        Ok(RawClientPacket {
            header: PacketHeader {
                size: pkt_size,
                tag: aes_token,
            },
            payload: decrypted.into(),
        })
    }

    async fn write_to_socket<T>(&mut self, data: T, opcode: OpcodeServer) -> anyhow::Result<()>
    where
        T: DekuContainerWrite,
    {
        let data = data.to_bytes()?;
        let mut result = BytesMut::with_capacity(data.len() + 2); // 2 for opcode
        result.extend(opcode.to_bytes()?);
        result.extend(data);
        let encrypted = self.aes_companion.encrypt(&result)?;
        let pkt = ServerPacket::new(encrypted.aes_tag, encrypted.cipher_text);
        trace!("Encrypted packet: {:?}", &pkt);
        self.client_socket_writer.write(&pkt.serialize()?).await?;
        Ok(())
    }

    async fn initial_packets(&mut self) -> anyhow::Result<()> {
        self.client_socket_writer
            .write(SERVER_TO_CLIENT_CONNECTION)
            .await?;
        let mut buf = BytesMut::with_capacity(CLIENT_TO_SERVER_CONNECTION.len());
        self.client_socket_reader.read_buf(&mut buf).await?;
        if buf.to_vec() != CLIENT_TO_SERVER_CONNECTION.to_vec() {
            return Err(anyhow!("Bad response from client."));
        };
        Ok(())
    }

    async fn init_encryption_state(&mut self) -> anyhow::Result<()> {
        let dos_challenge = rand::thread_rng().gen();
        let challenge = AuthChallenge::new(self.server_challenge.clone(), dos_challenge);
        self.write_to_socket(challenge, OpcodeServer::AuthChallenge)
            .await?;

        let packet = self.read_client_packet().await?;
        let (_, session_pkt): (_, AuthSession) = AuthSession::from_bytes((&packet.payload, 16))?;

        let acc: Account = self
            .redis
            .get(&session_pkt.realm_join_ticket)
            .await
            .unwrap();
        let mut session_secret = acc.client_secret;
        session_secret.extend(acc.server_secret);

        let mut key_hasher = sha2::Sha256::new();
        key_hasher.update(&session_secret);
        key_hasher.update(&REALM_WIN_AUTH_SEED);
        let digest_key_hash = key_hasher.finalize();

        let mut hmac_digester = HmacSha256::new_from_slice(&digest_key_hash).unwrap();
        hmac_digester.update(&session_pkt.local_challenge);
        hmac_digester.update(&self.server_challenge);
        hmac_digester.update(&AUTH_CHECK_SEED);
        let res = hmac_digester.finalize().into_bytes();
        assert_eq!(session_pkt.digest.as_slice(), &res[..24]);

        let mut key_data_hasher = sha2::Sha256::new();
        key_data_hasher.update(&session_secret);
        let key_data_hash = key_data_hasher.finalize();

        let mut session_key_hasher = HmacSha256::new_from_slice(&key_data_hash).unwrap();
        session_key_hasher.update(&self.server_challenge);
        session_key_hasher.update(&session_pkt.local_challenge);
        session_key_hasher.update(&SESSION_KEY_SEED);
        let session_key_seed = session_key_hasher.finalize().into_bytes();

        let session_key = generate_session_key::<Sha256>(&session_key_seed, 40);
        for (i, byte) in session_key.iter().enumerate() {
            self.session_key[i] = *byte;
        }

        let mut encryption_key_hasher = HmacSha256::new_from_slice(&self.session_key).unwrap();
        encryption_key_hasher.update(&session_pkt.local_challenge);
        encryption_key_hasher.update(&self.server_challenge);
        encryption_key_hasher.update(&ENCRYPTION_KEY_SEED);
        let encryption_key = encryption_key_hasher.finalize().into_bytes();
        for (i, b) in encryption_key[..16].iter().enumerate() {
            self.encryption_key[i] = *b;
        }

        Ok(())
    }

    async fn enter_encrypted_mode(&mut self) -> anyhow::Result<()> {
        let encrypted_mode = EncryptedMode::new(&self.rsa, &self.encryption_key);
        self.write_to_socket(encrypted_mode, OpcodeServer::EnterEncryptedMode)
            .await?;
        let packet = self.read_client_packet().await?;
        let (_, opcode) = OpcodeClient::from_bytes((&packet.payload, 0))?;
        assert_eq!(opcode, OpcodeClient::EnterEncryptedModeAck);
        self.aes_companion.init(&self.encryption_key)?;
        println!("KEY: {:?}", &self.encryption_key);
        Ok(())
    }

    async fn init_connection(&mut self) -> anyhow::Result<()> {
        self.initial_packets().await?;
        self.init_encryption_state().await?;
        self.enter_encrypted_mode().await?;

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
                    // writer.write(server_event)
                },
                Ok(client_event) = self.read_client_packet() => {
                    debug!(target: "WorldSession", "[{:?}] New packet received from client: {:?}", self.addr, client_event);
                    let (_, opcode) = OpcodeClient::from_bytes((&client_event.payload, 0))?;
                    let (_, ping) = Ping::from_bytes((&client_event.payload, 16))?;
                    let pong = Pong::from(ping);
                    self.write_to_socket(pong, OpcodeServer::Pong).await?;
                    // self.world_server_events.send(ServerEventEnum::NewClientPacket(self.addr, client_event)).await?;
                }
            };
        }
        Ok(())
    }
}
