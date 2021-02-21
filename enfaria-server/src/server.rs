use crate::prelude::*;
use async_std::io::prelude::*;
use async_std::task;
use parking_lot::RwLock;
use sqlx::mysql::MySqlPool;
use std::{
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};

pub fn tick(server: Arc<RwLock<ServerData>>, _pool: Arc<MySqlPool>) {
    task::block_on(async move {
        loop {
            let now = Instant::now();
            let mut s = server.write();

            // Receive packets
            for user in s.users.iter_mut() {
                receive_packets(user).await;
            }

            // PROCESSING START
            if !s.users.is_empty() {
                info!("Current users: {:?}", &s.users);
            }
            // PROCESSING END

            // Send packets
            for user in s.users.iter_mut() {
                send_packets(user).await;
            }

            drop(s);
            let delta = TICKRATE - now.elapsed().as_millis() as u64;
            sleep(Duration::from_millis(delta));
        }
    });
}

pub async fn receive_packets(user: &mut User) {
    let stream = &mut user.stream;
    let mut buffer = vec![0u8; 1024];

    while let Ok(v) = stream.read(&mut buffer).await {
        if v == 0 {
            return;
        };

        buffer = vec![0u8; 1024];
    }
}

pub async fn send_packets(user: &mut User) {
    let stream = &mut user.stream;

    for packet in user.send_queue.drain(..) {
        let serialized = urcontinue!(bincode::serialize(&packet));
        urcontinue!(stream.write(&serialized[..]).await);
    }
}
