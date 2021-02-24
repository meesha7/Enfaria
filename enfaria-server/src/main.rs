use crate::prelude::*;
use async_std::{net::TcpListener, task};
use parking_lot::RwLock;
use sqlx::mysql::MySqlPoolOptions;
use std::{env, net::SocketAddr, sync::Arc, thread::spawn};

pub mod connect;
pub mod data;
pub mod prelude;
pub mod tick;

// In milliseconds, 125 = 60 TPS
pub const TICKRATE: u64 = 125;
pub const SERVER_IP: &str = "127.0.0.1:8888";

fn main() {
    dotenv::dotenv().expect("Failed to setup dotenv.");
    env_logger::init();

    let data = Arc::new(RwLock::new(Server::new()));
    let server_ip: SocketAddr = SERVER_IP.parse().expect("Invalid server IP provided.");

    let listener = task::block_on(async { TcpListener::bind(server_ip).await.expect("Failed to bind listener.") });

    let pool = Arc::new(task::block_on(async {
        MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DATABASE").expect("DATABASE environment variable not found."))
            .await
            .expect("Failed to connect to database.")
    }));

    let data_c = data.clone();
    let pool_c = pool.clone();
    spawn(move || tick::tick(data_c, pool_c));

    connect::accept_connections(data, listener, pool)
}
