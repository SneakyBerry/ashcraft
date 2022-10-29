use crate::client_session::ClientSession;
use rustycraft_database::redis::RedisClient;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub struct SocketManager {
    bind_address: &'static str,
    redis: Arc<RedisClient>,
}

impl SocketManager {
    pub fn new() -> Self {
        SocketManager {
            bind_address: "0.0.0.0:8085",
            redis: Arc::new(RedisClient::new().expect("Redis connection is not alive")),
        }
    }

    pub async fn run_forever(mut self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.bind_address).await?;

        info!(target: "SocketManager", "World server listening on: {}", self.bind_address);

        loop {
            if let Ok((stream, _)) = listener.accept().await {
                let (tx, rx) = mpsc::channel(512);
                let tx = Arc::new(tx);
                tokio::spawn(ClientSession::new(stream, self.redis.clone(), rx, tx).serve());
            }
        }
    }
}
