//! Messages are behavior sent between clients and the authoritative server.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    // Client sends these to server
    Connect,
    Ping,
    Quit,
    // Server sends these to client
    Sync(Ecs),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ecs {
    CreateEntity,
    RemoveEntity,
    UpdateEntity,
}
