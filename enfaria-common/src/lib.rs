use std::net::SocketAddr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub beat: u64,
    pub destination: SocketAddr,
    pub command: Command,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Connect,
    Quit,
}
