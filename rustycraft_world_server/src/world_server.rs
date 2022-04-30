use crate::guid::ObjectGuid;
use crate::opcodes::OpcodeClient;
use crate::packets::auth::{Ping, Pong};
use crate::packets::characters::EnumCharactersResult;
use crate::packets::{ClientPacket, IntoServerPacket, RawClientPacket};
use crate::{ObjectEvent, OpcodeServer, ServerObject};
use anyhow::anyhow;
use deku::DekuContainerRead;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::realm_server::RealmServer;
use tokio::sync::{mpsc, RwLock};

#[derive(Debug)]
pub enum ServerEventEnum {
    NewObject(Box<dyn ServerObject>),
    NewEvent(ObjectEvent),
    ObjectDelete(ObjectGuid),
}

#[derive(Debug)]
pub struct WorldServer {
    objects: HashMap<ObjectGuid, Box<dyn ServerObject>>,
    event_receiver: mpsc::Receiver<ServerEventEnum>,
    event_sender: mpsc::Sender<ServerEventEnum>,
}

#[derive(Debug)]
pub struct WorldServerBuilder {
    event_receiver: Option<mpsc::Receiver<ServerEventEnum>>,
    event_sender: Option<mpsc::Sender<ServerEventEnum>>,
}

impl WorldServerBuilder {
    pub fn new() -> WorldServerBuilder {
        WorldServerBuilder {
            event_receiver: None,
            event_sender: None,
        }
    }

    pub fn get_event_sender(&mut self) -> mpsc::Sender<ServerEventEnum> {
        let (tx, rx) = mpsc::channel(4096);
        self.event_receiver = Some(rx);
        self.event_sender = Some(tx.clone());
        tx
    }

    pub fn build(self) -> anyhow::Result<WorldServer> {
        Ok(WorldServer {
            objects: Default::default(),
            event_receiver: self
                .event_receiver
                .ok_or_else(|| anyhow!("Events channel did not set"))?,
            event_sender: self
                .event_sender
                .ok_or_else(|| anyhow!("Events channel did not set"))?,
        })
    }
}

impl WorldServer {
    pub async fn run_forever(mut self) {
        loop {
            let message = self.event_receiver.recv().await;
            match message {
                Some(ServerEventEnum::NewObject(object)) => {
                    let guid = object.get_guid().await;
                    if let Some(guid) = guid {
                        self.objects.insert(guid, object);
                    }
                }
                Some(ServerEventEnum::NewEvent(event)) => {
                    let target = event.get_target_guid();
                    if self.objects.contains_key(&target) {
                        let target = self.objects.get(&target).unwrap();
                        target.interact(self.event_sender.clone(), event).await;
                    }
                }
                /// Produced if object no longer in world. e.g character logout
                Some(ServerEventEnum::ObjectDelete(guid)) => {
                    self.objects.remove(&guid);
                }
                None => break,
            }
        }
    }
}
