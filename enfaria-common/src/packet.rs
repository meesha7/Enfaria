use crate::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub beat: u64,
    pub session_id: String,
    pub destination: SocketAddr,
    pub message: Message,
}
