use crate::crypt::{AES128Companion, RSA};
use crate::packets::auth::ResumeComms;
use crate::packets::characters::LoginVerifyWorld;
use crate::packets::objects::UpdateObject;
use crate::realm_server::Error;
use crate::session_modules::crypt::EncryptedModeTrait;
use crate::world_listener::SessionHandler;
use crate::ServerEventEnum;
use rand::Rng;
use rustycraft_database::redis::RedisClient;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{split, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};

#[derive(Debug)]
pub struct WorldSession {
    pub(crate) addr: SocketAddr,
    pub(crate) redis: RedisClient,
    pub(crate) aes_companion: AES128Companion,
    pub(crate) client_socket_reader: ReadHalf<TcpStream>,
    pub(crate) client_socket_writer: WriteHalf<TcpStream>,
    pub(crate) encryption_key: [u8; 16],
    pub(crate) session_key: [u8; 40],
    pub(crate) server_challenge: [u8; 16],
}

impl WorldSession {
    pub(crate) async fn init_connection(&mut self) -> Result<(), Error> {
        self.initial_packets().await?;
        let res = ResumeComms {};
        self.write_to_socket(Box::new(res)).await?;
        let res = UpdateObject::new();
        self.write_to_socket(Box::new(res)).await?;
        Ok(())
    }

    pub async fn handle_player_login(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl SessionHandler for WorldSession {
    fn new(socket: TcpStream, _: Option<mpsc::Sender<ServerEventEnum>>) -> anyhow::Result<Self> {
        let peer_addr = socket.peer_addr()?;
        let (reader, writer) = split(socket);
        Ok(WorldSession {
            redis: RedisClient::new().unwrap(),
            addr: peer_addr,
            client_socket_reader: reader,
            client_socket_writer: writer,
            encryption_key: [0; 16],
            session_key: [0; 40],
            aes_companion: AES128Companion::new(),
            server_challenge: rand::thread_rng().gen(),
        })
    }

    async fn handle(self) -> Result<(), Error> {
        let addr = self.addr.clone();
        let rw_locked_self = Arc::new(RwLock::new(self));
        rw_locked_self.write().await.init_connection().await?;
        loop {
            let client_event = rw_locked_self.write().await.read_client_packet().await;
            debug!(target: "WorldSession", "[{:?}] New packet received from client: {:?}", &addr, client_event);
            match client_event {
                Err(Error::SocketError(er)) => Err(er)?,
                _ => {
                    error!(
                        "[ {:?} ] Unhandled packet: {:?}",
                        rw_locked_self.read().await.addr,
                        client_event
                    );
                }
            };
        }
        Ok(())
    }
}
