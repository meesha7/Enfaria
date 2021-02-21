pub use crate::data::{ServerData, User, UserId};
pub use crate::{TICKRATE, USER_ID};
pub use enfaria_common::*;
pub use log::info;
pub use std::net::SocketAddr;

#[macro_export]
macro_rules! urcontinue {
    ($e:expr) => {
        match $e {
            Ok(a) => a,
            Err(e) => {
                info!("Unwrap Continue Error: {:?}", e);
                continue;
            }
        };
    };
}

#[macro_export]
macro_rules! urreturn {
    ($e:expr) => {
        match $e {
            Ok(a) => a,
            Err(e) => {
                info!("Unwrap Return Error: {:?}", e);
                return;
            }
        };
    };
}

pub fn get_timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Time went backwards.")
        .as_millis()
}
