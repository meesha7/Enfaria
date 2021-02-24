//! Packets are the actual struct sent over the network, serialized.
use crate::Message;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub session_id: String,
    pub destination: SocketAddr,
    pub message: Message,
}
