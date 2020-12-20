use std::{
    sync::Arc,
    net::SocketAddr,
    thread::{spawn, park},
};
use parking_lot::RwLock;
use smol::net::UdpSocket;

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
pub const SERVER_IP: &str = "0.0.0.0:8888";

fn main() {
    let server = Arc::new(RwLock::new(ServerData::default()));
    let server_ip: SocketAddr = SERVER_IP.parse().unwrap();
    let socket = smol::block_on(async {UdpSocket::bind(server_ip).await.unwrap()});

    let server_one = server.clone();
    let socket_one = smol::block_on(async {socket.clone()});
    spawn(move || server_loop(server_one, socket_one));

    let server_two = server.clone();
    let socket_two = smol::block_on(async {socket.clone()});
    spawn(move || receive_data(server_two, socket_two));

    park()
}
