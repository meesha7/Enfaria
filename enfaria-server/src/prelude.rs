pub use crate::data::{ServerData, User, UserId};
pub use crate::{PLAYER_ID, TICKRATE};
pub use enfaria_common::*;
pub use log::info;
pub use std::net::SocketAddr;

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

pub fn send_packet(server: &mut ServerData, ip: SocketAddr, packet: Packet) {
    let user = match server.user_by_ip_mut(ip) {
        Some(u) => u,
        None => {
            info!("User not found! {:?}", &ip);
            return;
        }
    };

    if user.token != packet.session_id {
        info!("Invalid session ID {:?}", &ip);
        return;
    };

    user.send_queue.push(packet);
}

pub fn receive_packet(server: &mut ServerData, ip: SocketAddr, packet: Packet) {
    let user = match server.user_by_ip_mut(ip) {
        Some(u) => u,
        None => {
            info!("User not found! {:?}", &ip);
            return;
        }
    };

    if user.token != packet.session_id {
        info!("Invalid session ID {:?}", &ip);
        return;
    };

    user.receive_queue.push(packet);
}

pub fn get_timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Time went backwards.")
        .as_millis()
}
