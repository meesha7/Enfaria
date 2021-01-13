use crate::prelude::*;

#[derive(Debug, Default)]
pub struct ServerData {
    pub beat: u64,
    pub users: Vec<User>,
}

impl ServerData {
    pub fn user_by_id(&self, id: UserId) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }

    pub fn _user_by_id_mut(&mut self, id: UserId) -> Option<&mut User> {
        self.users.iter_mut().find(|u| u.id == id)
    }

    pub fn _user_by_ip(&self, ip: SocketAddr) -> Option<&User> {
        self.users.iter().find(|u| u.ip == ip)
    }

    pub fn user_by_ip_mut(&mut self, ip: SocketAddr) -> Option<&mut User> {
        self.users.iter_mut().find(|u| u.ip == ip)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct UserId(pub u64);

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub ip: SocketAddr,
    pub username: String,
    pub token: String,
    pub time: u128,
    pub position: Position,
    pub map: Map,
    pub send_queue: Vec<Packet>,
    pub receive_queue: Vec<Packet>,
}

impl User {
    pub fn new(id: UserId, ip: SocketAddr, username: String, token: String, map: Map) -> Self {
        User {
            id,
            ip,
            username,
            token,
            time: get_timestamp(),
            position: Position::default(),
            map,
            send_queue: vec![],
            receive_queue: vec![],
        }
    }
}
