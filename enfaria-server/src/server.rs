use std::{
    collections::HashMap,
    sync::Arc,
    net::SocketAddr,
    thread::sleep,
    time::{Instant, Duration},
};
use crate::prelude::*;
use parking_lot::RwLock;

pub mod send_data;
pub mod handle_quits;

pub async fn server_loop(server: Arc<RwLock<ServerData>>, server_ip: SocketAddr) {
    loop {
        let now = Instant::now();

        let players;
        let send_queue;

        {
            let mut s = server.write();

            // processing goes here
            handle_quits(&mut s);

            players = s.players.clone();
            send_queue = s.send_queue.clone();
            s.send_queue = HashMap::new();
            s.beat += 1;
        }

        send_data(players, send_queue, server_ip).await;


        let delta = TICKRATE - now.elapsed().as_millis() as u64;
        sleep(Duration::from_millis(delta));
    }
}
