use crate::*;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, Clone, Serialize, Deserialize, NativeClass)]
#[inherit(Reference)]
pub struct Packet {
    #[property]
    pub beat: u64,
    #[property]
    pub session_id: String,
    pub destination: SocketAddr,
    pub command: Command,
}

#[methods]
impl Packet {
    pub fn new(_owner: &Reference) -> Self {
        Packet {
            beat: 0,
            session_id: "".to_string(),
            destination: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8888),
            command: Command::Quit,
        }
    }

    #[export]
    pub fn set_destination(&mut self, _owner: &Reference, destination: String) {
        self.destination = gresult!(destination.parse())
    }

    #[export]
    pub fn get_destination(&self, _owner: &Reference) -> String {
        self.destination.to_string()
    }

    #[export]
    pub fn set_command(&mut self, _owner: &Reference, command: Command) {
        self.command = command
    }

    #[export]
    pub fn get_command(&self, _owner: &Reference) -> Command {
        self.command.clone()
    }

    #[allow(clippy::wrong_self_convention)]
    #[export]
    pub fn to_bytes(&self, _owner: &Reference) -> Vec<u8> {
        gresult!(bincode::serialize(self))
    }

    #[export]
    pub fn from_bytes(&mut self, owner: &Reference, bytes: Vec<u8>) {
        let packet: Packet = gresult!(bincode::deserialize(&bytes));
        self.beat = packet.beat;
        self.session_id = packet.session_id;
        self.set_destination(owner, packet.destination.to_string());
        self.set_command(owner, packet.command);
    }
}
