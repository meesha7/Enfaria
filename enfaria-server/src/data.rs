use crate::prelude::*;
use async_std::net::TcpStream;

#[derive(Default)]
pub struct ServerData {
    pub beat: u64,
    pub users: Vec<User>,
    pub state: State,
}

impl ServerData {
    pub fn new() -> Self {
        Self {
            state: State::new(state::Mode::Server),
            ..Default::default()
        }
    }

    pub fn user_by_id(&self, id: UserId) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }

    pub fn user_by_id_mut(&mut self, id: UserId) -> Option<&mut User> {
        self.users.iter_mut().find(|u| u.id == id)
    }

    pub fn user_by_ip(&self, ip: SocketAddr) -> Option<&User> {
        self.users.iter().find(|u| u.ip == ip)
    }

    pub fn user_by_ip_mut(&mut self, ip: SocketAddr) -> Option<&mut User> {
        self.users.iter_mut().find(|u| u.ip == ip)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct UserId(pub u64);

#[derive(Debug)]
pub struct User {
    pub id: UserId,
    pub ip: SocketAddr,
    pub stream: TcpStream,
    pub username: String,
    pub token: String,
    pub time: u128,
    pub send_queue: Vec<Packet>,
    pub receive_queue: Vec<Packet>,
}

impl User {
    pub fn new(id: UserId, ip: SocketAddr, stream: TcpStream, username: String, token: String) -> Self {
        User {
            id,
            ip,
            stream,
            username,
            token,
            time: get_timestamp(),
            send_queue: vec![],
            receive_queue: vec![],
        }
    }

    pub fn queue_packet(&mut self, beat: u64, message: Message) {
        let packet = Packet {
            beat,
            session_id: self.token.clone(),
            destination: self.ip,
            message,
        };

        self.send_queue.push(packet);
    }
}
