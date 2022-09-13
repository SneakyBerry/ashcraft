use crate::server::Client;
use tokio::net::TcpListener;

pub struct SocketManager {
    bind_address: &'static str,
}

impl SocketManager {
    pub fn new() -> Self {
        SocketManager {
            bind_address: "0.0.0.0:3724",
        }
    }

    pub async fn run_forever(self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.bind_address).await.unwrap();

        info!(target: "SocketManager", "Auth server listening on: {}", self.bind_address);

        loop {
            if let Ok((stream, peer_addr)) = listener.accept().await {
                println!("{:?}", &stream);
                println!("{:?}", &peer_addr);
                tokio::spawn(Client::new(stream).serve_connection());
            }
        }
    }
}
