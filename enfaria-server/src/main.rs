use std::{
    env,
    sync::Arc,
    net::SocketAddr,
    thread::{spawn, park},
};
use parking_lot::RwLock;
use async_std::{task, net::UdpSocket};
use sqlx::mysql::MySqlPoolOptions;

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
    dotenv::dotenv().ok();
    env_logger::init();

    let server = Arc::new(RwLock::new(ServerData::default()));
    let server_ip: SocketAddr = SERVER_IP.parse().unwrap();
    let socket = Arc::new(task::block_on(async {UdpSocket::bind(server_ip).await.unwrap()}));
    let pool = Arc::new(task::block_on( async {
        MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap()).await.unwrap()
    }));

    let server_one = server.clone();
    let socket_one = socket.clone();
    let pool_one = pool.clone();
    spawn(move || server_loop(server_one, socket_one, pool_one));

    let server_two = server.clone();
    let socket_two = socket.clone();
    let pool_two = pool.clone();
    spawn(move || receive_data(server_two, socket_two, pool_two));

    park()
}
