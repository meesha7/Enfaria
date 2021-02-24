use crate::prelude::*;
use async_std::net::TcpStream;

#[derive(Default)]
pub struct Server {
    pub state: State,
    pub users: Vec<User>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            state: State::new(state::Mode::Server),
            ..Default::default()
        }
    }

    pub fn user_by_username(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|ref u| u.username == username)
    }

    pub fn user_by_username_mut(&mut self, username: &str) -> Option<&mut User> {
        self.users.iter_mut().find(|ref u| u.username == username)
    }

    pub fn user_by_ip(&self, ip: SocketAddr) -> Option<&User> {
        self.users.iter().find(|u| u.ip == ip)
    }

    pub fn user_by_ip_mut(&mut self, ip: SocketAddr) -> Option<&mut User> {
        self.users.iter_mut().find(|u| u.ip == ip)
    }
}

#[derive(Debug)]
pub struct User {
    pub ip: SocketAddr,
    pub stream: TcpStream,
    pub username: String,
    pub token: String,
    pub time: u128,
    pub send_queue: Vec<Packet>,
    pub receive_queue: Vec<Packet>,
    pub drop: bool,
}

impl User {
    pub fn new(ip: SocketAddr, stream: TcpStream, username: String, token: String) -> Self {
        User {
            ip,
            stream,
            username,
            token,
            time: get_timestamp(),
            send_queue: vec![],
            receive_queue: vec![],
            drop: false,
        }
    }

    pub fn queue_packet(&mut self, message: Message) {
        let packet = Packet {
            session_id: self.token.clone(),
            destination: self.ip,
            message,
        };

        self.send_queue.push(packet);
    }
}
