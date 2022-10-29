use rustycraft_world_server::SocketManager;

fn socket_manager() -> anyhow::Result<()> {
    let tokio_rt = tokio::runtime::Runtime::new()?;

    let _ = rustycraft_logging::init_logging();
    let session_manager = SocketManager::new();
    tokio_rt.block_on(session_manager.run_forever())?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    std::thread::spawn(|| socket_manager()).join().unwrap()?;
    Ok(())
}
