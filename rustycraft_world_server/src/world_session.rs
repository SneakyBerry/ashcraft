use crate::opcodes::OpcodeClient;
use crate::packets::auth::{
    AuthChallenge, AuthResponse, AuthSession, AuthSuccessInfo, AuthWaitInfo,
};
use crate::packets::{PacketClient, PacketHeader, RawClientPacket, ServerPacket};
use crate::world_listener::WorldSessionHandler;
use crate::world_server::{NewSession, ServerEventEnum};
use crate::OpcodeServer;
use anyhow::anyhow;
use bytes::{Buf, Bytes, BytesMut};
use deku::prelude::*;
use rand::Rng;
use rustycraft_protocol::errors::WowRpcResponse;
use std::net::SocketAddr;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

const SERVER_TO_CLIENT_CONNECTION: &[u8; 53] =
    b"WORLD OF WARCRAFT CONNECTION - SERVER TO CLIENT - V2\n";
const CLIENT_TO_SERVER_CONNECTION: &[u8; 53] =
    b"WORLD OF WARCRAFT CONNECTION - CLIENT TO SERVER - V2\n";

pub struct WorldClientSession {
    addr: SocketAddr,
    client_socket_reader: ReadHalf<TcpStream>,
    client_socket_writer: WriteHalf<TcpStream>,
    world_server_events: Sender<ServerEventEnum>,
    server_challenge: [u8; 16],
    encryption_enabled: bool,
}

impl WorldClientSession {
    async fn init_connection(&mut self) -> anyhow::Result<()> {
        self.client_socket_writer
            .write(SERVER_TO_CLIENT_CONNECTION)
            .await?;
        let mut buf = BytesMut::with_capacity(CLIENT_TO_SERVER_CONNECTION.len());
        self.client_socket_reader.read_buf(&mut buf).await?;
        if buf.to_vec() != CLIENT_TO_SERVER_CONNECTION.to_vec() {
            return Err(anyhow!("Bad response from client."));
        };
        let challenge = AuthChallenge::new(self.server_challenge.clone(), rand::thread_rng().gen());
        let challenge_pkt =
            ServerPacket::new(OpcodeServer::AuthChallenge, challenge, false).serialize()?;
        println!("{:X}", &challenge_pkt);
        self.client_socket_writer
            .write(&challenge_pkt.to_vec())
            .await?;

        let packet = Self::read_client_packet(&mut self.client_socket_reader).await?;
        let (_, opcode) = OpcodeClient::from_bytes((&packet.payload, 0))?;
        let (_, session_pkt) = AuthSession::from_bytes((&packet.payload, 16))?;
        println!("Opcode: {:?}, payload: {:?}", &opcode, &session_pkt);
        // 080000000000000000000000000000006D25000000000101
        // 080000000000000000000000000000006D25000000000000
        // 070000000000000000000000000000006D250000000000
        // 070000000000000000000000000000006D2500000000C0
        // let success_info = AuthSuccessInfo;
        let auth_response = AuthResponse::new(WowRpcResponse::Ok, None, None);
        let auth_pkt =
            ServerPacket::new(OpcodeServer::AuthResponse, auth_response, false).serialize()?;
        println!("{:X}", &auth_pkt);
        self.client_socket_writer.write(&auth_pkt.to_vec()).await?;
        let packet = Self::read_client_packet(&mut self.client_socket_reader).await?;
        let (_, opcode) = OpcodeClient::from_bytes((&packet.payload, 0))?;
        println!("Opcode: {:?}, payload: {:?}", &opcode, &packet);

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
        println!(
            "Payload size: {}, aes_token: {:X}, packet: {:X}",
            &pkt_size, &aes_token, &packet
        );
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
