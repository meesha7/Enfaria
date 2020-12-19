use crate::prelude::*;
use std::{
    collections::HashMap,
    net::{UdpSocket, SocketAddr},
    time::{Instant, Duration},
    thread::sleep,
};
use smol::Async;
use farmer_common::Packet;

pub async fn send_data(players: HashMap<SocketAddr, UserId>, send_queue: HashMap<UserId, Vec<Packet>>, server_ip: SocketAddr) {
    let socket = Async::<UdpSocket>::bind(server_ip).unwrap();
    loop {
        let now = Instant::now();

        // Send queued packages
        for (ip, uid) in players.iter() {
            if let Some(sq) = send_queue.get(&uid) {
                for packet in sq.into_iter() {
                    let spacket = bincode::serialize(packet).unwrap();
                    let _ = socket.send_to(&spacket, ip.clone()).await;
                }
            }
        };

        let delta = TICKRATE - now.elapsed().as_millis() as u64;
        sleep(Duration::from_millis(delta))
    }
}
