use bytes::{Bytes, BytesMut};
use prost::Message;
use rustls::ServerConfig;
use rustycraft_protocol::bgs::protocol::Header;
use rustycraft_protocol::messages::RawMessage;
use rustycraft_protocol::rpc_responses::WowRpcResponse;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;

#[derive(Debug)]
pub enum SocketEvents {
    Close,
    Send(Bytes),
}

pub struct SocketManagerBuilder {}

impl SocketManagerBuilder {
    pub fn build(self, tls_context: ServerConfig) -> anyhow::Result<SocketManager> {
        let bind_address = "0.0.0.0:1119";
        Ok(SocketManager {
            bind_address,
            tls_context,
        })
    }
}

pub struct SocketManager {
    bind_address: &'static str,
    tls_context: ServerConfig,
}

impl SocketManager {
    pub fn builder() -> SocketManagerBuilder {
        SocketManagerBuilder {}
    }

    pub async fn handle_connection(
        addr: SocketAddr,
        socket: TlsStream<TcpStream>,
        response_ch: mpsc::Sender<RawMessage>,
        mut request_ch: mpsc::Receiver<SocketEvents>,
    ) -> Result<(), WowRpcResponse> {
        debug!(target: "SocketManager", "New connection from peer: {}", addr);
        let (mut sock_reader, mut sock_writer) = split(socket);
        #[allow(unreachable_code)]
        let writer = tokio::spawn(async move {
            loop {
                let headers_len = sock_reader.read_u16().await.ok()?;
                let mut read_buffer = BytesMut::with_capacity(headers_len as usize);
                sock_reader.read_buf(&mut read_buffer).await.ok()?;
                let headers: Header = Header::decode(&mut read_buffer).ok()?;
                let mut body_buf = BytesMut::with_capacity(headers.size? as usize);
                sock_reader.read_buf(&mut body_buf).await.ok()?;
                let payload = RawMessage {
                    headers,
                    data: body_buf.into(),
                };
                debug!(target: "SocketManager",
                    "[{:?}] Send new data to subscribers: {:?}",
                    addr, payload
                );
                response_ch.send(payload).await.ok()?;
            }
            Some(())
        });
        loop {
            match request_ch.recv().await {
                Some(SocketEvents::Close) | None => {
                    debug!(target: "SocketManager", "[{:?}] Received close event. Shutting down socket.", addr);
                    break;
                }
                Some(SocketEvents::Send(mut data)) => {
                    debug!(target: "SocketManager","[{:?}] Send data to client's socket: len: {} bytes, data: {:?}", addr, data.len(), &data);
                    let res = sock_writer.write(&mut data).await?;
                    assert_eq!(res, data.len());
                    sock_writer.flush().await?;
                }
            }
        }
        request_ch.close();
        sock_writer.flush().await?;
        sock_writer.shutdown().await?;
        writer.await?;
        debug!(target: "SocketManager", "[{:?}] Socket closed.", addr);
        Ok(())
    }

    pub async fn run_forever<T>(self) -> anyhow::Result<()>
    where
        T: SessionHandler,
    {
        let acceptor = TlsAcceptor::from(Arc::new(self.tls_context.clone()));
        let listener = TcpListener::bind(self.bind_address).await.unwrap();

        info!(target: "SocketManager", "Auth server listening on: {}", self.bind_address);

        loop {
            if let Ok((stream, peer_addr)) = listener.accept().await {
                let tls_stream = acceptor.accept(stream).await.ok();
                if let Some(tls_stream) = tls_stream {
                    let (req_tx, req_rx) = mpsc::channel(1024);
                    let (resp_tx, resp_rx) = mpsc::channel(1024);
                    tokio::spawn(Self::handle_connection(
                        peer_addr, tls_stream, resp_tx, req_rx,
                    ));
                    tokio::spawn(T::new(peer_addr, resp_rx, req_tx).handle());
                }
            }
        }
    }
}

#[async_trait::async_trait]
pub trait SessionHandler: 'static {
    fn new(
        addr: SocketAddr,
        rx: mpsc::Receiver<RawMessage>,
        tx: mpsc::Sender<SocketEvents>,
    ) -> Self;
    async fn handle(mut self) -> Result<(), SendError<SocketEvents>>;
}
