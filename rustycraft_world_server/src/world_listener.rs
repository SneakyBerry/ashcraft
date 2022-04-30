use crate::world_server::ServerEventEnum;
use crate::realm_server::Error;
use anyhow::anyhow;
use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum SocketEvents {
    Close,
    Send(Bytes),
}

pub struct SocketManagerBuilder {
    server_channel: Option<mpsc::Sender<ServerEventEnum>>,
    port: u16,
}

impl SocketManagerBuilder {
    pub fn new() -> SocketManagerBuilder {
        SocketManagerBuilder {
            server_channel: None,
            port: 0,
        }
    }
    pub fn set_server_channel(mut self, channel: mpsc::Sender<ServerEventEnum>) -> Self {
        self.server_channel = Some(channel);
        self
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn build(self) -> SocketManager {
        let bind_address = format!("0.0.0.0:{}", self.port);
        SocketManager {
            bind_address,
            server_channel: self.server_channel,
        }
    }
}

pub struct SocketManager {
    bind_address: String,
    server_channel: Option<mpsc::Sender<ServerEventEnum>>,
}

impl SocketManager {
    pub async fn run_forever<T>(self) -> anyhow::Result<()>
    where
        T: SessionHandler,
    {
        let listener = TcpListener::bind(&self.bind_address).await.unwrap();

        info!(target: "SocketManager", "Listening on: {}", self.bind_address);

        loop {
            if let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(T::new(stream, self.server_channel.clone())?.handle());
            }
        }
    }
}

pub enum Void {}

#[async_trait::async_trait]
pub trait SessionHandler: 'static {
    fn new(
        socket: TcpStream,
        world_server_tx: Option<mpsc::Sender<ServerEventEnum>>, // Channel for communicate with world server
    ) -> anyhow::Result<Self>
    where
        Self: Sized;
    async fn handle(self) -> Result<(), Error>;
}
