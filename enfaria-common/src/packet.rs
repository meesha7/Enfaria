use crate::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize, NativeClass)]
#[inherit(Node)]
pub struct Packet {
    pub beat: u64,
    pub session_id: String,
    pub destination: SocketAddr,
    pub command: Command,
}

#[methods]
impl Packet {
    fn new(_owner: &Node) -> Self {
        Packet {
            beat: 0,
            session_id: "".to_string(),
            destination: "0.0.0.0:8888".parse().unwrap(),
            command: Command::Quit,
        }
    }

    #[export]
    fn set_beat(&mut self, _owner: &Node, beat: u64) {
        self.beat = beat;
    }

    #[export]
    fn get_beat(&self, _owner: &Node) -> u64 {
        self.beat
    }

    #[export]
    fn set_session_id(&mut self, _owner: &Node, session_id: String) {
        self.session_id = session_id;
    }

    #[export]
    fn get_session_id(&self, _owner: &Node) -> String {
        self.session_id.clone()
    }

    #[export]
    fn set_destination(&mut self, _owner: &Node, destination: String) {
        self.destination = destination.parse().unwrap()
    }

    #[export]
    fn get_destination(&self, _owner: &Node) -> String {
        self.destination.to_string()
    }

    #[export]
    fn set_command(&mut self, _owner: &Node, command: String) {
        let mut com = match &command[..] {
            "connect" => Command::Connect,
            "ping" => Command::Ping,
            "quit" => Command::Quit,
            _ => Command::Quit,
        };
        if command.starts_with("move") {
            let position: Position = command.split(' ').collect::<Vec<&str>>()[1].into();
            com = Command::Move(position);
        }
        if command.starts_with("create_tile") {
            let split: Vec<&str> = command.split(' ').collect();
            let position: Position = split[1..4].join(" ").into();
            let tile: Tile = split[4].into();
            com = Command::CreateTile((position, tile))
        }
        self.command = com
    }

    #[export]
    fn get_command(&self, _owner: &Node) -> String {
        self.command.to_string()
    }

    #[export]
    fn set_all(&mut self, owner: &Node, beat: u64, session_id: String, destination: String, command: String) {
        self.set_beat(owner, beat);
        self.set_session_id(owner, session_id);
        self.set_destination(owner, destination);
        self.set_command(owner, command);
    }

    #[allow(clippy::wrong_self_convention)]
    #[export]
    fn to_bytes(&mut self, _owner: &Node) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    #[export]
    fn from_bytes(&mut self, owner: &Node, bytes: Vec<u8>) {
        let packet: Packet = bincode::deserialize(&bytes).unwrap();
        self.set_all(
            owner,
            packet.beat,
            packet.session_id,
            packet.destination.to_string(),
            packet.command.to_string(),
        )
    }
}
