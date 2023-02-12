use crate::session_handler::Connection;
use anyhow::anyhow;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub struct SocketManager {
    bind_address: &'static str,
    realm_manager_sender: mpsc::UnboundedSender<Connection>,
}

impl SocketManager {
    pub fn new(realm_manager_sender: mpsc::UnboundedSender<Connection>) -> Self {
        SocketManager {
            bind_address: "0.0.0.0:8085",
            realm_manager_sender,
        }
    }

    pub async fn run_forever(self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.bind_address).await?;

        info!(message = "World server listening", addr = self.bind_address);

        loop {
            if let Ok((stream, _)) = listener.accept().await {
                self.realm_manager_sender
                    .send(Connection::new(stream.peer_addr().unwrap(), stream))
                    .map_err(|e| {
                        anyhow!("Can't send new connection: peer {:?}", e.0.peer_addr())
                    })?;
            }
        }
    }
}
