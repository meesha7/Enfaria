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
}

impl Display for Command {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Connect => write!(fmt, "connect"),
            Command::Ping => write!(fmt, "ping"),
            Command::Quit => write!(fmt, "quit"),
            Command::Move(pos) => write!(fmt, "move {} {} {}", pos.x, pos.y, pos.z),
            Command::CreateTile((pos, tile)) => {
                write!(fmt, "create_tile {} {} {} {}", pos.x, pos.y, pos.z, tile)
            }
        }
    }
}
