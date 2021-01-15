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
    CreateItem((u16, Item)),
    MoveItem((u16, u16)),
}

impl Command {
    pub fn is_move(&self) -> bool {
        matches!(self, Command::Move(_))
    }

    pub fn is_move_item(&self) -> bool {
        matches!(self, Command::MoveItem(_))
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
            Command::CreateItem((pos, item)) => write!(fmt, "create_item {} {}", pos, item),
            Command::MoveItem((from, to)) => write!(fmt, "move_item {} {}", from, to),
        }
    }
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        match s {
            x if x.starts_with("connect") => Command::Connect,
            x if x.starts_with("ping") => Command::Ping,
            x if x.starts_with("quit") => Command::Quit,
            x if x.starts_with("move ") => {
                let split: Vec<&str> = x.split(' ').collect();
                let x = gresult!(goption!(split.get(1)).parse());
                let y = gresult!(goption!(split.get(2)).parse());
                let z = gresult!(goption!(split.get(3)).parse());
                Command::Move(Position { x, y, z })
            }
            x if x.starts_with("create_tile") => {
                let split: Vec<&str> = x.splitn(5, ' ').collect();
                let x = gresult!(goption!(split.get(1)).parse());
                let y = gresult!(goption!(split.get(2)).parse());
                let z = gresult!(goption!(split.get(3)).parse());
                let position = Position { x, y, z };
                let tile = (*goption!(split.get(4))).into();
                Command::CreateTile((position, tile))
            }
            x if x.starts_with("create_player") => {
                let split: Vec<&str> = x.splitn(5, ' ').collect();
                let x = gresult!(goption!(split.get(1)).parse());
                let y = gresult!(goption!(split.get(2)).parse());
                let z = gresult!(goption!(split.get(3)).parse());
                let username = goption!(split.get(4)).to_string();
                Command::CreatePlayer((Position { x, y, z }, username))
            }
            x if x.starts_with("create_item") => {
                let split: Vec<&str> = x.splitn(3, ' ').collect();
                let pos = gresult!(goption!(split.get(1)).parse());
                let item = (*goption!(split.get(2))).into();
                Command::CreateItem((pos, item))
            }
            x if x.starts_with("move_item") => {
                let split: Vec<&str> = x.split(' ').collect();
                let from = gresult!(goption!(split.get(1)).parse());
                let to = gresult!(goption!(split.get(2)).parse());
                Command::MoveItem((from, to))
            }
            _ => {
                godot_error!("Invalid command: {:?}", s);
                unreachable!();
            }
        }
    }
}
