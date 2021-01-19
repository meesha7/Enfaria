use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromVariant, ToVariant)]
pub enum Command {
    Connect,
    Ping,
    Quit,
    Move(Position),
    CreateTile((Position, Tile)),
    CreatePlayer((Position, String)),
    CreateItem((u16, Object)),
    MoveItem((u16, u16)),
    ChatSend(String),
    ChatReceive(String),
    CreateObject((Position, Object)),
}

impl Command {
    pub fn is_move(&self) -> bool {
        matches!(self, Command::Move(_))
    }

    pub fn is_move_item(&self) -> bool {
        matches!(self, Command::MoveItem(_))
    }

    pub fn is_chat_send(&self) -> bool {
        matches!(self, Command::ChatSend(_))
    }
}
