use crate::world_server::ServerEventEnum;
use anyhow::anyhow;
use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum SocketEvents {
    Close,
    Send(Bytes),
}

pub struct WorldSocketManagerBuilder {
    world_server_channel: Option<mpsc::Sender<ServerEventEnum>>,
}

impl WorldSocketManagerBuilder {
    pub fn new() -> WorldSocketManagerBuilder {
        WorldSocketManagerBuilder {
            world_server_channel: None,
        }
    }
    pub fn set_world_server_channel(
        &mut self,
        channel: mpsc::Sender<ServerEventEnum>,
    ) -> &mut Self {
        self.world_server_channel = Some(channel);
        self
    }

    pub fn build(self) -> anyhow::Result<WorldSocketManager> {
        let bind_address = "0.0.0.0:9900";
        Ok(WorldSocketManager {
            bind_address,
            world_server_channel: self
                .world_server_channel
                .ok_or_else(|| anyhow!("World server sender did not set"))?,
        })
    }
}

pub struct WorldSocketManager {
    bind_address: &'static str,
    world_server_channel: mpsc::Sender<ServerEventEnum>,
}

impl WorldSocketManager {
    pub async fn run_forever<T>(self) -> anyhow::Result<()>
    where
        T: WorldSessionHandler,
    {
        let listener = TcpListener::bind(self.bind_address).await.unwrap();

        info!(target: "WorldSocketManager", "World server listening on: {}", self.bind_address);

        loop {
            if let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(T::new(stream, self.world_server_channel.clone())?.handle());
            }
        }
    }
}

#[async_trait::async_trait]
pub trait WorldSessionHandler: 'static {
    fn new(
        socket: TcpStream,
        world_server_tx: mpsc::Sender<ServerEventEnum>, // Channel for communicate with world server
    ) -> anyhow::Result<Self>
    where
        Self: Sized;
    async fn handle(mut self) -> anyhow::Result<()>;
}
