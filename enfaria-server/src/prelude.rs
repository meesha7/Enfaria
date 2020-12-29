pub use enfaria_common::{Command, Packet, Position};
pub use crate::data::{ServerData, UserId};
pub use crate::{TICKRATE, PLAYER_ID};
pub use crate::server::{
    server_loop,
    send_data::send_data,
    handle_quits::handle_quits,
};
pub use crate::receive::receive_data;
pub use log::info;

macro_rules! urcontinue {
    ($e:expr) => {
        match $e {
            Ok(a) => a,
            Err(e) => {
                info!("Unwrap Continue Error: {:?}", e);
                continue
            },
        };
    };
}


use std::net::SocketAddr;
use std::collections::hash_map::Entry;


pub fn send_packet(server: &mut ServerData, ip: SocketAddr, packet: Packet) {
    let id = match server.players.get(&ip) {
        Some(i) => *i,
        None => {
            info!("Player not found {:?}", &ip);
            return;
        },
    };
    let session_id = match server.tokens.get(&id) {
        Some(i) => i,
        None => {
            info!("Session ID not found {:?}", &ip);
            return;
        },
    };
    if session_id != &packet.session_id {
        info!("Invalid session ID {:?}", &ip);
        return;
    };
    match server.send_queue.entry(id) {
        Entry::Occupied(o) => { o.into_mut().push(packet); },
        Entry::Vacant(v) => { v.insert(vec![packet]); }
    }
}


pub fn receive_packet(server: &mut ServerData, ip: SocketAddr, packet: Packet) {
    let id = match server.players.get(&ip) {
        Some(i) => *i,
        None => {
            info!("Player not found {:?}", &ip);
            return;
        },
    };
    let session_id = match server.tokens.get(&id) {
        Some(i) => i,
        None => {
            info!("Session ID not found {:?}", &ip);
            return;
        },
    };
    if session_id != &packet.session_id {
        info!("Invalid session ID {:?}", &ip);
        return;
    };
    match server.receive_queue.entry(id) {
        Entry::Occupied(o) => { o.into_mut().push(packet); },
        Entry::Vacant(v) => { v.insert(vec![packet]); }
    }
}
