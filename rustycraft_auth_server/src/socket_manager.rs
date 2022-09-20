use crate::server::Client;
use rustycraft_database::redis::RedisClient;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct SocketManager {
    // TODO: Move to config
    bind_address: &'static str,
    redis_client: Arc<RedisClient>,
}

impl SocketManager {
    pub fn new() -> Self {
        SocketManager {
            bind_address: "0.0.0.0:3724",
            redis_client: Arc::new(RedisClient::new().expect("Can not connect to Redis")),
        }
    }

    pub async fn run_forever(self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.bind_address).await?;

        info!(target: "SocketManager", "Auth server listening on: {}", self.bind_address);

        loop {
            if let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(Client::new(stream, self.redis_client.clone()).serve_connection());
            }
        }
    }
}
