use rustycraft_world_server::world_listener::WorldSocketManagerBuilder;
use rustycraft_world_server::world_server::WorldServerBuilder;
use rustycraft_world_server::world_session::WorldClientSession;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = rustycraft_logging::init_logging();
    let mut world_server_builder = WorldServerBuilder::new();
    let world_server_channel = world_server_builder.get_event_sender();
    let world_server = world_server_builder.build()?;
    let mut world_socket_manager_builder = WorldSocketManagerBuilder::new();
    world_socket_manager_builder.set_world_server_channel(world_server_channel);
    let world_socket_manager = world_socket_manager_builder.build()?;
    tokio::spawn(world_socket_manager.run_forever::<WorldClientSession>());
    world_server.run_forever().await;
    Ok(())
}
