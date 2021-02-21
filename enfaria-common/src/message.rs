use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    Connect,
    Ping,
    Quit,
    Sync(Ecs),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ecs {
    CreateEntity,
    RemoveEntity,
}
