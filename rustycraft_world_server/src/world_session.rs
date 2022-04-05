use crate::packets::PacketClient;
use crate::world_listener::WorldSessionHandler;
use crate::world_server::{NewSession, ServerEventEnum};
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender};

const SERVER_TO_CLIENT_CONNECTION: &[u8; 53] =
    b"WORLD OF WARCRAFT CONNECTION - SERVER TO CLIENT - V2\n";
const CLIENT_TO_SERVER_CONNECTION: &[u8; 53] =
    b"WORLD OF WARCRAFT CONNECTION - CLIENT TO SERVER - V2\n";

pub struct WorldClientSession {
    client_socket: TcpStream,
    world_server_events: Sender<ServerEventEnum>,
}

impl WorldClientSession {
    async fn init_connection(&mut self) -> anyhow::Result<()> {
        todo!();
    }

    async fn read_client_packet(reader: &mut ReadHalf<TcpStream>) -> anyhow::Result<PacketClient> {
        let mut buf = bytes::BytesMut::new();
        reader.read_buf(&mut buf).await?;
        // ClientEvent::parse_from_bytes(buf)?
        todo!()
    }
}

#[async_trait::async_trait]
impl WorldSessionHandler for WorldClientSession {
    fn new(socket: TcpStream, world_server_tx: Sender<ServerEventEnum>) -> Self {
        WorldClientSession {
            client_socket: socket,
            world_server_events: world_server_tx,
        }
    }

    async fn handle(mut self) -> anyhow::Result<()> {
        self.init_connection().await?;
        let (world_tx, mut world_rx) = mpsc::channel(2048);
        let peer_addr = self.client_socket.peer_addr()?;
        self.world_server_events
            .send(ServerEventEnum::NewSession(NewSession {
                addr: peer_addr,
                sender: world_tx,
            }))
            .await?;
        let (mut reader, mut writer) = split(self.client_socket);
        loop {
            tokio::select! {
                Some(server_event) = world_rx.recv() => {
                    debug!(target: "WorldSession", "[{:?}] New packet received from server: {:?}", peer_addr, server_event);
                    writer.write(SERVER_TO_CLIENT_CONNECTION).await?;
                },
                Ok(client_event) = Self::read_client_packet(&mut reader) => {
                    debug!(target: "WorldSession", "[{:?}] New packet received from client: {:?}", peer_addr, client_event);
                    self.world_server_events.send(ServerEventEnum::NewClientPacket(peer_addr, client_event)).await?;
                }
            };
        }
        Ok(())
    }
}
