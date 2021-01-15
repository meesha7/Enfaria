use crate::prelude::*;
use async_std::net::UdpSocket;
use std::sync::Arc;

pub async fn send_data(users: Vec<User>, socket: Arc<UdpSocket>) {
    // Send queued packages
    for user in users.into_iter() {
        for packet in user.send_queue.into_iter() {
            info!("Sending: {:?}", &packet);
            let spacket = urcontinue!(bincode::serialize(&packet));
            let _ = socket.send_to(&spacket, user.ip).await;
        }
    }
}
