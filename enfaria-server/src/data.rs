use std::net::SocketAddr;
use std::collections::HashMap;
use enfaria_common::Packet;

#[derive(Debug, Default)]
pub struct ServerData {
    pub beat: u64,
    pub positions: HashMap<UserId, UserPosition>,
    pub players: HashMap<SocketAddr, UserId>,
    pub send_queue: HashMap<UserId, Vec<Packet>>,
    pub receive_queue: HashMap<UserId, Vec<Packet>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct UserId(pub u64);

#[derive(Debug, Copy, Clone)]
pub struct UserPosition {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}
