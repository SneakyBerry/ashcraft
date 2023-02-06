use rustycraft_database::redis::RedisClient;
use rustycraft_world_server::realm::RealmHandler;
use rustycraft_world_server::session_handler::Connection;
use rustycraft_world_server::world::WorldHandler;
use rustycraft_world_server::SocketManager;
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

fn socket_manager(conn_receiver: UnboundedSender<Connection>) -> anyhow::Result<()> {
    let tokio_rt = tokio::runtime::Runtime::new()?;

    let _ = rustycraft_logging::init_logging();
    let session_manager = SocketManager::new(conn_receiver);
    tokio_rt.block_on(session_manager.run_forever())?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let redis = Arc::new(RedisClient::new().expect("Redis connection is not alive"));
    let (incoming_tx, incoming_rx) = tokio::sync::mpsc::unbounded_channel();
    let (world_tx, world_rx) = tokio::sync::mpsc::unbounded_channel();
    let mut realm_manager = RealmHandler::new(incoming_rx, world_tx, redis);
    let world_manager = WorldHandler::new(world_rx, incoming_tx.clone());

    let mut join_handles = vec![];
    join_handles.push(std::thread::spawn(|| socket_manager(incoming_tx).unwrap()));
    join_handles.push(std::thread::spawn(move || world_manager.run()));
    join_handles.push(std::thread::spawn(move || {
        let tokio_rt = tokio::runtime::Runtime::new().unwrap();
        tokio_rt.block_on(realm_manager.run());
    }));

    for h in join_handles {
        h.join().unwrap()
    }

    Ok(())
}
