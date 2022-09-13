use rustycraft_auth_server::socket_manager::SocketManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = rustycraft_logging::init_logging();
    let session_manager = SocketManager::new();
    session_manager.run_forever().await?;
    Ok(())
}
