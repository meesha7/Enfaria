use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use gdnative::prelude::*;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, NativeClass)]
#[inherit(Node)]
pub struct Packet {
    pub beat: u64,
    pub destination: SocketAddr,
    pub command: Command,
}

#[methods]
impl Packet {
    fn new(_owner: &Node) -> Self {
        Packet { beat: 0, destination: "0.0.0.0:8888".parse().unwrap(), command: Command::Quit }
    }

    #[export]
    fn set_beat(&mut self, _owner: &Node, beat: u64) {
        self.beat = beat;
    }

    #[export]
    fn set_destination(&mut self, _owner: &Node, destination: String) {
        self.destination = destination.parse().unwrap()
    }

    #[export]
    fn set_command(&mut self, _owner: &Node, command: String) {
        self.command = match &command[..] {
            "connect" => Command::Connect,
            "quit" => Command::Quit,
            _ => unreachable!()
        }
    }

    #[export]
    fn set_all(&mut self, owner: &Node, beat: u64, destination: String, command: String) {
        self.set_beat(owner, beat);
        self.set_destination(owner, destination);
        self.set_command(owner, command);
    }

    #[export]
    fn to_bytes(&mut self, _owner: &Node) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Connect,
    Quit,
}

fn init(handle: InitHandle) {
    handle.add_class::<Packet>();
}

godot_init!(init);
