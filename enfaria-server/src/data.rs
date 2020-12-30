use std::net::SocketAddr;
use std::collections::HashMap;
use enfaria_common::{
    {Packet, Position},
    map::Map,
};

#[derive(Debug, Default)]
pub struct ServerData {
    pub beat: u64,
    pub maps: HashMap<UserId, Map>,
    pub positions: HashMap<UserId, Position>,
    pub players: HashMap<SocketAddr, UserId>,
    pub send_queue: HashMap<UserId, Vec<Packet>>,
    pub receive_queue: HashMap<UserId, Vec<Packet>>,
    pub tokens: HashMap<UserId, String>,
    pub times: HashMap<UserId, u128>,
    pub usernames: HashMap<UserId, String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct UserId(pub u64);
