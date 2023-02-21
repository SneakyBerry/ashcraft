use rustycraft_database::redis::RedisClient;
use rustycraft_world_server::realm::RealmHandler;
use rustycraft_world_server::session_handler::Connection;
use rustycraft_world_server::world::WorldHandler;
use rustycraft_world_server::SocketManager;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use tokio::sync::mpsc::UnboundedSender;

fn socket_manager(conn_receiver: UnboundedSender<Connection>) -> anyhow::Result<()> {
    let tokio_rt = tokio::runtime::Runtime::new()?;

    let session_manager = SocketManager::new(conn_receiver);
    tokio_rt.block_on(session_manager.run_forever())?;
    Ok(())
}

fn unpark_on_panic<F: FnOnce() -> R + Send + 'static, R: Send + 'static>(
    name: &str,
    child: F,
) -> JoinHandle<thread::Result<R>> {
    let main = thread::current();
    thread::Builder::new()
        .name(name.into())
        .spawn(move || {
            let result = catch_unwind(AssertUnwindSafe(child));
            main.unpark();
            result
        })
        .expect("Can't spawn world manager thread")
}

fn main() -> thread::Result<()> {
    let _ = rustycraft_logging::init_logging();
    let (incoming_tx, incoming_rx) = tokio::sync::mpsc::unbounded_channel();
    let (world_tx, world_rx) = tokio::sync::mpsc::unbounded_channel();

    let socket_mgr_incoming_tx = incoming_tx.clone();
    let threads = [
        unpark_on_panic("Socket manager", move || {
            socket_manager(socket_mgr_incoming_tx).unwrap()
        }),
        unpark_on_panic("World manager", move || {
            let world_manager = WorldHandler::new(world_rx, incoming_tx);
            world_manager.run()
        }),
        unpark_on_panic("Realm manager", || {
            let tokio_rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let redis = Arc::new(RedisClient::new().expect("Redis connection is not alive"));
            let realm_manager = RealmHandler::new(incoming_rx, world_tx, redis);
            tokio_rt.block_on(realm_manager.run());
        }),
    ];

    while threads.iter().all(|t| !t.is_finished()) {
        thread::park();
    }

    let stopped_thread = threads.into_iter().find(|t| t.is_finished()).unwrap();
    resume_unwind(stopped_thread.join().unwrap().unwrap_err())
}
