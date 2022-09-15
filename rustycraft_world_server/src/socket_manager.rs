use crate::client_session::ClientSession;
use rustycraft_database::redis::RedisClient;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct SocketManager {
    bind_address: &'static str,
    redis: Arc<RedisClient>,
    connections: HashMap<SocketAddr, Arc<ClientSession>>,
}

impl SocketManager {
    pub fn new() -> Self {
        SocketManager {
            bind_address: "0.0.0.0:8085",
            redis: Arc::new(RedisClient::new().expect("Redis connection is not alive")),
            connections: Default::default(),
        }
    }

    pub async fn run_forever(mut self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.bind_address).await.unwrap();

        info!(target: "SocketManager", "World server listening on: {}", self.bind_address);

        loop {
            if let Ok((stream, addr)) = listener.accept().await {
                let session = Arc::new(ClientSession::new(stream, self.redis.clone()));
                tokio::spawn(session.clone().serve());
                self.connections.insert(addr, session);
            }
        }
    }
}
