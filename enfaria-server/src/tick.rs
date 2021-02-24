use crate::prelude::*;
use async_std::prelude::*;
use async_std::task;
use futures::future::join_all;
use parking_lot::RwLock;
use sqlx::mysql::MySqlPool;
use std::{
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};

pub fn tick(server: Arc<RwLock<Server>>, _pool: Arc<MySqlPool>) {
    task::block_on(async move {
        loop {
            // Start measuring tick and lock the server.
            let now = Instant::now();
            let mut s = server.write();

            // Drop bad users
            s.users.retain(|user| !user.drop);

            // Receive packets
            let futs: Vec<_> = s.users.iter_mut().map(|user| receive_packets(user)).collect();
            join_all(futs).await;

            // PROCESSING START
            // .. DO STUFF ..
            // PROCESSING END

            // Send packets
            let futs: Vec<_> = s.users.iter_mut().map(|user| send_packets(user)).collect();
            join_all(futs).await;

            // Drop server lock before sleeping.
            drop(s);

            // Sleep if necessary.
            if let Some(delta) = TICKRATE.checked_sub(now.elapsed().as_millis() as u64) {
                sleep(Duration::from_millis(delta));
            };
        }
    });
}

pub async fn receive_packets(user: &mut User) {
    let stream = &mut user.stream;

    let mut buffer = vec![0u8; 1024];
    loop {
        if stream.read(&mut buffer).await.is_err() {
            info!("Dropping user named: {:?}", &user.username);
            user.drop = true;
            return;
        };
        let packet: Packet = match bincode::deserialize(&buffer) {
            Ok(p) => p,
            Err(e) => {
                info!("Failed to deserialize received packet: {:?}", e);
                return;
            }
        };

        info!("Received packet {:?}", &packet);
        user.receive_queue.push(packet);
        buffer = vec![0u8; 1024];
    }
}

pub async fn send_packets(user: &mut User) {
    let stream = &mut user.stream;

    for packet in user.send_queue.drain(..) {
        let serialized = match bincode::serialize(&packet) {
            Ok(s) => s,
            Err(e) => {
                info!("Failed to serialize packet: {:?}", e);
                continue;
            }
        };

        match stream.write(&serialized[..]).await {
            Ok(_) => {}
            Err(e) => {
                info!("Failed to write to client stream: {:?}", e);
                continue;
            }
        };
    }
}
