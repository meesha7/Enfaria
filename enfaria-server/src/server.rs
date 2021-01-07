use crate::prelude::*;
use async_std::{net::UdpSocket, task};
use parking_lot::RwLock;
use sqlx::mysql::MySqlPool;
use std::{
    collections::HashMap,
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};

pub mod handle_quits;
pub mod ping_players;
pub mod send_data;

pub fn server_loop(server: Arc<RwLock<ServerData>>, socket: Arc<UdpSocket>, _pool: Arc<MySqlPool>) {
    task::block_on(async move {
        loop {
            let now = Instant::now();

            let players;
            let send_queue;

            {
                let mut s = server.write();

                // processing goes here

                if s.beat % 40 == 0 {
                    ping_players(&mut s);
                }
                handle_quits(&mut s);

                players = s.players.clone();
                send_queue = s.send_queue.clone();
                s.send_queue = HashMap::new();
                s.beat += 1;
            }

            send_data(players, send_queue, socket.clone()).await;

            let delta = TICKRATE - now.elapsed().as_millis() as u64;
            sleep(Duration::from_millis(delta));
        }
    });
}
