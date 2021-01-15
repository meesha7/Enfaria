use crate::prelude::*;
use async_std::{net::UdpSocket, task};
use parking_lot::RwLock;
use sqlx::mysql::MySqlPool;
use std::{
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};

pub mod handle_quits;
pub mod move_items;
pub mod move_players;
pub mod ping_players;
pub mod send_data;
pub mod sort_data;

pub fn server_loop(server: Arc<RwLock<ServerData>>, socket: Arc<UdpSocket>, _pool: Arc<MySqlPool>) {
    task::block_on(async move {
        loop {
            let now = Instant::now();

            let users;

            {
                let mut s = server.write();
                sort_data::sort_data(&mut s);

                // processing goes here

                move_players::move_players(&mut s);
                move_items::move_items(&mut s);

                // end of processing

                if s.beat % 40 == 0 {
                    ping_players::ping_players(&mut s);
                }
                handle_quits::handle_quits(&mut s);

                users = s.users.clone();
                s.beat += 1;
                for mut user in s.users.iter_mut() {
                    user.send_queue = vec![]
                }
            }

            send_data::send_data(users, socket.clone()).await;

            let delta = TICKRATE - now.elapsed().as_millis() as u64;
            sleep(Duration::from_millis(delta));
        }
    });
}
