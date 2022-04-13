use boring::hash::MessageDigest;
use boring::nid::Nid;
use boring::pkey::PKey;
use hmac::SimpleHmac;
use hmac::{Hmac, Mac};
use rustycraft_world_server::constants::{
    AUTH_CHECK_SEED, ENABLE_ENCRYPTION_SEED, ENCRYPTION_KEY_SEED, REALM_WIN_AUTH_SEED,
    SERVER_PRIVATE_KEY, SESSION_KEY_SEED,
};
use rustycraft_world_server::utils::generate_session_key;
use rustycraft_world_server::world_listener::WorldSocketManagerBuilder;
use rustycraft_world_server::world_server::WorldServerBuilder;
use rustycraft_world_server::world_session::WorldClientSession;
use sha2::Digest;
use sha2::Sha256;
use std::ffi::c_void;
use std::os::raw::{c_int, c_uint};

type HmacSha256 = Hmac<Sha256>;
use boring::rsa::{Padding, Rsa};
use boring::sign::Signer;
use boring_sys::RSA;
use rustycraft_world_server::crypt::AES128;

use aes_gcm::{AeadInPlace, Aes128Gcm, Key, Nonce, Tag}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};


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
