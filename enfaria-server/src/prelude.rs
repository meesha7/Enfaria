pub use crate::data::{Server, User};
pub use crate::TICKRATE;
pub use enfaria_common::*;
pub use log::info;
pub use std::net::SocketAddr;

pub fn get_timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Time went backwards.")
        .as_millis()
}
