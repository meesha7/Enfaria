use crate::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Connect,
    Ping,
    Quit,
    Move(Position),
    CreateTile((Position, Tile)),
    CreatePlayer((Position, String)),
}

impl Command {
    pub fn is_move(&self) -> bool {
        match self {
            Command::Move(_) => true,
            _ => false,
        }
    }
}

impl Display for Command {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Connect => write!(fmt, "connect"),
            Command::Ping => write!(fmt, "ping"),
            Command::Quit => write!(fmt, "quit"),
            Command::Move(pos) => write!(fmt, "move {} {} {}", pos.x, pos.y, pos.z),
            Command::CreateTile((pos, tile)) => write!(fmt, "create_tile {} {} {} {}", pos.x, pos.y, pos.z, tile),
            Command::CreatePlayer((pos, name)) => write!(fmt, "create_player {} {} {} {}", pos.x, pos.y, pos.z, name),
        }
    }
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        match s {
            x if x.starts_with("connect") => Command::Connect,
            x if x.starts_with("ping") => Command::Ping,
            x if x.starts_with("quit") => Command::Quit,
            x if x.starts_with("move") => {
                let split: Vec<&str> = x.split(' ').collect();
                let x = split.get(1).unwrap().parse().unwrap();
                let y = split.get(2).unwrap().parse().unwrap();
                let z = split.get(3).unwrap().parse().unwrap();
                Command::Move(Position { x, y, z })
            }
            x if x.starts_with("create_tile") => {
                let split: Vec<&str> = x.split(' ').collect();
                let x = split.get(1).unwrap().parse().unwrap();
                let y = split.get(2).unwrap().parse().unwrap();
                let z = split.get(3).unwrap().parse().unwrap();
                let position = Position { x, y, z };
                let tile = (*split.get(4).unwrap()).into();
                Command::CreateTile((position, tile))
            }
            x if x.starts_with("create_player") => {
                let split: Vec<&str> = x.splitn(5, ' ').collect();
                let x = split.get(1).unwrap().parse().unwrap();
                let y = split.get(2).unwrap().parse().unwrap();
                let z = split.get(3).unwrap().parse().unwrap();
                let username = split.get(4).unwrap().to_string();
                Command::CreatePlayer((Position { x, y, z }, username))
            }
            _ => unreachable!(),
        }
    }
}
