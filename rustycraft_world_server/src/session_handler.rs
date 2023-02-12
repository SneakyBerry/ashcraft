use bytes::BytesMut;
use deku::DekuContainerRead;
use rustycraft_logging::Valuable;
use rustycraft_world_packets::opcodes::Opcode;
use rustycraft_world_packets::{ClientPacket, ServerPacket};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex, OnceCell};
use wow_srp::wrath_header::ServerCrypto;

#[derive(Debug)]
pub enum ConnectionState {
    New,
    Handshake,
    Auth,
    Game,
}

pub struct Connection {
    pub state: ConnectionState,
    peer_addr: SocketAddr,
    encryption: Arc<Mutex<OnceCell<ServerCrypto>>>,
    sender: mpsc::UnboundedSender<Box<dyn ServerPacket>>,
    receiver: mpsc::UnboundedReceiver<ClientPacket>,
}

impl Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("state", &self.state)
            .field("peer_addr", &self.peer_addr)
            .finish()
    }
}

impl Connection {
    pub fn receiver(&mut self) -> &mut mpsc::UnboundedReceiver<ClientPacket> {
        &mut self.receiver
    }
    pub fn sender(&self) -> &mpsc::UnboundedSender<Box<dyn ServerPacket>> {
        &self.sender
    }

    pub fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }

    pub fn encryption(&self) -> Arc<Mutex<OnceCell<ServerCrypto>>> {
        self.encryption.clone()
    }

    pub fn new(peer_addr: SocketAddr, socket: TcpStream) -> Self {
        let encryption = Arc::new(Mutex::new(OnceCell::new()));
        let (reader, writer) = socket.into_split();
        let (client_tx, client_rx) = mpsc::unbounded_channel();
        let (server_tx, server_rx) = mpsc::unbounded_channel();
        tokio::spawn(read_socket(reader, encryption.clone(), client_tx));
        tokio::spawn(write_socket(writer, encryption.clone(), server_rx));
        Self {
            peer_addr,
            state: ConnectionState::Handshake,
            encryption,
            sender: server_tx,
            receiver: client_rx,
        }
    }
}

async fn read_socket(
    mut reader: OwnedReadHalf,
    encryption: Arc<Mutex<OnceCell<ServerCrypto>>>,
    tx: mpsc::UnboundedSender<ClientPacket>,
) -> anyhow::Result<()> {
    // TODO: Reuse buffers
    let mut headers = [0; 6];
    loop {
        if reader.read_exact(&mut headers).await? > 0 {
            {
                if let Some(encryption) = encryption.lock().await.get_mut() {
                    encryption.decrypt(&mut headers);
                }
            }
            // Size includes opcode with len 4 that already been parsed
            let pkt_size = u16::from_be_bytes(headers[0..2].try_into()?).saturating_sub(4);
            let (_, opcode) = Opcode::from_bytes((&headers[2..], 0))?;
            let mut data = BytesMut::zeroed(pkt_size as usize);
            reader.read_exact(&mut data).await?;
            trace!(
                addr = %reader.peer_addr().expect("Peer without IP"),
                opcode = ?opcode,
                size = %pkt_size,
                body = ?data
            );
            if let Ok(packet) = ClientPacket::parse(opcode, data.freeze()) {
                tx.send(packet)?;
            } else {
                warn!(message = "Unknown packet", opcode = ?opcode);
            }
        }
    }
}

// TODO: Error propagation
pub(crate) async fn write_socket(
    mut writer: OwnedWriteHalf,
    encryption: Arc<Mutex<OnceCell<ServerCrypto>>>,
    mut rx: mpsc::UnboundedReceiver<Box<dyn ServerPacket>>,
) -> anyhow::Result<()> {
    while let Some(mut data) = rx.recv().await {
        let payload = data.encode(encryption.lock().await.get_mut().map(|e| e.encrypter()))?;
        writer
            .write_all(&payload)
            .await
            .map_err(anyhow::Error::from)?
    }
    Ok(())
}
