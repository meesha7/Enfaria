use crate::*;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, Clone, Serialize, Deserialize, NativeClass)]
#[inherit(Reference)]
pub struct Packet {
    pub beat: u64,
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
    pub fn set_beat(&mut self, _owner: &Reference, beat: u64) {
        self.beat = beat;
    }

    #[export]
    pub fn get_beat(&self, _owner: &Reference) -> u64 {
        self.beat
    }

    #[export]
    pub fn set_session_id(&mut self, _owner: &Reference, session_id: String) {
        self.session_id = session_id;
    }

    #[export]
    pub fn get_session_id(&self, _owner: &Reference) -> String {
        self.session_id.clone()
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
    pub fn set_command(&mut self, _owner: &Reference, command: String) {
        self.command = command.into()
    }

    #[export]
    pub fn get_command(&self, _owner: &Reference) -> String {
        self.command.to_string()
    }

    #[export]
    pub fn set_all(&mut self, owner: &Reference, beat: u64, session_id: String, destination: String, command: String) {
        self.set_beat(owner, beat);
        self.set_session_id(owner, session_id);
        self.set_destination(owner, destination);
        self.set_command(owner, command);
    }

    #[allow(clippy::wrong_self_convention)]
    #[export]
    pub fn to_bytes(&self, _owner: &Reference) -> Vec<u8> {
        gresult!(bincode::serialize(self))
    }

    #[export]
    pub fn from_bytes(&mut self, owner: &Reference, bytes: Vec<u8>) {
        let packet: Packet = gresult!(bincode::deserialize(&bytes));
        self.set_all(
            owner,
            packet.beat,
            packet.session_id,
            packet.destination.to_string(),
            packet.command.to_string(),
        )
    }
}
