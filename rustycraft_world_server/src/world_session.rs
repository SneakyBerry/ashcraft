use crate::constants::{
    AUTH_CHECK_SEED, CLIENT_TO_SERVER_CONNECTION, REALM_WIN_AUTH_SEED, SERVER_TO_CLIENT_CONNECTION,
    SESSION_KEY,
};
use crate::opcodes::OpcodeClient;
use crate::packets::auth::{
    AuthChallenge, AuthResponse, AuthSession, AuthSuccessInfo, AuthWaitInfo, CharacterTemplate,
    CharacterTemplateClass, Class, EncryptedMode, GameTime, RaceClassAvailability,
};
use crate::packets::{PacketClient, PacketHeader, RawClientPacket, ServerPacket};
use crate::world_listener::WorldSessionHandler;
use crate::world_server::{NewSession, ServerEventEnum};
use crate::OpcodeServer;
use anyhow::anyhow;
use bytes::{Buf, Bytes, BytesMut};
use deku::prelude::*;
use hmac::{Mac, SimpleHmac};
use rand::Rng;
use rustycraft_protocol::expansions::Expansions;
use rustycraft_protocol::races::Races;
use rustycraft_protocol::rpc_responses::WowRpcResponse;
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

pub struct WorldClientSession {
    addr: SocketAddr,
    client_socket_reader: ReadHalf<TcpStream>,
    client_socket_writer: WriteHalf<TcpStream>,
    world_server_events: Sender<ServerEventEnum>,
    server_challenge: [u8; 16],
    encryption_key: [u8; 16],
    session_key: [u8; 40],
    encryption_enabled: bool,
}

type HmacSha256 = SimpleHmac<Sha256>;

impl WorldClientSession {
    async fn enter_encrypted_mode(&mut self) -> anyhow::Result<()> {
        let encrypted_mode = EncryptedMode::new(&Vec::new());
        let encrypt_mode_pkt =
            ServerPacket::new(OpcodeServer::EnterEncryptedMode, encrypted_mode, false)
                .serialize()?;
        println!("{:X}", &encrypt_mode_pkt);
        self.client_socket_writer
            .write(&encrypt_mode_pkt.to_vec())
            .await?;
        let packet = Self::read_client_packet(&mut self.client_socket_reader).await?;
        let (_, opcode) = OpcodeClient::from_bytes((&packet.payload, 0))?;
        println!("Opcode: {:?}, payload: {:?}", &opcode, &packet);
        self.encryption_enabled = true;
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
        let challenge_pkt =
            ServerPacket::new(OpcodeServer::AuthChallenge, challenge, false).serialize()?;
        self.client_socket_writer
            .write(&challenge_pkt.to_vec())
            .await?;

        let packet = Self::read_client_packet(&mut self.client_socket_reader).await?;
        let (_, opcode) = OpcodeClient::from_bytes((&packet.payload, 0))?;
        let (_, session_pkt): (_, AuthSession) = AuthSession::from_bytes((&packet.payload, 16))?;

        let mut key_hasher = sha2::Sha256::new();
        key_hasher.update(SESSION_KEY);
        key_hasher.update(REALM_WIN_AUTH_SEED.as_bytes());

        let digest_key_hash = key_hasher.finalize();
        println!("{:?}", &digest_key_hash);

        let mut hmac_digester = HmacSha256::new_from_slice(&digest_key_hash).unwrap();
        hmac_digester.update(&session_pkt.local_challenge);
        hmac_digester.update(&self.server_challenge);
        hmac_digester.update(&AUTH_CHECK_SEED);
        println!("{:?}", &session_pkt.local_challenge);
        println!("{:?}", &self.server_challenge);
        println!("{:?}", &AUTH_CHECK_SEED);
        let res = hmac_digester.finalize().into_bytes();
        println!(
            "CLIENT DIGEST:{} {:?}",
            session_pkt.digest.len(),
            &session_pkt.digest
        );
        println!("SERVER DIGEST:{} {:?}", res.len(), &res);

        // println!("Opcode: {:?}, payload: {:?}", &opcode, &session_pkt);
        Ok(())
    }

    async fn init_connection(&mut self) -> anyhow::Result<()> {
        self.initial_packets().await?;
        self.init_encryption_state().await?;
        self.enter_encrypted_mode().await?;

        Ok(())
    }

    async fn read_client_packet(
        reader: &mut ReadHalf<TcpStream>,
    ) -> anyhow::Result<RawClientPacket> {
        let pkt_size = reader.read_u32_le().await?;
        let mut aes_token = BytesMut::with_capacity(12);
        reader.read_buf(&mut aes_token).await?;
        let mut packet = bytes::BytesMut::with_capacity(pkt_size as usize);
        reader.read_buf(&mut packet).await?;
        // println!(
        //     "Payload size: {}, aes_token: {:X}, packet: {:X}",
        //     &pkt_size, &aes_token, &packet
        // );
        Ok(RawClientPacket {
            header: PacketHeader {
                size: pkt_size,
                tag: aes_token.to_vec(),
            },
            payload: packet.into(),
        })
    }
}

#[async_trait::async_trait]
impl WorldSessionHandler for WorldClientSession {
    fn new(socket: TcpStream, world_server_tx: Sender<ServerEventEnum>) -> anyhow::Result<Self> {
        let peer_addr = socket.peer_addr()?;
        let (reader, writer) = split(socket);
        Ok(WorldClientSession {
            addr: peer_addr,
            client_socket_reader: reader,
            client_socket_writer: writer,
            world_server_events: world_server_tx,
            server_challenge: rand::thread_rng().gen(),
            encryption_key: [0; 16],
            session_key: [0; 40],
            encryption_enabled: false,
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
                Ok(client_event) = Self::read_client_packet(&mut self.client_socket_reader) => {
                    debug!(target: "WorldSession", "[{:?}] New packet received from client: {:?}", self.addr, client_event);
                    self.world_server_events.send(ServerEventEnum::NewClientPacket(self.addr, client_event)).await?;
                }
            };
        }
        Ok(())
    }
}
