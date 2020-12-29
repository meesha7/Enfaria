use crate::prelude::*;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
};
use async_std::net::UdpSocket;
use log::info;

pub async fn send_data(players: HashMap<SocketAddr, UserId>, send_queue: HashMap<UserId, Vec<Packet>>, socket: Arc<UdpSocket>) {
    // Send queued packages
    for (ip, uid) in players.iter() {
        if let Some(sq) = send_queue.get(&uid) {
            for packet in sq.into_iter() {
                info!("Sending: {:?}", &packet);
                let spacket = bincode::serialize(packet).unwrap();
                let _ = socket.send_to(&spacket, ip.clone()).await;
            }
        }
    };
}
