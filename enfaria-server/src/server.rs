use std::{
    collections::HashMap,
    sync::Arc,
    thread::sleep,
    time::{Instant, Duration},
};
use crate::prelude::*;
use parking_lot::RwLock;
use smol::net::UdpSocket;

pub mod send_data;
pub mod handle_quits;

pub fn server_loop(server: Arc<RwLock<ServerData>>, socket: UdpSocket) {
    smol::block_on(async move {
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

            send_data(players, send_queue, socket.clone()).await;

            let delta = TICKRATE - now.elapsed().as_millis() as u64;
            sleep(Duration::from_millis(delta));
        }
    });
}
