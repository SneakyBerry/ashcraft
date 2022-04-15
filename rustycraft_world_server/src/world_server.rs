use crate::opcodes::OpcodeClient;
use crate::packets::auth::{Ping, Pong};
use crate::packets::{ClientPacket, IntoServerPacket, RawClientPacket};
use crate::OpcodeServer;
use anyhow::anyhow;
use deku::DekuContainerRead;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct NewSession {
    pub addr: SocketAddr,
    pub sender: mpsc::Sender<Box<dyn IntoServerPacket>>,
}

#[derive(Debug)]
pub enum ServerEventEnum {
    NewSession(NewSession),
    NewClientPacket(SocketAddr, ClientPacket),
}

#[derive(Debug)]
pub struct WorldServer {
    connections: HashMap<SocketAddr, mpsc::Sender<Box<dyn IntoServerPacket>>>,
    events: mpsc::Receiver<ServerEventEnum>,
}

#[derive(Debug)]
pub struct WorldServerBuilder {
    events: Option<mpsc::Receiver<ServerEventEnum>>,
}

impl WorldServerBuilder {
    pub fn new() -> WorldServerBuilder {
        WorldServerBuilder { events: None }
    }

    pub fn get_event_sender(&mut self) -> mpsc::Sender<ServerEventEnum> {
        let (tx, rx) = mpsc::channel(4096);
        self.events = Some(rx);
        tx
    }

    pub fn build(self) -> anyhow::Result<WorldServer> {
        Ok(WorldServer {
            connections: Default::default(),
            events: self
                .events
                .ok_or_else(|| anyhow!("Events channel did not set"))?,
        })
    }
}

impl WorldServer {
    pub async fn run_forever(mut self) {
        loop {
            let message = self.events.recv().await;
            match message {
                Some(ServerEventEnum::NewSession(session)) => {
                    self.connections.insert(session.addr, session.sender);
                }
                Some(ServerEventEnum::NewClientPacket(sender, packet)) => {
                    let conn = self.connections.get(&sender).unwrap();
                    match packet {
                        ClientPacket::Ping(ping) => {
                            let pong = Pong::from(ping);
                            conn.send(Box::new(pong)).await.unwrap();
                        },
                        _ => ()
                    };
                }
                None => break,
            }
        }
    }
}
