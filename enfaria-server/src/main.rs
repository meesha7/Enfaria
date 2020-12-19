use std::{
    sync::Arc,
    net::SocketAddr,
    thread::{park, spawn},
};
use parking_lot::RwLock;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref PLAYER_ID: RwLock<u64> = {
        RwLock::new(0)
    };
}


#[macro_use]
mod prelude;
use prelude::*;
mod data;
mod receive;
mod server;

// In milliseconds, 125 = 60 TPS
pub const TICKRATE: u64 = 125;
pub const SERVER_IP: &str = "0.0.0.0:55555";

fn main() {
    let server = Arc::new(RwLock::new(ServerData::default()));
    let server_ip: SocketAddr = SERVER_IP.parse().unwrap();

    let server_one = server.clone();
    spawn(move || server_loop(server_one, server_ip));

    let server_two = server.clone();
    spawn(move || receive_data(server_two, server_ip));

    drop(server);

    park()
}
