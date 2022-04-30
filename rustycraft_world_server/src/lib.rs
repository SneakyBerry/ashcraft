pub mod classes;
pub mod constants;
pub mod crypt;
pub mod guid;
pub mod opcodes;
pub mod packets;
pub mod races;
mod session_modules;
pub mod utils;
pub mod world_events;
pub mod world_listener;
pub mod world_server;
pub mod realm_server;
pub mod world_session;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;

use crate::guid::ObjectGuid;
use crate::opcodes::OpcodeServer;
use crate::packets::IntoServerPacket;
use std::fmt::Debug;
use tokio::sync::mpsc;
use crate::world_server::ServerEventEnum;

#[derive(Debug)]
pub enum ObjectEvent {}

impl ObjectEvent {
    fn get_target_guid(&self) -> ObjectGuid {
        todo!()
    }
}

#[async_trait::async_trait]
pub trait ServerObject: Send + Debug + Sync {
    async fn get_guid(&self) -> Option<ObjectGuid>;
    async fn interact(&self, callback: mpsc::Sender<ServerEventEnum>, event: ObjectEvent);
    async fn update(&self) {}
}
