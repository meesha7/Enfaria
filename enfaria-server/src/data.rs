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
    pub map: Map,
    pub player: Player,
    pub send_queue: Vec<Packet>,
    pub receive_queue: Vec<Packet>,
}

impl User {
    pub fn new(id: UserId, ip: SocketAddr, username: String, token: String, map: Map, player: Player) -> Self {
        User {
            id,
            ip,
            username,
            token,
            time: get_timestamp(),
            map,
            player,
            send_queue: vec![],
            receive_queue: vec![],
        }
    }

    pub fn send_packet(&mut self, packet: Packet) {
        if self.token != packet.session_id {
            info!("Invalid session ID {:?}", &self.username);
            return;
        };

        if self.ip != packet.destination {
            info!("Invalid destination {:?}", &self.username);
            return;
        };

        self.send_queue.push(packet);
    }
}
