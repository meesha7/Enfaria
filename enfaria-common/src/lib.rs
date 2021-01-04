use std::{net::SocketAddr, fmt::{Display, Formatter}};
use serde::{Serialize, Deserialize};
use gdnative::prelude::*;
use crate::map::Tile;

pub mod map;
pub mod player;
pub mod item;

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
        Packet { beat: 0, session_id: "".to_string(), destination: "0.0.0.0:8888".parse().unwrap(), command: Command::Quit }
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
            _ => Command::Quit
        };
        if command.starts_with("move") {
            let position: Position = command.split(" ").collect::<Vec<&str>>()[1].into();
            com = Command::Move(position);
        }
        if command.starts_with("create_tile") {
            let split: Vec<&str> = command.split(" ").collect();
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

    #[export]
    fn to_bytes(&mut self, _owner: &Node) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    #[export]
    fn from_bytes(&mut self, owner: &Node, bytes: Vec<u8>) {
        let packet: Packet = bincode::deserialize(&bytes).unwrap();
        self.set_all(owner, packet.beat, packet.session_id, packet.destination.to_string(), packet.command.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Connect,
    Ping,
    Quit,
    Move(Position),
    CreateTile((Position, Tile)),
}

impl Display for Command {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Connect => write!(fmt, "connect"),
            Command::Ping => write!(fmt, "ping"),
            Command::Quit => write!(fmt, "quit"),
            Command::Move(pos) => write!(fmt, "move {} {} {}", pos.x, pos.y, pos.z),
            Command::CreateTile((pos, tile)) => write!(fmt, "create_tile {} {} {} {}", pos.x, pos.y, pos.z, tile),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl From<String> for Position {
    fn from(string: String) -> Self {
        string.into()
    }
}

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        let split: Vec<u64> = s.split(" ").map(|x| x.parse().unwrap()).collect();
        Position {
            x: split[0],
            y: split[1],
            z: split[2],
        }
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Packet>();
}

godot_init!(init);
