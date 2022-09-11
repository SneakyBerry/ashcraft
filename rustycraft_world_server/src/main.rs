use deku::{DekuContainerRead, DekuContainerWrite};
use rand::Rng;
use rustycraft_world_server::guid::{HighGuid, ObjectGuid};
use rustycraft_world_server::packets::auth::{AddrType, ConnectTo, ConnectToSerial};
use rustycraft_world_server::realm_server::RealmServer;
use rustycraft_world_server::utils::{pack_u128, unpack_u128};
use rustycraft_world_server::world_listener::SocketManagerBuilder;
use rustycraft_world_server::world_server::WorldServerBuilder;
use rustycraft_world_server::world_session::WorldSession;
use std::collections::HashSet;
use std::fmt::Debug;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = rustycraft_logging::init_logging();
    let mut world_server_builder = WorldServerBuilder::new();
    let world_server_channel = world_server_builder.get_event_sender();
    let world_server = world_server_builder.build()?;
    let mut realm_socket_manager_builder = SocketManagerBuilder::new().set_port(9900);
    let realm_socket_manager = realm_socket_manager_builder.build();
    tokio::spawn(realm_socket_manager.run_forever::<RealmServer>());
    let mut realm_socket_manager_builder = SocketManagerBuilder::new()
        .set_server_channel(world_server_channel)
        .set_port(9901);
    let realm_socket_manager = realm_socket_manager_builder.build();
    tokio::spawn(realm_socket_manager.run_forever::<WorldSession>());
    world_server.run_forever().await;
    Ok(())
}
