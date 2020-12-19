pub use crate::data::{ServerData, UserId, UserPosition};
pub use crate::{TICKRATE, PLAYER_ID};
pub use crate::server::{
    server_loop,
    send_data::send_data,
    handle_quits::handle_quits,
};
pub use crate::receive::receive_data;

macro_rules! urcontinue {
    ($e:expr) => {
        match $e {
            Ok(a) => a,
            Err(_) => continue,
        };
    };
}
