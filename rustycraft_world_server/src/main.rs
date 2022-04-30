use rand::Rng;
use rustycraft_world_server::guid::{HighGuid, ObjectGuid};
use rustycraft_world_server::packets::auth::{AddrType, ConnectTo, ConnectToSerial};
use rustycraft_world_server::realm_server::RealmServer;
use rustycraft_world_server::utils::{pack_u128, unpack_u128};
use rustycraft_world_server::world_listener::SocketManagerBuilder;
use rustycraft_world_server::world_server::WorldServerBuilder;
use rustycraft_world_server::world_session::WorldSession;
use deku::{DekuContainerRead, DekuContainerWrite};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = rustycraft_logging::init_logging();
    let guid: u128 = 0xBC42446B8068908700000300015F1B88;
    println!("{:b}", &guid);
    let a = guid.to_le_bytes();
    let res: ObjectGuid = ObjectGuid::from_bytes((&a, 0)).unwrap().1;
    println!("{:?}", &res);
    println!("{:?}", &res.to_bytes());
    let res: ObjectGuid = ObjectGuid::from_bytes((&res.to_bytes().unwrap(), 0)).unwrap().1;
    println!("{:?}", &res);

    // let mut world_server_builder = WorldServerBuilder::new();
    // let world_server_channel = world_server_builder.get_event_sender();
    // let world_server = world_server_builder.build()?;
    // let mut realm_socket_manager_builder = SocketManagerBuilder::new().set_port(9900);
    // let realm_socket_manager = realm_socket_manager_builder.build();
    // tokio::spawn(realm_socket_manager.run_forever::<RealmServer>());
    // let mut realm_socket_manager_builder = SocketManagerBuilder::new()
    //     .set_server_channel(world_server_channel)
    //     .set_port(9901);
    // let realm_socket_manager = realm_socket_manager_builder.build();
    // tokio::spawn(realm_socket_manager.run_forever::<WorldSession>());
    // world_server.run_forever().await;
    Ok(())
}
