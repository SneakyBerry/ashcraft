pub mod config;
pub mod dispatcher;
mod realmlist;
pub mod services;
pub mod socket_manager;
mod utils;
pub mod web_handler;
mod web_models;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::socket_manager::{SessionHandler, SocketEvents};
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, rsa_private_keys};
use rustycraft_protocol::bgs::protocol::account::v1::AccountService;
use rustycraft_protocol::bgs::protocol::authentication::v1::AuthenticationService;
use rustycraft_protocol::bgs::protocol::connection::v1::ConnectionService;
use rustycraft_protocol::bgs::protocol::game_utilities::v1::GameUtilitiesService;
use rustycraft_protocol::bgs::protocol::{Header, NoData};
use rustycraft_protocol::errors::WowRpcResponse;
use rustycraft_protocol::messages::{LoggingAttributes, OutgoingMessage, RawMessage};
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Receiver, Sender};

pub enum State {
    Created,
    Initialized,
    Running,
    Stopped,
}

pub fn load_certs(path: &str) -> anyhow::Result<Vec<Certificate>> {
    Ok(certs(&mut BufReader::new(File::open(path)?))
        .map(|mut c| c.drain(..).map(Certificate).collect())?)
}

pub fn load_keys(path: &str) -> anyhow::Result<Vec<PrivateKey>> {
    Ok(rsa_private_keys(&mut BufReader::new(File::open(path)?))
        .map(|mut c| c.drain(..).map(PrivateKey).collect())?)
}

pub struct Server {
    token: u8,
    addr: SocketAddr,
    rx: Receiver<RawMessage>,
    tx: Sender<SocketEvents>,
}

impl Server {
    async fn handle_error(&mut self, error: WowRpcResponse) -> Result<(), SendError<SocketEvents>> {
        let headers = Header {
            status: Some(error as u32),
            ..Default::default()
        };
        let mut msg = OutgoingMessage::<NoData> {
            headers,
            message: None,
        };
        self.tx.send(SocketEvents::Send(msg.encode(true))).await?;
        Ok(())
    }
}

impl LoggingAttributes for Server {
    fn get_client_addr(&self) -> SocketAddr {
        self.addr
    }
}

#[async_trait::async_trait]
impl SessionHandler for Server {
    fn new(addr: SocketAddr, rx: Receiver<RawMessage>, tx: Sender<SocketEvents>) -> Self {
        Server {
            token: 0,
            addr,
            rx,
            tx,
        }
    }

    async fn handle(mut self) -> Result<(), SendError<SocketEvents>> {
        loop {
            let msg = self.rx.recv().await;
            let response = match msg {
                Some(message) => match message.headers.service_hash {
                    Some(<Self as ConnectionService>::ORIGINAL_HASH) => {
                        ConnectionService::dispatch(&mut self, message).await
                    }
                    Some(<Self as AuthenticationService>::ORIGINAL_HASH) => {
                        AuthenticationService::dispatch(&mut self, message).await
                    }
                    Some(<Self as AccountService>::ORIGINAL_HASH) => {
                        AccountService::dispatch(&mut self, message).await
                    }
                    Some(<Self as GameUtilitiesService>::ORIGINAL_HASH) => {
                        GameUtilitiesService::dispatch(&mut self, message).await
                    }
                    _ => Err(WowRpcResponse::NotImplemented),
                },
                None => break,
            };
            match response {
                Ok(data) => self.tx.send(SocketEvents::Send(data)).await?,
                Err(e) => {
                    self.handle_error(e).await?;
                    break;
                }
            };
            self.token += 1;
        }
        Ok(())
    }
}
