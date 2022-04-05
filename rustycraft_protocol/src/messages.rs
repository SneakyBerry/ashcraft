use crate::bgs::protocol::connection::v1::ConnectResponse;
use crate::bgs::protocol::{Header, ProcessId};
use bytes::{Bytes, BytesMut};
use prost::Message as _;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct RawMessage {
    pub headers: Header,
    pub data: Bytes,
}

#[derive(Debug)]
pub struct IncomingMessage<I>
where
    I: prost::Message + Send + Default,
{
    pub message: I,
}

impl<I> IncomingMessage<I>
where
    I: prost::Message + Send + Default,
{
    pub fn parse_from_bytes(buf: BytesMut) -> Option<IncomingMessage<I>> {
        let message = I::decode(buf).ok()?;

        Some(IncomingMessage { message })
    }
}

#[derive(Debug)]
pub struct OutgoingMessage<O>
where
    O: Send + prost::Message + Default,
{
    pub headers: Header,
    pub message: Option<O>,
}

impl<O> OutgoingMessage<O>
where
    O: Send + prost::Message + Default,
{
    pub fn encode(&mut self, is_response: bool) -> Bytes {
        let message_len = self.message.as_ref().map_or(0, |m| m.encoded_len());
        self.headers.size = Some(message_len as u32);
        self.headers.service_id = if is_response { 0xFE } else { 0 };
        let buffer_len = self.headers.encoded_len() + message_len + 2;
        let mut response_buffer = BytesMut::with_capacity(buffer_len);
        response_buffer.extend((self.headers.encoded_len() as u16).to_be_bytes());
        self.headers
            .encode(&mut response_buffer)
            .expect("Encode headers error.");
        self.message.as_ref().map(|msg| {
            msg.encode(&mut response_buffer)
                .expect("Encode message error.")
        });
        response_buffer.into()
    }
}

pub trait LoggingAttributes {
    fn get_client_addr(&self) -> SocketAddr;
}

impl ProcessId {
    pub fn get_default() -> Self {
        ProcessId {
            label: std::process::id(),
            epoch: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as u32,
        }
    }
}

impl ConnectResponse {
    pub fn get_default() -> Self {
        ConnectResponse {
            server_time: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis() as u64,
            ),
            server_id: ProcessId::get_default(),
            ..Default::default()
        }
    }
}
