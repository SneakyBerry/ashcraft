use crate::opcodes::{OpcodeClient, OpcodeServer};
use std::net::SocketAddr;

#[derive(Debug)]
pub struct ClientEvent {
    pub opcode: OpcodeClient,
    pub sender: SocketAddr,
}

#[derive(Debug)]
pub struct ServerEvent {
    pub opcode: OpcodeServer,
}
