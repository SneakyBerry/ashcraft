pub mod core;
pub mod realm;
pub mod server;
pub mod session_handler;
pub mod socket_manager;
pub mod world;
pub mod world_manager;

pub use socket_manager::SocketManager;

#[macro_use]
extern crate tracing;

mod crate_macro {
    #[macro_export]
    macro_rules! parse_or_skip {
        () => {};
    }
}
